use super::mayeuta::synthesize_mayeuta_response;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};

pub fn run(process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let prompt = process_inputs
        .get("prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "prompt requerido".to_string())?;

    if prompt.is_empty() {
        return Ok(OrchestratorEnvelope::failure("prompt vacío", 1));
    }

    let response = synthesize_mayeuta_response(prompt);
    let ok = true;
    Ok(OrchestratorEnvelope {
        success: ok,
        status_code: 0,
        data: Some(json!({
            "ok": ok,
            "response": response,
            "error": null,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "kalma2-interact",
            "phases": [
                {
                    "phase_name": "Síntesis",
                    "status": "executed",
                    "handler": "kalma2-interact-core",
                },
                {
                    "phase_name": "Respuesta",
                    "status": "executed",
                    "handler": "kalma2-interact-core",
                    "response_preview": response.chars().take(80).collect::<String>(),
                }
            ]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn kalma2_returns_two_line_response() {
        let env = run(&json!({"prompt": "ping"})).unwrap();
        assert!(env.success);
        let resp = env.data.as_ref().unwrap()["response"].as_str().unwrap();
        assert!(resp.contains("Tormentosa/Aiúa"));
        assert!(resp.contains('\n'));
    }
}
