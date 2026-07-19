//! Peaje Termodinámico — emisión fractal fail-soft (D3.13).

use super::fractal::{load_fractal_dirs, write_fractal_event};
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn run(
    repo: &Path,
    process_name: &str,
    state: &Value,
    process_inputs: &Value,
    exit_code: i32,
    duration_ms: i64,
    success: bool,
) -> Value {
    let (tele_dir, orch_dir, _) = load_fractal_dirs(repo);
    fs::create_dir_all(repo.join(&tele_dir)).ok();
    fs::create_dir_all(repo.join(&orch_dir)).ok();

    let asset_id = state
        .get("asset_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let execution_id = state.get("execution_id").and_then(|v| v.as_str());
    let workspace_path = state
        .get("workspace_path")
        .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path")))
        .and_then(|v| v.as_str());
    let persist_ref = process_inputs.get("persist_ref").and_then(|v| v.as_str());
    let correlation_id = process_inputs
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let mut result = json!({
        "asset_id": asset_id,
        "duration_ms": duration_ms,
        "exit_code": exit_code,
    });

    if std::env::var("SDDIA_CHAOS_SIMULATE_TELEMETRY_IO_FAIL")
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes"))
        .unwrap_or(false)
    {
        result["telemetry_io_failed"] = json!(true);
        result["telemetry_error"] = json!("chaos lab: simulated telemetry I/O failure");
    } else {
        let telemetry_id = Uuid::new_v4().to_string();
        let mut payload = json!({
            "asset_id": asset_id,
            "exit_code": exit_code,
            "duration_ms": duration_ms,
            "process_name": process_name,
        });
        if let Some(eid) = execution_id {
            payload["execution_id"] = json!(eid);
        }
        if let Some(ws) = workspace_path {
            payload["workspace_path"] = json!(ws);
        }
        let telemetry_event = json!({
            "event_id": telemetry_id,
            "event_type": "Raw_Execution_Finished",
            "event_family": "telemetry",
            "timestamp": iso_now(),
            "emitter_agent": "execute-process",
            "payload": payload,
            "delivery_state": {},
        });
        match write_fractal_event(repo, &telemetry_event, &tele_dir) {
            Ok(seal) => {
                result["telemetry"] = seal;
            }
            Err(e) => {
                result["telemetry_error"] = json!(e);
                result["telemetry_io_failed"] = json!(true);
                eprintln!("[THERMODYNAMIC-TOLL-EMERGENCY] process={process_name} channel=telemetry: {e}");
            }
        }
    }

    if success {
        let ws = workspace_path
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        // PEC: workspace (legado) o correlation_id (lazo Kalma2 / EDA).
        if ws.is_some() || correlation_id.is_some() {
            let orch_id = Uuid::new_v4().to_string();
            let phase_count = state
                .get("phase_reports")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);
            let mut payload = json!({
                "asset_id": asset_id,
                "process_name": process_name,
                "status": "success",
            });
            if let Some(ref path) = ws {
                payload["workspace_path"] = json!(path);
            }
            if let Some(eid) = execution_id {
                payload["execution_id"] = json!(eid);
            }
            if phase_count > 0 {
                payload["phase_count"] = json!(phase_count);
            }
            if let Some(pr) = persist_ref {
                payload["persist_ref"] = json!(pr);
            }
            if let Some(ref cid) = correlation_id {
                payload["correlation_id"] = json!(cid);
            }
            let orch_event = json!({
                "event_id": orch_id,
                "event_type": "Process_Execution_Completed",
                "event_family": "orchestration",
                "timestamp": iso_now(),
                "emitter_agent": "execute-process",
                "payload": payload,
                "delivery_state": {},
            });
            match write_fractal_event(repo, &orch_event, &orch_dir) {
                Ok(seal) => {
                    result["orchestration"] = seal;
                }
                Err(e) => {
                    result["orchestration_error"] = json!(e);
                    result["orchestration_io_failed"] = json!(true);
                    eprintln!("[THERMODYNAMIC-TOLL-EMERGENCY] process={process_name} channel=orchestration: {e}");
                }
            }
        }
    }

    result
}

const THERMODYNAMIC_EXEMPT: &[&str] = &[
    "route-domain-event",
    "route-telemetry",
    "route-orchestration",
    "route-domain",
    "telemetry-batch-stub",
    "radamanto-batch",
    "memory-evolution-ingest",
    "cerbero-governance-react",
    "fix-tool-process",
    "kalma2-interact",
];

pub fn is_exempt(process_name: &str) -> bool {
    THERMODYNAMIC_EXEMPT.contains(&process_name)
}
