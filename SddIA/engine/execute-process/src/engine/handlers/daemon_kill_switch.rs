//! Handler nativo `daemon-kill-switch` (P4).

use super::governance_daemon::run as run_governance;
use super::super::daemons::{
    build_process_execution_completed_event, list_indexed_daemon_ids, pid_alive, read_lock,
    remove_lock, status_dir,
};
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let grace = process_inputs
        .get("kill_grace_seconds")
        .and_then(|v| v.as_i64())
        .unwrap_or(10) as i32;
    let asset_id = process_inputs
        .get("asset_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let workspace_path = process_inputs
        .get("repository_path")
        .or_else(|| process_inputs.get("workspace_path"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let daemon_ids = list_indexed_daemon_ids(repo)?;
    let mut purge_report: Vec<Value> = Vec::new();
    let mut all_ok = true;

    for daemon_id in &daemon_ids {
        let mut entry = json!({"daemon_id": daemon_id});
        if let Some(lock) = read_lock(repo, daemon_id) {
            let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            if pid_alive(pid) {
                let mut kill_inputs = json!({
                    "operation": "kill",
                    "daemon_id": daemon_id,
                    "kill_grace_seconds": grace,
                    "asset_id": asset_id,
                });
                if let Some(ref ws) = workspace_path {
                    kill_inputs["repository_path"] = json!(ws);
                }
                let out = run_governance(repo, &kill_inputs)?;
                entry["kill"] = out.data.clone().unwrap_or(json!({}));
                if !out.success {
                    all_ok = false;
                }
            } else {
                remove_lock(repo, daemon_id);
                entry["kill"] = json!({"operation_status": "noop", "reason": "no live lock"});
            }
        } else {
            entry["kill"] = json!({"operation_status": "noop", "reason": "no live lock"});
        }
        purge_report.push(entry);
    }

    let mut stale_locks: Vec<String> = Vec::new();
    let status_root = status_dir(repo)?;
    if status_root.is_dir() {
        for entry in std::fs::read_dir(&status_root).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("lock") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            if let Some(body) = read_lock(repo, stem) {
                let pid = body.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                if pid_alive(pid) {
                    continue;
                }
            }
            remove_lock(repo, stem);
            stale_locks.push(stem.to_string());
        }
    }

    let event_id = Uuid::new_v4().to_string();
    let status = if all_ok { "success" } else { "failed" };
    let mut event = build_process_execution_completed_event(
        &event_id,
        &asset_id,
        "daemon-kill-switch",
        status,
        workspace_path.as_deref(),
    );
    if let Some(payload) = event.get_mut("payload").and_then(|v| v.as_object_mut()) {
        payload.insert("purge_report".into(), json!(purge_report));
        payload.insert("stale_locks_removed".into(), json!(stale_locks));
        payload.insert("daemon_count".into(), json!(daemon_ids.len()));
    }
    if let Some(obj) = event.as_object_mut() {
        obj.insert("emitter_agent".into(), json!("daemon-kill-switch"));
    }
    let (_, orch_dir, _, _) = load_fractal_dirs(repo);
    let orch = write_fractal_event(repo, &event, &orch_dir).unwrap_or(json!({"error": "write failed"}));

    Ok(OrchestratorEnvelope {
        success: all_ok,
        status_code: if all_ok { 0 } else { 1 },
        data: Some(json!({
            "purge_report": purge_report,
            "stale_locks_removed": stale_locks,
            "orchestration_event_id": orch.get("event_id"),
            "orchestration_event_path": orch.get("target_path"),
        })),
        error: if all_ok {
            None
        } else {
            Some("purga parcial fallida".into())
        },
        execution_report: Some(json!({
            "process_name": "daemon-kill-switch",
            "phases": [{
                "phase_name": "Purga global",
                "status": if all_ok { "executed" } else { "failed" },
                "handler": "daemon-kill-switch-core",
            }],
        })),
        exit_code: if all_ok { 0 } else { 1 },
    })
}
