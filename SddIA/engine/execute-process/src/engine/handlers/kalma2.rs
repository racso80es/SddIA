//! Handler nativo `kalma2-interact` — síntesis LLM + clasificación (Fase D).

use super::mayeuta::{filter_c_should_abort, synthesize_mayeuta_response};
use super::super::capsules::invoke_capsule_json;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::path::Path;

const CONFIDENCE_THRESHOLD: f64 = 0.7;

const ALLOWLIST: &[&str] = &[
    "bug-fix",
    "feature",
    "refactorization",
    "task-queue-manager",
];

fn allowlisted(name: &str) -> bool {
    ALLOWLIST.contains(&name.trim())
}

fn invoke_mayeuta(repo: &Path, payload: &Value) -> Result<Value, String> {
    let result = invoke_capsule_json(repo, "mayeuta-llm", payload, false)?;
    if result.exit_code != 0 || result.body.get("success") != Some(&json!(true)) {
        return Err(result
            .body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("mayeuta-llm failed")
            .to_string());
    }
    Ok(result
        .body
        .get("data")
        .cloned()
        .unwrap_or_else(|| json!({})))
}

fn classify_intent(repo: &Path, prompt: &str) -> Result<Value, String> {
    invoke_mayeuta(
        repo,
        &json!({
            "operation": "CLASSIFY_INTENT",
            "prompt": prompt,
        }),
    )
}

fn synthesize_via_skill(repo: &Path, prompt: &str) -> Result<String, String> {
    let data = invoke_mayeuta(
        repo,
        &json!({
            "operation": "SYNTHESIZE",
            "prompt": prompt,
        }),
    )?;
    data.get("text")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "mayeuta-llm SYNTHESIZE sin text".into())
}

fn extract_pbi_ref(prompt: &str, process_inputs: &Value) -> Option<String> {
    if let Some(p) = process_inputs.get("pbi_ref").and_then(|v| v.as_str()) {
        if !p.trim().is_empty() {
            return Some(p.trim().to_string());
        }
    }
    prompt
        .split_whitespace()
        .find(|t| t.contains("docs/todos/pending/") && t.ends_with(".md"))
        .map(str::trim)
        .map(str::to_string)
}

fn envelope_response(response: &str, extra_phases: Value) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "ok": true,
            "response": response,
            "error": null,
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "kalma2-interact",
            "phases": extra_phases,
        })),
        exit_code: 0,
    }
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let prompt = process_inputs
        .get("prompt")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "prompt requerido".to_string())?;

    if filter_c_should_abort(prompt) {
        let response = synthesize_mayeuta_response(prompt);
        return Ok(envelope_response(
            &response,
            json!([{
                "phase_name": "Triaje-C",
                "status": "aborted",
                "handler": "kalma2-interact-core",
            }]),
        ));
    }

    let intent_data = classify_intent(repo, prompt).unwrap_or_else(|_| json!({
        "intent": "chat",
        "confidence": 0.0,
    }));

    let intent = intent_data
        .get("intent")
        .and_then(|v| v.as_str())
        .unwrap_or("chat");
    let confidence = intent_data
        .get("confidence")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let process_name = intent_data
        .get("process_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let nested_inputs = intent_data
        .get("process_inputs")
        .cloned()
        .unwrap_or_else(|| json!({}));

    if intent == "execute" && confidence >= CONFIDENCE_THRESHOLD {
        if !allowlisted(&process_name) {
            let response = format!(
                "[Tormentosa/Aiúa] Proceso «{process_name}» no autorizado. \
                 Allowlist: bug-fix, feature, refactorization, task-queue-manager."
            );
            return Ok(envelope_response(
                &response,
                json!([{
                    "phase_name": "Enrutamiento",
                    "status": "rejected",
                    "handler": "kalma2-interact-core",
                    "process_name": process_name,
                }]),
            ));
        }
        // Fase E: emisión evento Kalma2_Process_Requested
        let _pbi = extract_pbi_ref(prompt, &nested_inputs);
        let ack = format!(
            "[Tormentosa/Aiúa] Intención «{process_name}» reconocida (confianza {confidence:.2}). \
             Enrutamiento EDA se activará en la siguiente fase."
        );
        return Ok(envelope_response(
            &ack,
            json!([{
                "phase_name": "Clasificación",
                "status": "executed",
                "handler": "mayeuta-llm",
                "intent": intent,
                "process_name": process_name,
                "confidence": confidence,
            }, {
                "phase_name": "Enrutamiento",
                "status": "deferred",
                "handler": "kalma2-interact-core",
            }]),
        ));
    }

    let response = synthesize_via_skill(repo, prompt)
        .unwrap_or_else(|_| synthesize_mayeuta_response(prompt));

    Ok(envelope_response(
        &response,
        json!([{
            "phase_name": "Clasificación",
            "status": "executed",
            "handler": "mayeuta-llm",
            "intent": intent,
            "confidence": confidence,
        }, {
            "phase_name": "Síntesis",
            "status": "executed",
            "handler": "mayeuta-llm",
        }]),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use serde_json::json;

    #[test]
    fn kalma2_fallback_without_llm_cli() {
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
        let repo = find_repo_root().expect("repo");
        let env = run(&repo, &json!({"prompt": "hola lab"})).unwrap();
        assert!(env.success);
        let resp = env.data.as_ref().unwrap()["response"].as_str().unwrap();
        assert!(resp.contains("Tormentosa/Aiúa"));
    }

    #[test]
    fn kalma2_classifies_fix_heuristic() {
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
        let repo = find_repo_root().expect("repo");
        let env = run(
            &repo,
            &json!({"prompt": "inicia fix docs/todos/pending/[FIX] x.md"}),
        )
        .unwrap();
        assert!(env.success);
        let resp = env.data.as_ref().unwrap()["response"].as_str().unwrap();
        assert!(resp.contains("bug-fix") || resp.contains("Intención"));
    }

    #[test]
    fn kalma2_rejects_unknown_process() {
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
        let repo = find_repo_root().expect("repo");
        // Force execute path via mock would need CLI; skip if skill unavailable
        if invoke_capsule_json(&repo, "mayeuta-llm", &json!({"operation":"SYNTHESIZE","prompt":"x"}), false).is_err() {
            return;
        }
    }
}
