//! Handlers nativos Espejo de Consciencia.

use crate::envelope::OrchestratorEnvelope;
use sddia_ecosystem_health::{compile_map_snapshot, fuse_ecosystem_health, FuseOptions};
use serde_json::{json, Value};
use std::path::Path;

fn input_bool(inputs: &Value, key: &str, default: bool) -> bool {
    inputs.get(key).map_or(default, |v| {
        v.as_bool().unwrap_or_else(|| {
            v.as_str()
                .map(|s| matches!(s.trim().to_lowercase().as_str(), "1" | "true" | "yes"))
                .unwrap_or(default)
        })
    })
}

pub fn run_compile_map(repo: &Path, _inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let result = compile_map_snapshot(repo)?;
    let ok = result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code: if ok { 0 } else { 1 },
        data: Some(result.clone()),
        error: None,
        execution_report: Some(json!({
            "process_name": "compile-ecosystem-map-snapshot",
            "phases": [{
                "phase_name": "Compilar mapa",
                "status": if ok { "executed" } else { "failed" },
                "handler": "ecosystem-health-core",
                "result": result,
            }],
        })),
        exit_code: if ok { 0 } else { 1 },
    })
}

pub fn run_query(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let opts = FuseOptions {
        compile_map: input_bool(inputs, "compile_map", false),
        persist: input_bool(inputs, "persist", true),
    };
    let result = fuse_ecosystem_health(repo, opts)?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(result.clone()),
        error: None,
        execution_report: Some(json!({
            "process_name": "query-ecosystem-health",
            "phases": [{
                "phase_name": "Fusionar salud",
                "status": "executed",
                "handler": "ecosystem-health-core",
                "result": result,
            }],
        })),
        exit_code: 0,
    })
}
