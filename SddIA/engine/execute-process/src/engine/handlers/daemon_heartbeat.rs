//! Handler nativo `daemon-heartbeat-audit` (P4).

use super::heartbeat_audit_thresholds::{
    load_heartbeat_audit_thresholds, monotonic_ms, HeartbeatAuditThresholds,
};
use super::phagocyte_recovered_fracture_pbis::{env_apply_enabled, run_phagocyte};
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

struct AuditClockResult {
    host_suspend: bool,
    skew_seconds: Option<i64>,
}

fn update_audit_clocks(state: &mut Value, thresholds: &HeartbeatAuditThresholds) -> AuditClockResult {
    let now_wall = iso_now();
    let now_mono = monotonic_ms();
    let root = match state.as_object_mut() {
        Some(o) => o,
        None => {
            return AuditClockResult {
                host_suspend: false,
                skew_seconds: None,
            }
        }
    };

    let last_wall = root
        .get("last_audit_wall_at")
        .and_then(|v| v.as_str())
        .and_then(parse_iso);
    let last_mono = root.get("last_audit_mono_ms").and_then(|v| v.as_u64());

    let mut result = AuditClockResult {
        host_suspend: false,
        skew_seconds: None,
    };

    if let (Some(lw), Some(lm), Some(nm)) = (last_wall, last_mono, now_mono) {
        let wall_delta = (Utc::now() - lw).num_seconds();
        let mono_delta_ms = nm.saturating_sub(lm);
        let mono_delta = (mono_delta_ms / 1000) as i64;
        let skew = wall_delta - mono_delta;
        result.skew_seconds = Some(skew);
        if skew >= thresholds.suspend_skew_seconds {
            result.host_suspend = true;
        }
    }

    root.insert("last_audit_wall_at".into(), json!(now_wall));
    if let Some(nm) = now_mono {
        root.insert("last_audit_mono_ms".into(), json!(nm));
    }

    result
}

fn reanchor_daemons_on_suspend(state: &mut Value, repo: &Path, now_iso: &str) {
    let daemons = match state
        .as_object_mut()
        .and_then(|o| o.get_mut("daemons"))
        .and_then(|d| d.as_object_mut())
    {
        Some(d) => d,
        None => return,
    };
    for daemon_id in list_indexed_daemon_ids(repo).unwrap_or_default() {
        let lock = read_lock(repo, &daemon_id);
        let Some(lock) = lock else { continue };
        let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if !pid_alive(pid) {
            continue;
        }
        let mut entry = daemons
            .get(&daemon_id)
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        entry.insert("last_heartbeat_at".into(), json!(now_iso));
        entry.insert("missed_cycles".into(), json!(0));
        entry.insert("classification".into(), json!("host_suspend"));
        entry.insert(
            "heartbeat_interval_seconds".into(),
            json!(daemon_interval(repo, &daemon_id)),
        );
        daemons.insert(daemon_id, Value::Object(entry));
    }
}

fn emit_system_fracture(
    repo: &Path,
    daemon_id: &str,
    daemon_uuid: &str,
    missed_cycles: i64,
    last_heartbeat_at: Option<&str>,
    threshold: i64,
) -> Result<Value, String> {
    let pending = load_eda_pending(repo)?;
    let event_id = Uuid::new_v4().to_string();
    let error_trace = format!(
        "Centinela {daemon_id} omitió {missed_cycles} ciclos consecutivos de Daemon_Heartbeat (umbral={threshold}). last_heartbeat={}",
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

fn record_heartbeat_at(state: &mut Value, repo: &Path, payload: &Value, at: Option<&str>) {
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
    let had_fracture = entry.get("fracture_event_id").is_some();
    let ts = at
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| payload.get("timestamp").and_then(|v| v.as_str()))
        .unwrap_or_else(|| "");
    let ts = if ts.is_empty() { iso_now() } else { ts.to_string() };
    let prev = entry
        .get("last_heartbeat_at")
        .and_then(|v| v.as_str())
        .and_then(parse_iso);
    let next = parse_iso(&ts);
    let keep = match (prev, next) {
        (Some(p), Some(n)) if n < p => p.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        (_, Some(n)) => n.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        _ => ts,
    };
    entry.insert("last_heartbeat_at".into(), json!(keep));
    entry.insert("missed_cycles".into(), json!(0));
    entry.insert(
        "heartbeat_interval_seconds".into(),
        json!(daemon_interval(repo, daemon_id)),
    );
    entry.insert(
        "classification".into(),
        json!(if had_fracture { "recovered" } else { "healthy" }),
    );
    entry.remove("fracture_event_id");
    daemons.insert(daemon_id.to_string(), Value::Object(entry));
}

fn heartbeats_side_dir(repo: &Path) -> Result<std::path::PathBuf, String> {
    Ok(state_dir(repo)?.join("heartbeats"))
}

fn telemetry_dir(repo: &Path) -> std::path::PathBuf {
    use super::super::eda_bus::load_eda_fractal;
    load_eda_fractal(repo)
        .ok()
        .and_then(|m| m.get("telemetry").cloned())
        .map(|rel| {
            let t = rel.trim().trim_start_matches("./");
            repo.join(t)
        })
        .unwrap_or_else(|| repo.join(".events/telemetry"))
}

fn ingest_regime(repo: &Path, state: &mut Value) -> Result<u32, String> {
    let mut ingested = 0u32;

    if let Ok(dir) = heartbeats_side_dir(repo) {
        if dir.is_dir() {
            for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("json") {
                    continue;
                }
                let Ok(raw) = fs::read_to_string(&path) else {
                    continue;
                };
                let Ok(body) = serde_json::from_str::<Value>(&raw) else {
                    continue;
                };
                let ts = body.get("timestamp").and_then(|v| v.as_str());
                let name = body
                    .get("daemon_name")
                    .and_then(|v| v.as_str())
                    .or_else(|| path.file_stem().and_then(|s| s.to_str()));
                let Some(name) = name else { continue };
                let payload = json!({
                    "daemon_name": name,
                    "timestamp": ts,
                    "pid": body.get("pid"),
                    "status": body.get("status").unwrap_or(&json!("alive")),
                });
                record_heartbeat_at(state, repo, &payload, ts);
                ingested += 1;
            }
        }
    }

    let tel = telemetry_dir(repo);
    if tel.is_dir() {
        let mut latest: std::collections::HashMap<String, (std::time::SystemTime, Value, String)> =
            std::collections::HashMap::new();
        for entry in fs::read_dir(&tel).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(meta) = path.metadata() else { continue };
            let Ok(mtime) = meta.modified() else { continue };
            let Ok(raw) = fs::read_to_string(&path) else { continue };
            let Ok(body) = serde_json::from_str::<Value>(&raw) else { continue };
            if body.get("event_type").and_then(|v| v.as_str()) != Some("Daemon_Heartbeat") {
                continue;
            }
            let payload = body.get("payload").cloned().unwrap_or(json!({}));
            let Some(name) = payload.get("daemon_name").and_then(|v| v.as_str()) else {
                continue;
            };
            let ts = body
                .get("timestamp")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let replace = match latest.get(name) {
                None => true,
                Some((prev, _, _)) => mtime > *prev,
            };
            if replace {
                latest.insert(name.to_string(), (mtime, payload, ts));
            }
        }
        for (_name, (_mtime, payload, ts)) in latest {
            let at = if ts.is_empty() { None } else { Some(ts.as_str()) };
            record_heartbeat_at(state, repo, &payload, at);
            ingested += 1;
        }
    }

    Ok(ingested)
}

pub fn effective_heartbeat_baseline(
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
    thresholds: &HeartbeatAuditThresholds,
    host_suspend: bool,
) -> Result<Option<Value>, String> {
    let lock = read_lock(repo, daemon_id);
    let Some(lock) = lock else {
        return Ok(None);
    };
    let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    if !pid_alive(pid) {
        return Ok(None);
    }

    if host_suspend {
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
        (interval * thresholds.missed_cycles_threshold) as f64
    };
    let missed = (elapsed / interval as f64) as i64;
    entry.insert("missed_cycles".into(), json!(missed));
    if missed >= thresholds.missed_cycles_threshold {
        entry.insert("classification".into(), json!("stale"));
    }
    daemons.insert(daemon_id.to_string(), Value::Object(entry.clone()));

    if missed < thresholds.missed_cycles_threshold {
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
        thresholds.missed_cycles_threshold,
    )?;
    let mut updated = entry;
    updated.insert(
        "fracture_event_id".into(),
        seal.get("event_id").cloned().unwrap_or(Value::Null),
    );
    daemons.insert(daemon_id.to_string(), Value::Object(updated));
    Ok(Some(seal))
}

fn all_running_healthy(state: &Value, repo: &Path, thresholds: &HeartbeatAuditThresholds) -> bool {
    for daemon_id in list_indexed_daemon_ids(repo).unwrap_or_default() {
        let lock = read_lock(repo, &daemon_id);
        let Some(lock) = lock else { continue };
        let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if !pid_alive(pid) {
            continue;
        }
        let missed = state
            .get("daemons")
            .and_then(|d| d.get(&daemon_id))
            .and_then(|e| e.get("missed_cycles"))
            .and_then(|v| v.as_i64())
            .unwrap_or(thresholds.missed_cycles_threshold);
        if missed > 0 {
            return false;
        }
    }
    true
}

struct AuditStalenessResult {
    fractures: Vec<Value>,
    suspend_reanchored: bool,
    skew_seconds: Option<i64>,
    phagocyte: Option<Value>,
}

fn audit_staleness(repo: &Path) -> Result<AuditStalenessResult, String> {
    let thresholds = load_heartbeat_audit_thresholds(repo);
    let mut state = load_state(repo);
    let clock = update_audit_clocks(&mut state, &thresholds);
    let now_iso = iso_now();
    if clock.host_suspend {
        reanchor_daemons_on_suspend(&mut state, repo, &now_iso);
    }
    let _ = ingest_regime(repo, &mut state)?;
    let mut fractures = Vec::new();
    for daemon_id in list_indexed_daemon_ids(repo)? {
        if let Some(seal) = audit_running_daemon(
            repo,
            &mut state,
            &daemon_id,
            &thresholds,
            clock.host_suspend,
        )? {
            fractures.push(seal);
        }
    }
    save_state(repo, &state)?;

    let mut phagocyte = None;
    if fractures.is_empty() && all_running_healthy(&state, repo, &thresholds) {
        phagocyte = Some(run_phagocyte(repo, env_apply_enabled())?);
    }

    Ok(AuditStalenessResult {
        fractures,
        suspend_reanchored: clock.host_suspend,
        skew_seconds: clock.skew_seconds,
        phagocyte,
    })
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
    let event_ts = body.get("timestamp").and_then(|v| v.as_str());
    record_heartbeat_at(&mut state, repo, &payload, event_ts);
    let _ = ingest_regime(repo, &mut state)?;
    save_state(repo, &state)?;

    let audit = audit_staleness(repo)?;
    stamp_delivery_state(&event_path, SUBSCRIBER_KEY, "success");
    Ok(json!({
        "ok": true,
        "status": "audited",
        "fractures_emitted": audit.fractures,
        "suspend_reanchored": audit.suspend_reanchored,
        "skew_seconds": audit.skew_seconds,
        "phagocyte": audit.phagocyte,
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

    let audit = audit_staleness(repo)?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "status": "sweep",
            "fractures_emitted": audit.fractures,
            "suspend_reanchored": audit.suspend_reanchored,
            "skew_seconds": audit.skew_seconds,
            "phagocyte": audit.phagocyte,
        })),
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
    use super::super::heartbeat_audit_thresholds::HeartbeatAuditThresholds;

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
        assert!(missed < 3, "missed={missed}");
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

    #[test]
    fn suspend_skew_detected_when_wall_runs_ahead_of_mono() {
        let mut state = json!({"daemons": {}});
        let thresholds = HeartbeatAuditThresholds {
            missed_cycles_threshold: 3,
            suspend_skew_seconds: 60,
        };
        let past = (Utc::now() - chrono::Duration::hours(2))
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        state
            .as_object_mut()
            .unwrap()
            .insert("last_audit_wall_at".into(), json!(past));
        state
            .as_object_mut()
            .unwrap()
            .insert("last_audit_mono_ms".into(), json!(monotonic_ms().unwrap()));
        let result = update_audit_clocks(&mut state, &thresholds);
        assert!(result.host_suspend);
        assert!(result.skew_seconds.unwrap_or(0) >= 60);
    }

    #[test]
    fn thresholds_from_ssot_not_hardcoded_const() {
        let dir = tempfile::TempDir::new().unwrap();
        let repo = dir.path();
        std::fs::create_dir_all(repo.join("SddIA/daemons")).unwrap();
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"argos":{"heartbeat_audit_thresholds":"SddIA/daemons/heartbeat-audit.thresholds.json"},"daemons_instance":{"state":".SddIA/daemons/state"}}"#,
        )
        .unwrap();
        std::fs::write(
            repo.join("SddIA/daemons/heartbeat-audit.thresholds.json"),
            r#"{"missed_cycles_threshold":9,"suspend_skew_seconds":300}"#,
        )
        .unwrap();
        std::fs::create_dir_all(repo.join(".SddIA/daemons/state")).unwrap();
        let t = load_heartbeat_audit_thresholds(repo);
        assert_eq!(t.missed_cycles_threshold, 9);
    }
}
