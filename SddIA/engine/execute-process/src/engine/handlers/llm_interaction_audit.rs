//! Handler nativo `llm-interaction-audit` — append de filas y fricción bajo `paths.auditsPath`.

use super::super::llm_interaction_audit::{
    append_friction, append_interaction, fetch_llm_audit_active, resolve_tracker_ref, FrictionRow,
    InteractionRow,
};
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

fn str_field(v: &Value, key: &str) -> Result<String, String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("input requerido: {key}"))
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let op = inputs
        .get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("record_interaction");
    let result: Result<Value, String> = (|| {
        let tracker_ref = resolve_tracker_ref(inputs).ok_or_else(|| "tracker_ref requerido".to_string())?;
        match op {
            "record_interaction" => {
                let row = InteractionRow {
                    timestamp: str_field(inputs, "timestamp")?,
                    rol: str_field(inputs, "rol")?,
                    tipo: str_field(inputs, "tipo")?,
                    resumen: str_field(inputs, "resumen")?,
                };
                append_interaction(repo, &tracker_ref, &row, true)
            }
            "record_friction" => {
                let friction = FrictionRow {
                    comando: str_field(inputs, "comando")?,
                    success: inputs
                        .get("success")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    error_code: str_field(inputs, "error_code")?,
                    handler: str_field(inputs, "handler")?,
                    mensaje: inputs
                        .get("mensaje")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                };
                append_friction(repo, &tracker_ref, &friction, true)
            }
            "probe_active" => {
                let active = fetch_llm_audit_active(repo, &tracker_ref)?;
                Ok(json!({"tracker_ref": tracker_ref, "llm_audit_active": active}))
            }
            other => Err(format!("operation no soportada: {other}")),
        }
    })();

    match result {
        Ok(data) => Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            exit_code: 0,
            data: Some(data),
            error: None,
            execution_report: Some(json!({
                "process_name": "llm-interaction-audit",
                "phases": [{
                    "phase_name": "Registro",
                    "status": "executed",
                    "handler": "llm-interaction-audit-native",
                }],
            })),
        }),
        Err(e) => Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            exit_code: 1,
            data: Some(json!({"error_code": "LLM_AUDIT_FAILED"})),
            error: Some(e),
            execution_report: Some(json!({
                "process_name": "llm-interaction-audit",
                "phases": [{
                    "phase_name": "Registro",
                    "status": "failed",
                    "handler": "llm-interaction-audit-native",
                }],
            })),
        }),
    }
}
