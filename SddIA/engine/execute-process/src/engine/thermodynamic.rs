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

const LIFECYCLE_PROCESSES: &[&str] = &["bug-fix", "feature", "refactorization"];

/// Deriva `cycle_phase` desde `phase_reports` (laudo L5 kalma2-full-cycle).
/// Solo aplica a procesos de ciclo de vida; resto → None (compat bridge).
pub fn derive_cycle_phase(process_name: &str, phase_reports: Option<&Value>) -> Option<&'static str> {
    if !LIFECYCLE_PROCESSES.contains(&process_name) {
        return None;
    }
    let Some(arr) = phase_reports.and_then(|v| v.as_array()) else {
        return Some("completed");
    };
    let mut has_awaiting = false;
    let mut has_simulated = false;
    for p in arr {
        let st = p.get("status").and_then(|v| v.as_str()).unwrap_or("");
        if matches!(st, "awaiting" | "awaiting_agents") {
            has_awaiting = true;
        } else if st == "simulated" {
            has_simulated = true;
        }
    }
    if has_awaiting {
        Some("awaiting_agents")
    } else if has_simulated {
        Some("initialized")
    } else {
        Some("completed")
    }
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

    // PEC: éxito (legado) o fallo con correlation_id (lazo Kalma2 — evita 404/timeout ciego).
    let ws = workspace_path
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let emit_pec = (success && (ws.is_some() || correlation_id.is_some()))
        || (!success && correlation_id.is_some());
    if emit_pec {
        let orch_id = Uuid::new_v4().to_string();
        let phase_reports = state.get("phase_reports");
        let phase_count = phase_reports.and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
        let pec_status = if success { "success" } else { "failed" };
        let mut payload = json!({
            "asset_id": asset_id,
            "process_name": process_name,
            "status": pec_status,
            "exit_code": exit_code,
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
        if success {
            if let Some(phase) = derive_cycle_phase(process_name, phase_reports) {
                payload["cycle_phase"] = json!(phase);
            }
        } else {
            payload["cycle_phase"] = json!("failed");
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

    result
}

/// Emite PEC temprano `initialized` (TQM → hijo lifecycle) para sondeo Kalma2.
pub fn emit_initialized_pec(
    repo: &Path,
    process_name: &str,
    correlation_id: &str,
) -> Result<Value, String> {
    let (_, orch_dir, _) = load_fractal_dirs(repo);
    fs::create_dir_all(repo.join(&orch_dir)).ok();
    let orch_id = Uuid::new_v4().to_string();
    let payload = json!({
        "process_name": process_name,
        "status": "success",
        "correlation_id": correlation_id,
        "cycle_phase": "initialized",
        "emitter_hint": "task-queue-manager-early-pec",
    });
    let orch_event = json!({
        "event_id": orch_id,
        "event_type": "Process_Execution_Completed",
        "event_family": "orchestration",
        "timestamp": iso_now(),
        "emitter_agent": "task-queue-manager",
        "payload": payload,
        "delivery_state": {},
    });
    write_fractal_event(repo, &orch_event, &orch_dir)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_initialized_when_simulated() {
        let reports = json!([
            {"phase_name": "Inicialización", "status": "executed"},
            {"phase_name": "Diseño", "status": "simulated"},
            {"phase_name": "Cierre", "status": "skipped"}
        ]);
        assert_eq!(
            derive_cycle_phase("bug-fix", Some(&reports)),
            Some("initialized")
        );
    }

    #[test]
    fn derive_awaiting_agents_priority() {
        let reports = json!([
            {"status": "simulated"},
            {"status": "awaiting_agents"}
        ]);
        assert_eq!(
            derive_cycle_phase("feature", Some(&reports)),
            Some("awaiting_agents")
        );
    }

    #[test]
    fn derive_completed_without_simulated() {
        let reports = json!([
            {"status": "executed"},
            {"status": "skipped"}
        ]);
        assert_eq!(
            derive_cycle_phase("bug-fix", Some(&reports)),
            Some("completed")
        );
    }

    #[test]
    fn derive_none_for_non_lifecycle() {
        let reports = json!([{"status": "simulated"}]);
        assert_eq!(derive_cycle_phase("task-queue-manager", Some(&reports)), None);
    }

    #[test]
    fn emit_initialized_pec_writes_orchestration() {
        let tmp = tempfile::tempdir().expect("tmp");
        let repo = tmp.path();
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"telemetry":"./.events/telemetry","orchestration":"./.events/orchestration","domain":"./.events/domain"}}"#,
        )
        .unwrap();
        let seal = emit_initialized_pec(repo, "feature", "11111111-1111-4111-8111-111111111111")
            .expect("pec");
        let path = seal.get("target_path").and_then(|v| v.as_str()).unwrap();
        let raw = std::fs::read_to_string(repo.join(path)).unwrap();
        let ev: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(ev["event_type"], "Process_Execution_Completed");
        assert_eq!(ev["payload"]["cycle_phase"], "initialized");
        assert_eq!(ev["payload"]["correlation_id"], "11111111-1111-4111-8111-111111111111");
    }
}
