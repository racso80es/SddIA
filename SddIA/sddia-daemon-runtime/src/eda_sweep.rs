use crate::eda_bus::{ensure_event_bus_topology, header_path, load_registry, EdBusPaths};
use crate::{ensure_fractal_dirs, load_bus_topology, BusTopology};
use sddia_core::ports::event_manager::{
    load_dispatched_event, DeliveryStatus, ListDeliveriesRequest, ListEventsRequest,
    ListPendingRequest, MoveToDlqRequest, PurgeEventRequest,
};
use sddia_event_manager_host::with_productive_port;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

pub const POLL_SECONDS: u64 = 5;

#[derive(Debug, Clone, Default)]
pub struct SweepReport {
    pub purged: Vec<Value>,
    pub dead_lettered: Vec<Value>,
    pub kaizen_alerts: Vec<String>,
    pub kaizen_finalized: Vec<Value>,
    pub skipped: Vec<Value>,
}

#[cfg(test)]
fn delivery_stamp_terminal_ok(status: &str) -> bool {
    status == "success" || status == "skipped" || status.starts_with("skipped")
}

#[cfg(test)]
fn delivery_stamp_terminal(status: &str) -> bool {
    delivery_stamp_terminal_ok(status) || status == "failed"
}

#[cfg(test)]
fn fractal_delivery_state<'a>(body: &'a Value) -> Option<&'a serde_json::Map<String, Value>> {
    body.get("delivery_state").and_then(|v| v.as_object())
}

/// Todos los stamps terminales OK (success|skipped*) — sin `failed`.
#[cfg(test)]
fn fractal_event_all_ok(body: &Value) -> bool {
    let Some(ds) = fractal_delivery_state(body) else {
        return false;
    };
    if ds.is_empty() {
        return false;
    }
    if ds.values().any(|v| v.as_str() == Some("failed")) {
        return false;
    }
    ds.values().all(|v| {
        v.as_str()
            .map(delivery_stamp_terminal_ok)
            .unwrap_or(false)
    })
}

/// Todos los stamps terminales y al menos un `failed` → DLQ (laudo C2).
#[cfg(test)]
fn fractal_event_terminal_with_failure(body: &Value) -> bool {
    let Some(ds) = fractal_delivery_state(body) else {
        return false;
    };
    if ds.is_empty() {
        return false;
    }
    let all_terminal = ds.values().all(|v| {
        v.as_str()
            .map(delivery_stamp_terminal)
            .unwrap_or(false)
    });
    all_terminal && ds.values().any(|v| v.as_str() == Some("failed"))
}

#[cfg(test)]
fn move_fractal_to_dead_letter(path: &Path, dead_letter_dir: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(dead_letter_dir)
        .map_err(|e| format!("mkdir dead_letter {}: {e}", dead_letter_dir.display()))?;
    let file_name = path
        .file_name()
        .ok_or_else(|| format!("dead_letter move: sin nombre {}", path.display()))?;
    let dest = dead_letter_dir.join(file_name);
    if dest.exists() {
        let _ = fs::remove_file(&dest);
    }
    fs::rename(path, &dest).map_err(|e| {
        format!(
            "dead_letter rename {} → {}: {e}",
            path.display(),
            dest.display()
        )
    })?;
    Ok(dest)
}

fn sweep_fractal_family(repo: &Path, family: &str, report: &mut SweepReport) -> Result<(), String> {
    with_productive_port(repo, "sweep_fractal_family", |port| {
        let listed = port
            .list_events(ListEventsRequest {
                family: Some(family.to_string()),
                event_type: None,
                state: None,
                limit: Some(5_000),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
        for row in listed.events {
            let event_id = row.event_id;
            let deliveries = port
                .list_deliveries(ListDeliveriesRequest {
                    event_id: Some(event_id.clone()),
                    subscriber_id: None,
                    status: None,
                    limit: Some(1_000),
                    cursor: None,
                })
                .map_err(|e| e.to_string())?;
            if deliveries.deliveries.is_empty() {
                continue;
            }
            let all_ack = deliveries
                .deliveries
                .iter()
                .all(|d| d.status == DeliveryStatus::Ack);
            let all_terminal = deliveries.deliveries.iter().all(|d| {
                matches!(d.status, DeliveryStatus::Ack | DeliveryStatus::Nack)
            });
            let any_nack = deliveries
                .deliveries
                .iter()
                .any(|d| d.status == DeliveryStatus::Nack);
            if all_ack {
                let _ = port.purge_event(PurgeEventRequest {
                    event_id: event_id.clone(),
                });
                report.purged.push(json!({
                    "family": family,
                    "event_id": event_id,
                }));
            } else if any_nack && all_terminal {
                let _ = port.move_to_dlq(MoveToDlqRequest {
                    event_id: event_id.clone(),
                    reason: "fractal-subscriber-failure".into(),
                });
                report.dead_lettered.push(json!({
                    "family": family,
                    "event_id": event_id,
                }));
            }
        }
        Ok(())
    })
}

fn sweep_fractal_bus(repo: &Path, report: &mut SweepReport) -> Result<(), String> {
    let top: BusTopology = load_bus_topology(repo);
    let _ = ensure_fractal_dirs(&top);
    sweep_fractal_family(repo, "domain", report)?;
    sweep_fractal_family(repo, "orchestration", report)?;
    sweep_fractal_family(repo, "telemetry", report)?;
    sweep_progress_leaf(repo, &top, report)?;
    Ok(())
}

/// MAN-43 O5: el progreso vive en el puerto; no se indexan directorios `.events/progress/`.
fn sweep_progress_leaf(_repo: &Path, _top: &BusTopology, _report: &mut SweepReport) -> Result<(), String> {
    Ok(())
}

pub fn sweep_once(repo: &Path) -> Result<SweepReport, String> {
    let bus = ensure_event_bus_topology(repo)?;
    let registry = load_registry(&bus).ok();
    let mut report = SweepReport::default();

    let pending_ids: Vec<String> = with_productive_port(repo, "sweep_list_pending", |port| {
        port.list_pending(ListPendingRequest {
            limit: Some(10_000),
        })
        .map(|r| r.events.into_iter().map(|e| e.event_id).collect())
        .map_err(|e| e.to_string())
    })?;

    for event_uuid in pending_ids {
        {
            let result = try_sweep_event(repo, &bus, &event_uuid, registry.as_ref())?;
            match result.get("status").and_then(|v| v.as_str()) {
                Some("purged") => report.purged.push(json!({
                    "event_uuid": event_uuid,
                    "witnesses": result.get("witnesses").unwrap_or(&json!(0)),
                    "headers": result.get("headers").unwrap_or(&json!(0)),
                    "pending": result.get("pending").unwrap_or(&json!(0)),
                })),
                Some("kaizen-finalized") => report.kaizen_finalized.push(json!({
                    "event_uuid": event_uuid,
                    "pending": result.get("pending").unwrap_or(&json!(0)),
                    "headers": result.get("headers").unwrap_or(&json!(0)),
                })),
                Some("kaizen") => {
                    let dead = list_witnesses(repo, &bus, "dead_letter_subscribers", &event_uuid);
                    emit_kaizen_alert(
                        &event_uuid,
                        result.get("event_type").and_then(|v| v.as_str()).unwrap_or(""),
                        &dead,
                    );
                    report.kaizen_alerts.push(event_uuid);
                }
                Some(status)
                    if matches!(
                        status,
                        "invalid-json"
                            | "missing-event_type"
                            | "no-subscribers"
                            | "absent"
                            | "invalid-registry"
                    ) =>
                {
                    report.skipped.push(json!({
                        "event_uuid": event_uuid,
                        "reason": status,
                    }));
                }
                Some("in-flight") => report.skipped.push(json!({
                    "event_uuid": event_uuid,
                    "reason": "subscribers-in-flight",
                    "in_flight": result.get("in_flight").cloned().unwrap_or(json!([])),
                })),
                Some("awaiting") => report.skipped.push(json!({
                    "event_uuid": event_uuid,
                    "reason": "awaiting-subscribers",
                    "pending": result.get("pending_subscribers").cloned().unwrap_or(json!([])),
                })),
                _ => {}
            }
        }
    }

    sweep_fractal_bus(repo, &mut report)?;
    Ok(report)
}

pub fn try_sweep_event(
    repo: &Path,
    bus: &EdBusPaths,
    event_uuid: &str,
    registry: Option<&Value>,
) -> Result<Value, String> {
    let mut base = json!({"event_uuid": event_uuid, "purged": false});
    let pending_path = bus.pending.join(format!("{event_uuid}.json"));
    let event: Value = match with_productive_port(repo, "sweep_load_dispatched", |port| {
        load_dispatched_event(port, event_uuid, &pending_path).map_err(|e| e.to_string())
    }) {
        Ok(Some(event)) => event,
        Ok(None) => {
            base["status"] = json!("absent");
            return Ok(base);
        }
        Err(_) => {
            base["status"] = json!("invalid-json");
            return Ok(base);
        }
    };

    let event_type = event.get("event_type").and_then(|v| v.as_str());
    let Some(event_type) = event_type.filter(|s| !s.is_empty()) else {
        base["status"] = json!("missing-event_type");
        return Ok(base);
    };

    let payload = event
        .get("payload")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    let payload_val = Value::Object(payload.clone());

    let registry_val = match registry {
        Some(r) => r.clone(),
        None => match load_registry(bus) {
            Ok(r) => r,
            Err(_) => {
                base["status"] = json!("invalid-registry");
                base["event_type"] = json!(event_type);
                return Ok(base);
            }
        },
    };

    let applicable = applicable_subscriber_ids(&registry_val, event_type, &payload_val);
    let dead = list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid);

    if !dead.is_empty() {
        let origin = resolve_origin_topology(&payload_val);
        let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
        let terminals = terminal_subscriber_names(repo, bus, event_uuid);
        let applicable_set: HashSet<_> = applicable.iter().cloned().collect();
        if !applicable.is_empty()
            && applicable_set.is_subset(&terminals)
            && in_flight.intersection(&applicable_set).next().is_none()
        {
            let finalized = finalize_kaizen_terminal(
                repo,
                bus,
                event_uuid,
                &pending_path,
                &registry_val,
                event_type,
                &origin,
            )?;
            base["status"] = json!("kaizen-finalized");
            base["purged"] = json!(true);
            base["finalized"] = json!(true);
            base["event_type"] = json!(event_type);
            base["dead_letter_witnesses"] = json!(dead.iter().map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).collect::<Vec<_>>());
            if let Some(obj) = finalized.as_object() {
                for (k, v) in obj {
                    base[k] = v.clone();
                }
            }
            return Ok(base);
        }
        base["status"] = json!("kaizen");
        base["event_type"] = json!(event_type);
        base["dead_letter_witnesses"] = json!(dead.iter().map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).collect::<Vec<_>>());
        return Ok(base);
    }

    if applicable.is_empty() {
        let archived = archive_event_after_sweep(repo, bus, event_uuid, Some(event_type))?;
        base["status"] = json!("purged");
        base["purged"] = json!(true);
        base["event_type"] = json!(event_type);
        merge_counts(&mut base, &archived);
        return Ok(base);
    }

    let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
    let overlap: Vec<_> = in_flight
        .intersection(&applicable.iter().cloned().collect())
        .cloned()
        .collect();
    if !overlap.is_empty() {
        base["status"] = json!("in-flight");
        base["event_type"] = json!(event_type);
        base["in_flight"] = json!(overlap);
        return Ok(base);
    }

    let done = processed_subscriber_names(repo, bus, event_uuid);
    let applicable_set: HashSet<_> = applicable.iter().cloned().collect();
    if applicable_set.is_subset(&done) {
        let archived = archive_event_after_sweep(repo, bus, event_uuid, Some(event_type))?;
        base["status"] = json!("purged");
        base["purged"] = json!(true);
        base["event_type"] = json!(event_type);
        merge_counts(&mut base, &archived);
        return Ok(base);
    }

    let pending_subs: Vec<_> = applicable_set.difference(&done).cloned().collect();
    base["status"] = json!("awaiting");
    base["event_type"] = json!(event_type);
    base["pending_subscribers"] = json!(pending_subs);
    Ok(base)
}

fn merge_counts(base: &mut Value, archived: &Value) {
    for key in ["witnesses", "headers", "pending"] {
        if let Some(v) = archived.get(key) {
            base[key] = v.clone();
        }
    }
}

fn resolve_origin_topology(payload: &Value) -> String {
    payload
        .get("origin_topology")
        .and_then(|v| v.as_str())
        .filter(|s| matches!(*s, "core" | "local"))
        .unwrap_or("core")
        .to_string()
}

fn subscriber_applies(subscriber: &Value, origin_topology: &str) -> bool {
    let applies = subscriber
        .get("applies_to_origin_topology")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["core".into()]);
    if applies.is_empty() {
        return origin_topology == "core";
    }
    applies.iter().any(|s| s == origin_topology)
}

fn subscriber_id(subscriber: &Value) -> String {
    let agent = subscriber
        .get("agent")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if agent.is_empty() {
        return "unknown".into();
    }
    for key in ["process", "action", "tool"] {
        if let Some(val) = subscriber.get(key).and_then(|v| v.as_str()) {
            let val = val.trim();
            if !val.is_empty() {
                return format!("{agent}.{val}");
            }
        }
    }
    agent.to_string()
}

fn applicable_subscriber_ids(registry: &Value, event_type: &str, payload: &Value) -> Vec<String> {
    let origin = resolve_origin_topology(payload);
    registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .map(|subs| {
            subs.iter()
                .filter(|s| subscriber_applies(s, &origin))
                .map(subscriber_id)
                .collect()
        })
        .unwrap_or_default()
}

fn resolve_witness_key(state_key: &str) -> &str {
    match state_key {
        "subscriber_processing" | "processing" => "processing_subscribers",
        "subscriber_processed" | "processed" => "processed_subscribers",
        "subscriber_dead_letter" | "dead_letter" => "dead_letter_subscribers",
        other => other,
    }
}

pub fn list_witnesses(repo: &Path, _bus: &EdBusPaths, state_key: &str, event_uuid: &str) -> Vec<PathBuf> {
    let folder_key = resolve_witness_key(state_key);
    let Ok(rows) = with_productive_port(repo, "list_witnesses", |port| {
        port.list_deliveries(ListDeliveriesRequest {
            event_id: Some(event_uuid.to_string()),
            subscriber_id: None,
            status: None,
            cursor: None,
            limit: Some(10_000),
        })
        .map_err(|e| e.to_string())
    }) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    for row in rows.deliveries {
        if row.event_id != event_uuid {
            continue;
        }
        let Some(sub) = row
            .subscriber_id
            .as_deref()
            .filter(|s| !s.is_empty())
        else {
            continue;
        };
        let include = match (folder_key, row.status) {
            ("processing_subscribers", DeliveryStatus::Processing) => true,
            ("dead_letter_subscribers", DeliveryStatus::Nack) => true,
            ("processed_subscribers", DeliveryStatus::Ack) => true,
            _ => false,
        };
        if include {
            paths.push(PathBuf::from(format!("{event_uuid}.{sub}.json")));
        }
    }
    paths.sort();
    paths
}

fn witness_suffix(path: &Path, event_uuid: &str) -> Option<String> {
    let name = path.file_name()?.to_string_lossy();
    let prefix = format!("{event_uuid}.");
    if !name.starts_with(&prefix) || !name.ends_with(".json") {
        return None;
    }
    let mid = &name[prefix.len()..name.len() - 5];
    if mid.is_empty() {
        None
    } else {
        Some(mid.to_string())
    }
}

fn terminal_subscriber_names(repo: &Path, bus: &EdBusPaths, event_uuid: &str) -> HashSet<String> {
    let mut names = HashSet::new();
    for key in ["processed_subscribers", "dead_letter_subscribers"] {
        for path in list_witnesses(repo, bus, key, event_uuid) {
            if let Some(s) = witness_suffix(&path, event_uuid) {
                names.insert(s);
            }
        }
    }
    names
}

fn in_flight_subscriber_names(repo: &Path, bus: &EdBusPaths, event_uuid: &str) -> HashSet<String> {
    list_witnesses(repo, bus, "processing_subscribers", event_uuid)
        .into_iter()
        .filter_map(|p| witness_suffix(&p, event_uuid))
        .collect()
}

fn processed_subscriber_names(repo: &Path, bus: &EdBusPaths, event_uuid: &str) -> HashSet<String> {
    list_witnesses(repo, bus, "processed_subscribers", event_uuid)
        .into_iter()
        .filter_map(|p| witness_suffix(&p, event_uuid))
        .collect()
}

fn safe_remove_path(path: &Path) -> bool {
    if !path.is_file() {
        return true;
    }
    for attempt in 0..3 {
        match fs::remove_file(path) {
            Ok(()) => return true,
            Err(_) if attempt < 2 => thread::sleep(Duration::from_millis(50)),
            Err(_) => return !path.is_file(),
        }
    }
    !path.is_file()
}

fn ensure_state_header(bus: &EdBusPaths, state: &str, event_uuid: &str, source: &Path) -> Result<(), String> {
    let dest = header_path(bus, state, event_uuid);
    if dest.is_file() {
        return Ok(());
    }
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("mkdir header: {e}"))?;
    }
    fs::copy(source, &dest).map_err(|e| format!("copy header: {e}"))?;
    Ok(())
}

fn maybe_purge_processing_header(
    repo: &Path,
    bus: &EdBusPaths,
    event_uuid: &str,
    registry: &Value,
    event_type: &str,
    origin_topology: &str,
) -> bool {
    let required: Vec<String> = registry
        .get(event_type)
        .and_then(|v| v.as_array())
        .map(|subs| {
            subs.iter()
                .filter(|s| subscriber_applies(s, origin_topology))
                .map(subscriber_id)
                .collect()
        })
        .unwrap_or_default();
    if required.is_empty() {
        return false;
    }
    let terminals = terminal_subscriber_names(repo, bus, event_uuid);
    let in_flight = in_flight_subscriber_names(repo, bus, event_uuid);
    let req_set: HashSet<_> = required.iter().cloned().collect();
    if !req_set.is_subset(&terminals) {
        return false;
    }
    if !in_flight.intersection(&req_set).collect::<Vec<_>>().is_empty() {
        return false;
    }
    let header = header_path(bus, "processing", event_uuid);
    if header.is_file() {
        return safe_remove_path(&header);
    }
    false
}

fn finalize_kaizen_terminal(
    repo: &Path,
    bus: &EdBusPaths,
    event_uuid: &str,
    pending_path: &Path,
    registry: &Value,
    event_type: &str,
    origin_topology: &str,
) -> Result<Value, String> {
    let mut counts = json!({"pending": 0, "headers": 0});
    let dead_header = header_path(bus, "dead_letter", event_uuid);
    if !dead_header.is_file() && pending_path.is_file() {
        ensure_state_header(bus, "dead_letter", event_uuid, pending_path)?;
        counts["headers"] = json!(1);
    }
    if pending_path.is_file() && safe_remove_path(pending_path) {
        counts["pending"] = json!(1);
    }
    if maybe_purge_processing_header(repo, bus, event_uuid, registry, event_type, origin_topology) {
        counts["headers"] = json!(counts["headers"].as_i64().unwrap_or(0) + 1);
    }
    Ok(counts)
}

pub fn archive_event_after_sweep(
    repo: &Path,
    bus: &EdBusPaths,
    event_uuid: &str,
    event_type: Option<&str>,
) -> Result<Value, String> {
    let _ = repo;
    let mut counts = json!({"witnesses": 0, "headers": 0, "pending": 0});
    let mut resolved_type = event_type.map(|s| s.to_string());

    if resolved_type.is_none() {
        for state in ["processed", "processing"] {
            let header = header_path(bus, state, event_uuid);
            if !header.is_file() {
                continue;
            }
            if let Ok(raw) = fs::read_to_string(&header) {
                if let Ok(body) = serde_json::from_str::<Value>(&raw) {
                    if let Some(et) = body.get("event_type").and_then(|v| v.as_str()) {
                        resolved_type = Some(et.to_string());
                        break;
                    }
                }
            }
        }
    }

    let pending = bus.pending.join(format!("{event_uuid}.json"));
    if pending.is_file() && safe_remove_path(&pending) {
        counts["pending"] = json!(1);
    }
    for state in ["processing", "processed"] {
        let header = header_path(bus, state, event_uuid);
        if header.is_file() && safe_remove_path(&header) {
            counts["headers"] = json!(counts["headers"].as_i64().unwrap_or(0) + 1);
        }
    }
    for path in list_witnesses(repo, bus, "processed_subscribers", event_uuid) {
        if safe_remove_path(&path) {
            counts["witnesses"] = json!(counts["witnesses"].as_i64().unwrap_or(0) + 1);
        }
    }
    let _ = resolved_type;
    Ok(counts)
}

fn emit_kaizen_alert(event_uuid: &str, event_type: &str, witnesses: &[PathBuf]) {
    let mut details = Vec::new();
    for path in witnesses {
        let mut entry = json!({"witness": path.file_name().map(|n| n.to_string_lossy().into_owned())});
        if let Ok(raw) = fs::read_to_string(path) {
            if let Ok(body) = serde_json::from_str::<Value>(&raw) {
                entry["subscriber"] = body.get("subscriber").cloned().unwrap_or(Value::Null);
                entry["error_trace"] = body.get("error_trace").cloned().unwrap_or(Value::Null);
            }
        }
        details.push(entry);
    }
    let alert = json!({
        "alert_type": "kaizen_eda_dead_letter",
        "event_uuid": event_uuid,
        "event_type": event_type,
        "message": "Testigo en dead-letter — padre NO purgado",
        "witnesses": details,
    });
    eprintln!("{}", serde_json::to_string(&alert).unwrap_or_default());
}

#[cfg(test)]
mod fractal_dlq_tests {
    use super::*;
    use std::io::Write;
    use uuid::Uuid;

    #[test]
    fn terminal_with_failure_moves_to_dead_letter() {
        let base = std::env::temp_dir().join(format!("sddia-dlq-{}", Uuid::new_v4()));
        let domain = base.join("domain");
        let dl = base.join("dead-letter");
        fs::create_dir_all(&domain).unwrap();
        let path = domain.join("evt.json");
        let body = json!({
            "event_id": "evt",
            "delivery_state": {
                "cumulo.iota-immutable-publisher": "failed",
                "mayeuta.telegram-fallback-responder": "skipped-already-delivered"
            }
        });
        {
            let mut f = fs::File::create(&path).unwrap();
            f.write_all(serde_json::to_string_pretty(&body).unwrap().as_bytes())
                .unwrap();
        }
        assert!(fractal_event_terminal_with_failure(&body));
        assert!(!fractal_event_all_ok(&body));
        let dest = move_fractal_to_dead_letter(&path, &dl).unwrap();
        assert!(!path.exists());
        assert!(dest.is_file());
        assert_eq!(dest.parent().unwrap(), dl.as_path());
        let _ = fs::remove_dir_all(base);
    }

    #[test]
    fn all_ok_not_terminal_with_failure() {
        let body = json!({
            "delivery_state": {
                "a": "success",
                "b": "skipped-already-delivered"
            }
        });
        assert!(fractal_event_all_ok(&body));
        assert!(!fractal_event_terminal_with_failure(&body));
    }

    #[test]
    fn progress_sweep_leaf_does_not_purge_progress_dirs() {
        let base = std::env::temp_dir().join(format!("sddia-prog-sweep-{}", Uuid::new_v4()));
        let repo = base.join("repo");
        let progress = base.join("progress");
        let corr = "44444444-4444-4444-8444-444444444444";
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::create_dir_all(repo.join(".SddIA/runtime")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            format!(
                r#"{{"eda_bus":{{"store":".SddIA/runtime/events.redb"}},"eda_fractal":{{"telemetry":"{}/telemetry","progress":"{}/progress","orchestration":"{}/orch","domain":"{}/domain","dead_letter":"{}/dl"}}}}"#,
                base.display(),
                base.display(),
                base.display(),
                base.display(),
                base.display(),
            ),
        )
        .unwrap();
        std::fs::create_dir_all(&progress).unwrap();
        std::fs::create_dir_all(progress.join(corr)).unwrap();
        std::fs::write(
            progress.join(corr).join("t1.json"),
            r#"{"trace_id":"t1"}"#,
        )
        .unwrap();
        let top = load_bus_topology(&repo);
        let mut report = SweepReport::default();
        sweep_progress_leaf(&repo, &top, &mut report).unwrap();
        assert!(progress.join(corr).is_dir());
        assert!(report.purged.is_empty());
        let _ = std::fs::remove_dir_all(base);
    }
}
