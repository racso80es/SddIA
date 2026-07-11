//! Handler `route-domain-event` (P4) — core EDA nativo Rust.

use crate::envelope::OrchestratorEnvelope;
use crate::engine::route_domain_core::route_domain_event;
use serde_json::{json, Value};
use std::path::Path;

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event_rel = process_inputs
        .get("event_file_path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("event_file_path requerido")?;

    let out = route_domain_event(repo, event_rel);
    let ok = out.get("success").and_then(|v| v.as_bool()).unwrap_or(false)
        && out.get("exitCode").and_then(|v| v.as_i64()).unwrap_or(1) == 0;
    let status_code = out
        .get("exitCode")
        .and_then(|v| v.as_i64())
        .unwrap_or(if ok { 0 } else { 1 }) as i32;
    let dispatch_mode = out
        .get("data")
        .and_then(|d| d.get("dispatch_mode"))
        .and_then(|v| v.as_str())
        .map(str::to_string);

    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: out.get("data").cloned(),
        error: out
            .get("error")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        execution_report: Some(json!({
            "process_name": "route-domain-event",
            "phases": [{
                "phase_name": "Orquestación route-domain-event",
                "status": if ok { "executed" } else { "failed" },
                "handler": "route-domain-event-core-rust",
                "dispatch_mode": dispatch_mode,
            }],
        })),
        exit_code: status_code,
    })
}
