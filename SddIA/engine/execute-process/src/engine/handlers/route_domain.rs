//! Handler `route-domain-event` (P4) — core EDA nativo Rust.

use crate::envelope::OrchestratorEnvelope;
use crate::engine::route_domain_core::{
    input_truthy, resolve_route_event_path, route_domain_event, SyncRouteGuard,
};
use serde_json::{json, Value};
use std::path::Path;

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let blocking =
        input_truthy(process_inputs, "blocking") || input_truthy(process_inputs, "sync");
    let mut paths = vec![];
    if let Some(arr) = process_inputs.get("event_file_paths").and_then(|v| v.as_array()) {
        paths = arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
    } else {
        let event_rel = resolve_route_event_path(repo, process_inputs, blocking)?;
        paths.push(event_rel);
    }
    let _sync_guard = if blocking {
        Some(SyncRouteGuard::activate())
    } else {
        None
    };

    let out = crate::engine::route_domain_core::route_domain_batch(repo, paths.clone());
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

    let mut final_data = out.get("data").cloned();
    if paths.len() == 1 {
        if let Some(arr) = final_data.as_ref().and_then(|v| v.as_array()) {
            if let Some(first) = arr.first() {
                final_data = first.get("data").cloned();
            }
        }
    }

    Ok(OrchestratorEnvelope {
        success: ok,
        status_code,
        data: final_data,
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
                "blocking": blocking,
            }],
        })),
        exit_code: status_code,
    })
}
