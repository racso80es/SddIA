use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn write_capsule_envelope(
    workspace_path: &Path,
    correlation_id: &str,
    tool_name: &str,
    request: &Value,
    response: &Value,
) -> Result<(), String> {
    fs::create_dir_all(workspace_path).map_err(|e| e.to_string())?;
    let file = workspace_path.join(format!("capsule-invoke-{correlation_id}.json"));
    let envelope = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": "sddia-workspace-server",
        },
        "request": {
            "mcp_tool": tool_name,
            "capsule_request": request,
        },
        "result": response,
    });
    fs::write(&file, serde_json::to_string_pretty(&envelope).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn emit_raw_execution_finished(
    sddia_repo: &Path,
    correlation_id: &str,
    tool_name: &str,
    exit_code: i32,
    duration_ms: u128,
    workspace_path: Option<&Path>,
) -> Result<(), String> {
    let tele_dir = sddia_repo.join(".events/telemetry");
    fs::create_dir_all(&tele_dir).map_err(|e| e.to_string())?;
    let event_id = Uuid::new_v4().to_string();
    let mut payload = json!({
        "asset_id": "sddia-workspace-server",
        "exit_code": exit_code,
        "duration_ms": duration_ms,
        "process_name": tool_name,
        "correlation_id": correlation_id,
    });
    if let Some(ws) = workspace_path {
        payload["workspace_path"] = json!(ws.to_string_lossy());
    }
    let event = json!({
        "event_id": event_id,
        "event_type": "Raw_Execution_Finished",
        "event_family": "telemetry",
        "timestamp": chrono_lite_now(),
        "emitter_agent": "sddia-workspace-server",
        "payload": payload,
        "delivery_state": {},
    });
    let path = tele_dir.join(format!("{event_id}.json"));
    fs::write(path, serde_json::to_string_pretty(&event).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}
