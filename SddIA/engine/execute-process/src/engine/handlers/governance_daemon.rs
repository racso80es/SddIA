//! Handler nativo `governance-daemon-manager` (P4).

use super::super::daemons::{
    build_process_execution_completed_event, kill_pid, pid_alive, read_lock, remove_lock,
    resolve_daemon, resolve_daemon_binary, write_lock,
};
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;
use std::process::{Command, Stdio};
use uuid::Uuid;

fn emit_orchestration(
    repo: &Path,
    asset_id: &str,
    workspace_path: Option<&str>,
    status: &str,
    operation: &str,
    daemon_id: &str,
    daemon_uuid: &str,
    operation_status: &str,
    os_result: &Value,
) -> Value {
    let event_id = Uuid::new_v4().to_string();
    let mut base = build_process_execution_completed_event(
        &event_id,
        asset_id,
        "governance-daemon-manager",
        status,
        workspace_path,
    );
    if let Some(payload) = base.get_mut("payload").and_then(|v| v.as_object_mut()) {
        payload.insert("operation".into(), json!(operation));
        payload.insert("daemon_id".into(), json!(daemon_id));
        payload.insert("daemon_uuid".into(), json!(daemon_uuid));
        payload.insert("operation_status".into(), json!(operation_status));
        payload.insert("os_result".into(), os_result.clone());
    }
    if let Some(obj) = base.as_object_mut() {
        obj.insert("emitter_agent".into(), json!("governance-daemon-manager"));
    }
    let (_, orch_dir, _, _) = load_fractal_dirs(repo);
    write_fractal_event(repo, &base, &orch_dir).unwrap_or(json!({"error": "orchestration write failed"}))
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let operation = process_inputs
        .get("operation")
        .and_then(|v| v.as_str())
        .ok_or("operation debe ser start|status|kill")?;
    if !matches!(operation, "start" | "status" | "kill") {
        return Err("operation debe ser start|status|kill".into());
    }
    let daemon_id = process_inputs
        .get("daemon_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "daemon_id requerido".to_string())?;
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

    let spec = resolve_daemon(repo, daemon_id)?;
    let entry = repo.join(&spec.entrypoint);
    let lock_rel = super::super::daemons::lock_path(repo, daemon_id)?
        .strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();

    let mut os_result = json!({
        "entrypoint_resolved": spec.entrypoint,
        "runtime": spec.runtime,
        "lock_path": lock_rel,
    });
    let mut operation_status = "failed";
    let mut success = false;

    match operation {
        "start" => {
            let mut started = false;
            if let Some(existing) = read_lock(repo, daemon_id) {
                let pid = existing.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                if pid_alive(pid) {
                    os_result["pid"] = json!(pid);
                    os_result["alive"] = json!(true);
                    operation_status = "noop";
                    success = true;
                    started = true;
                } else {
                    remove_lock(repo, daemon_id);
                }
            }
            if !started {
                let launch_cmd: Vec<String> = if spec.runtime == "native-rust" {
                    if let Some(bin) = resolve_daemon_binary(repo, daemon_id) {
                        vec![bin.to_string_lossy().into_owned()]
                    } else if entry.is_file() {
                        vec![entry.to_string_lossy().into_owned()]
                    } else {
                        return envelope_err(format!(
                            "entrypoint no encontrado: {}",
                            spec.entrypoint
                        ));
                    }
                } else if entry.is_file() {
                    vec![spec.runtime.clone(), entry.to_string_lossy().into_owned()]
                } else {
                    return envelope_err(format!(
                        "entrypoint no encontrado: {}",
                        spec.entrypoint
                    ));
                };
                let mut cmd = Command::new(&launch_cmd[0]);
                if launch_cmd.len() > 1 {
                    cmd.args(&launch_cmd[1..]);
                }
                let child = cmd
                    .current_dir(repo)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .map_err(|e| format!("spawn daemon: {e}"))?;
                write_lock(
                    repo,
                    daemon_id,
                    child.id() as i32,
                    spec.heartbeat_interval_seconds,
                )?;
                os_result["pid"] = json!(child.id());
                os_result["alive"] = json!(true);
                operation_status = "succeeded";
                success = true;
            }
        }
        "status" => {
            if let Some(lock) = read_lock(repo, daemon_id) {
                let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let alive = pid_alive(pid);
                os_result["pid"] = json!(pid);
                os_result["alive"] = json!(alive);
                os_result["started_at"] = lock.get("started_at").cloned().unwrap_or(Value::Null);
                os_result["heartbeat_interval_seconds"] =
                    lock.get("heartbeat_interval_seconds").cloned().unwrap_or(Value::Null);
                if !alive {
                    remove_lock(repo, daemon_id);
                }
            } else {
                os_result["alive"] = json!(false);
                os_result["pid"] = Value::Null;
            }
            operation_status = "succeeded";
            success = true;
        }
        "kill" => {
            if let Some(lock) = read_lock(repo, daemon_id) {
                let pid = lock.get("pid").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                if pid_alive(pid) {
                    let kill_out = kill_pid(pid, grace);
                    for (k, v) in kill_out.as_object().unwrap_or(&serde_json::Map::new()) {
                        os_result[k.clone()] = v.clone();
                    }
                    if kill_out.get("alive").and_then(|v| v.as_bool()) != Some(true) {
                        remove_lock(repo, daemon_id);
                        os_result["lock_removed"] = json!(true);
                    }
                    operation_status = if kill_out.get("alive").and_then(|v| v.as_bool()) == Some(true) {
                        "failed"
                    } else {
                        "succeeded"
                    };
                    success = operation_status != "failed";
                } else {
                    remove_lock(repo, daemon_id);
                    os_result["pid"] = lock.get("pid").cloned().unwrap_or(Value::Null);
                    os_result["alive"] = json!(false);
                    operation_status = "noop";
                    success = true;
                }
            } else {
                os_result["pid"] = Value::Null;
                os_result["alive"] = json!(false);
                operation_status = "noop";
                success = true;
            }
        }
        _ => unreachable!(),
    }

    let orch_status = if success { "success" } else { "failed" };
    let orch = emit_orchestration(
        repo,
        &asset_id,
        workspace_path.as_deref(),
        orch_status,
        operation,
        daemon_id,
        &spec.daemon_uuid,
        operation_status,
        &os_result,
    );

    Ok(OrchestratorEnvelope {
        success,
        status_code: if success { 0 } else { 1 },
        data: Some(json!({
            "operation": operation,
            "daemon_id": daemon_id,
            "daemon_uuid": spec.daemon_uuid,
            "operation_status": operation_status,
            "os_result": os_result,
            "orchestration_event_id": orch.get("event_id"),
            "orchestration_event_path": orch.get("target_path"),
        })),
        error: if success {
            None
        } else {
            Some(format!("operación {operation} falló"))
        },
        execution_report: Some(json!({
            "process_name": "governance-daemon-manager",
            "phases": [{
                "phase_name": "Actuación OS",
                "status": if success { "executed" } else { "failed" },
                "handler": "governance-daemon-manager-core",
            }],
        })),
        exit_code: if success { 0 } else { 1 },
    })
}

fn envelope_err(msg: String) -> Result<OrchestratorEnvelope, String> {
    Ok(OrchestratorEnvelope {
        success: false,
        status_code: 1,
        data: None,
        error: Some(msg),
        execution_report: Some(json!({
            "process_name": "governance-daemon-manager",
            "phases": [],
        })),
        exit_code: 1,
    })
}
