//! Handler nativo `daemon-heartbeat-audit` (P4).

use super::super::daemons::{
    daemon_interval, iso_now, list_indexed_daemon_ids, load_eda_pending, parse_iso, pid_alive,
    read_lock, resolve_daemon_uuid, stamp_delivery_state, state_dir, write_json_atomic,
};
use chrono::Utc;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

const SUBSCRIBER_KEY: &str = "argos.daemon-heartbeat-audit";
const MISSED_CYCLES_THRESHOLD: i64 = 3;

fn heartbeat_state_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    Ok(state_dir(repo)?.join("heartbeat-audit.json"))
}

fn load_state(repo: &Path) -> Value {
    let path = heartbeat_state_path(repo).ok();
    let Some(path) = path.filter(|p| p.is_file()) else {
        return json!({"daemons": {}});
    };
    let mut body = fs::read_to_string(&path)
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .unwrap_or(json!({"daemons": {}}));
    if !body.get("daemons").and_then(|v| v.as_object()).is_some() {
        if let Some(obj) = body.as_object_mut() {
            obj.insert("daemons".into(), json!({}));
        }
    }
    body
}

fn save_state(repo: &Path, state: &Value) -> Result<(), String> {
    write_json_atomic(&heartbeat_state_path(repo)?, state)
}

fn emit_system_fracture(
    repo: &Path,
    daemon_id: &str,
    daemon_uuid: &str,
    missed_cycles: i64,
    last_heartbeat_at: Option<&str>,
) -> Result<Value, String> {
    let pending = load_eda_pending(repo)?;
    let event_id = Uuid::new_v4().to_string();
    let error_trace = format!(
        "Centinela {daemon_id} omitió {missed_cycles} ciclos consecutivos de Daemon_Heartbeat (umbral={MISSED_CYCLES_THRESHOLD}). last_heartbeat={}",
        last_heartbeat_at.unwrap_or("never")
    );
    let event = json!({
        "event_id": event_id,
        "event_type": "System_Fracture_Detected",
        "timestamp": iso_now(),
        "emitter_agent": "argos",
        "payload": {
            "process_name": daemon_id,
            "error_trace": error_trace,
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
            "daemon_uuid": daemon_uuid,
            "missed_cycles": missed_cycles,
        },
    });
    let target = repo.join(&pending).join(format!("{event_id}.json"));
    write_json_atomic(&target, &event)?;
    Ok(json!({
        "event_id": event_id,
        "target_path": target.strip_prefix(repo).unwrap_or(&target).to_string_lossy().replace('\\', "/"),
    }))
}

fn record_heartbeat(state: &mut Value, repo: &Path, payload: &Value) {
    let Some(daemon_name) = payload.get("daemon_name").and_then(|v| v.as_str()) else {
        return;
    };
    let daemon_id = daemon_name.trim();
    if daemon_id.is_empty() {
        return;
    }
    let daemons = state
        .as_object_mut()
        .and_then(|o| o.get_mut("daemons"))
        .and_then(|d| d.as_object_mut());
    let Some(daemons) = daemons else {
        return;
    };
    let mut entry = daemons
        .get(daemon_id)
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    entry.insert("last_heartbeat_at".into(), json!(iso_now()));
    entry.insert("missed_cycles".into(), json!(0));
    entry.insert(
        "heartbeat_interval_seconds".into(),
        json!(daemon_interval(repo, daemon_id)),
    );
    entry.remove("fracture_event_id");
    daemons.insert(daemon_id.to_string(), Value::Object(entry));
}

/// Baseline de latido: el más reciente entre estado persistido y arranque del lock.
/// Evita falsos positivos tras downtime + reinicio (last_hb obsoleto, PID nuevo).
fn effective_heartbeat_baseline(
    last_heartbeat_at: Option<&str>,
    lock_started_at: Option<&str>,
) -> Option<chrono::DateTime<Utc>> {
    let hb = last_heartbeat_at.and_then(parse_iso);
    let started = lock_started_at.and_then(parse_iso);
    match (hb, started) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn audit_running_daemon(
    repo: &Path,
    state: &mut Value,
    daemon_id: &str,
) -> Result<Option<Value>, String> {
    let lock = read_lock(repo, daemon_id);
    let Some(lock) = lock else {
        return Ok(None);
    };
    let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    if !pid_alive(pid) {
        return Ok(None);
    }

    let interval = daemon_interval(repo, daemon_id) as i64;
    let daemons = state
        .as_object_mut()
        .and_then(|o| o.get_mut("daemons"))
        .and_then(|d| d.as_object_mut())
        .ok_or("state daemons invalid")?;
    let mut entry = daemons
        .get(daemon_id)
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    entry.insert("heartbeat_interval_seconds".into(), json!(interval));

    let baseline = effective_heartbeat_baseline(
        entry.get("last_heartbeat_at").and_then(|v| v.as_str()),
        lock.get("started_at").and_then(|v| v.as_str()),
    );
    let elapsed = if let Some(last) = baseline {
        (Utc::now() - last).num_seconds().max(0) as f64
    } else {
        (interval * MISSED_CYCLES_THRESHOLD) as f64
    };
    let missed = (elapsed / interval as f64) as i64;
    entry.insert("missed_cycles".into(), json!(missed));
    daemons.insert(daemon_id.to_string(), Value::Object(entry.clone()));

    if missed < MISSED_CYCLES_THRESHOLD {
        return Ok(None);
    }
    if entry.get("fracture_event_id").is_some() {
        return Ok(None);
    }

    let baseline_iso = baseline.map(|dt| dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    let seal = emit_system_fracture(
        repo,
        daemon_id,
        &resolve_daemon_uuid(repo, daemon_id),
        missed,
        baseline_iso.as_deref(),
    )?;
    let mut updated = entry;
    updated.insert(
        "fracture_event_id".into(),
        seal.get("event_id").cloned().unwrap_or(Value::Null),
    );
    daemons.insert(daemon_id.to_string(), Value::Object(updated));
    Ok(Some(seal))
}

fn audit_staleness(repo: &Path) -> Result<Vec<Value>, String> {
    let mut state = load_state(repo);
    let mut fractures = Vec::new();
    for daemon_id in list_indexed_daemon_ids(repo)? {
        if let Some(seal) = audit_running_daemon(repo, &mut state, &daemon_id)? {
            fractures.push(seal);
        }
    }
    save_state(repo, &state)?;
    Ok(fractures)
}

pub fn audit_telemetry_file(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let event_path = (repo.join(rel_path.trim()))
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if !event_path.is_file() {
        return Ok(json!({"ok": false, "error": format!("no existe: {rel_path}")}));
    }
    let body: Value =
        serde_json::from_str(&fs::read_to_string(&event_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    if body.get("event_type").and_then(|v| v.as_str()) != Some("Daemon_Heartbeat") {
        stamp_delivery_state(&event_path, SUBSCRIBER_KEY, "skipped");
        return Ok(json!({"ok": true, "status": "skipped", "reason": "wrong_event_type"}));
    }
    let payload = body.get("payload").cloned().unwrap_or(json!({}));
    if !payload.is_object() {
        return Ok(json!({"ok": false, "error": "payload invalido"}));
    }

    let mut state = load_state(repo);
    record_heartbeat(&mut state, repo, &payload);
    save_state(repo, &state)?;

    let fractures = audit_staleness(repo)?;
    stamp_delivery_state(&event_path, SUBSCRIBER_KEY, "success");
    Ok(json!({
        "ok": true,
        "status": "audited",
        "fractures_emitted": fractures,
        "daemon_name": payload.get("daemon_name"),
    }))
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    if let Some(rel) = process_inputs.get("event_file_path").and_then(|v| v.as_str()) {
        if !rel.trim().is_empty() {
            let result = audit_telemetry_file(repo, rel.trim())?;
            let ok = result.get("ok").and_then(|v| v.as_bool()) == Some(true);
            return Ok(OrchestratorEnvelope {
                success: ok,
                status_code: if ok { 0 } else { 1 },
                data: Some(result.clone()),
                error: if ok {
                    None
                } else {
                    result
                        .get("error")
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                },
                execution_report: Some(json!({
                    "process_name": "daemon-heartbeat-audit",
                    "phases": [{
                        "phase_name": "Auditoría staleness",
                        "status": if ok { "executed" } else { "failed" },
                        "handler": "daemon-heartbeat-audit-core",
                    }],
                })),
                exit_code: if ok { 0 } else { 1 },
            });
        }
    }

    let fractures = audit_staleness(repo)?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({"status": "sweep", "fractures_emitted": fractures})),
        error: None,
        execution_report: Some(json!({
            "process_name": "daemon-heartbeat-audit",
            "phases": [{
                "phase_name": "Auditoría staleness",
                "status": "executed",
                "handler": "daemon-heartbeat-audit-core",
            }],
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_prefers_newer_started_at_on_cold_start() {
        let stale = "2026-07-19T17:13:18Z";
        let started = "2026-07-19T17:32:58Z";
        let baseline = effective_heartbeat_baseline(Some(stale), Some(started)).unwrap();
        assert_eq!(
            baseline.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2026-07-19T17:32:58Z"
        );
        let elapsed = (parse_iso("2026-07-19T17:33:01Z").unwrap() - baseline)
            .num_seconds()
            .max(0);
        let missed = elapsed / 30;
        assert!(missed < MISSED_CYCLES_THRESHOLD, "missed={missed}");
    }

    #[test]
    fn baseline_prefers_newer_heartbeat_in_steady_state() {
        let hb = "2026-07-19T17:40:29Z";
        let started = "2026-07-19T17:32:58Z";
        let baseline = effective_heartbeat_baseline(Some(hb), Some(started)).unwrap();
        assert_eq!(
            baseline.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2026-07-19T17:40:29Z"
        );
    }

    #[test]
    fn baseline_falls_back_to_started_when_no_heartbeat() {
        let started = "2026-07-19T17:32:58Z";
        let baseline = effective_heartbeat_baseline(None, Some(started)).unwrap();
        assert_eq!(
            baseline.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2026-07-19T17:32:58Z"
        );
    }
}
