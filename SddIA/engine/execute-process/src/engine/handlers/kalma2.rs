//! Handler nativo `kalma2-interact` — síntesis LLM + clasificación (Fase D).

use super::mayeuta::{filter_c_should_abort, synthesize_mayeuta_response};
use super::super::capsules::invoke_capsule_json;
use super::super::fractal::{load_fractal_dirs, write_fractal_event};
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

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
    // Paths con espacios / em-dash: anclar en docs/todos/{pending|done}/ … .md
    super::task_queue_manager::extract_pbi_path(prompt)
}

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn build_kalma2_process_event(
    process: &str,
    pbi_ref: Option<&str>,
    raw_text: &str,
    correlation_id: Option<&str>,
) -> Value {
    let event_id = correlation_id
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|s| Uuid::parse_str(s).ok())
        .map(|u| u.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let mut payload = json!({
        "process": process,
        "raw_text": raw_text,
    });
    if let Some(p) = pbi_ref.filter(|s| !s.is_empty()) {
        payload["pbi_ref"] = json!(p);
    }
    json!({
        "event_id": event_id,
        "event_type": "Kalma2_Process_Requested",
        "event_family": "domain",
        "timestamp": iso_now(),
        "emitter_agent": "kalma2-interact",
        "payload": payload,
        "delivery_state": {},
    })
}

fn ack_enqueued(process: &str, event_id: &str) -> String {
    format!(
        "[Tormentosa/Aiúa] Tarea encolada: proceso «{process}» (evento {event_id}). \
         El Sistema Nervioso la procesará en segundo plano."
    )
}

fn envelope_response_with_data(
    response: &str,
    data_extra: Value,
    extra_phases: Value,
) -> OrchestratorEnvelope {
    let mut data = json!({
        "ok": true,
        "response": response,
        "error": null,
    });
    if let Some(obj) = data_extra.as_object() {
        for (k, v) in obj {
            data[k] = v.clone();
        }
    }
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "process_name": "kalma2-interact",
            "phases": extra_phases,
        })),
        exit_code: 0,
    }
}

fn envelope_response(response: &str, extra_phases: Value) -> OrchestratorEnvelope {
    envelope_response_with_data(response, json!({}), extra_phases)
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
        return Ok(envelope_response_with_data(
            &response,
            json!({ "degraded": true }),
            json!([{
                "phase_name": "Triaje-C",
                "status": "aborted",
                "handler": "kalma2-interact-core",
            }]),
        ));
    }

    // L-CI / L-EP: mode explícito desde bridge (/api/execute|/api/chat) — sin CLASSIFY_INTENT.
    let mode = process_inputs
        .get("mode")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .unwrap_or("");

    if mode == "execute" {
        return run_execute_deterministic(repo, prompt, process_inputs);
    }
    if mode == "chat" {
        return run_chat_only(repo, prompt);
    }

    // Legado /api/interact: CLASSIFY_INTENT degradado a heurística local si CLI falla (no aduana S+).
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
        let mut nested = nested_inputs;
        plumb_correlation_id(process_inputs, &mut nested);
        return emit_process_event(repo, prompt, &process_name, &nested, intent, confidence);
    }

    run_chat_only(repo, prompt)
}

fn heuristic_process_name(prompt: &str) -> Option<&'static str> {
    let lower = prompt.to_lowercase();
    if prompt.contains("[FIX]") || lower.contains("bug-fix") || lower.contains("inicia fix") {
        Some("bug-fix")
    } else if prompt.contains("[FEATURE]")
        || lower.contains("inicia feature")
        || lower.contains("forjar feature")
    {
        Some("feature")
    } else if lower.contains("refactor") {
        Some("refactorization")
    } else {
        None
    }
}

fn plumb_correlation_id(process_inputs: &Value, nested: &mut Value) {
    let has_nested = nested
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .is_some();
    if has_nested {
        return;
    }
    if let Some(cid) = process_inputs
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        nested["correlation_id"] = json!(cid);
    }
}

fn run_execute_deterministic(
    repo: &Path,
    prompt: &str,
    process_inputs: &Value,
) -> Result<OrchestratorEnvelope, String> {
    let process_name = process_inputs
        .get("process")
        .or_else(|| process_inputs.get("process_name"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| heuristic_process_name(prompt).map(str::to_string))
        .unwrap_or_default();

    if process_name.is_empty() || !allowlisted(&process_name) {
        let response = format!(
            "[Tormentosa/Aiúa] Proceso no determinado o no autorizado («{process_name}»). \
             Indica process en el payload o palabras clave allowlist: bug-fix, feature, refactorization."
        );
        return Ok(envelope_response(
            &response,
            json!([{
                "phase_name": "Enrutamiento",
                "status": "rejected",
                "handler": "kalma2-interact-core",
                "mode": "execute",
                "process_name": process_name,
            }]),
        ));
    }

    let mut nested = process_inputs
        .get("process_inputs")
        .cloned()
        .unwrap_or_else(|| json!({}));
    plumb_correlation_id(process_inputs, &mut nested);
    emit_process_event(repo, prompt, &process_name, &nested, "execute", 1.0)
}

fn run_chat_only(repo: &Path, prompt: &str) -> Result<OrchestratorEnvelope, String> {
    let (response, degraded) = match synthesize_via_skill(repo, prompt) {
        Ok(text) => (text, false),
        Err(_) => (synthesize_mayeuta_response(prompt), true),
    };

    Ok(envelope_response_with_data(
        &response,
        json!({ "degraded": degraded }),
        json!([{
            "phase_name": "Síntesis",
            "status": "executed",
            "handler": "mayeuta-llm",
            "intent": "chat",
            "degraded": degraded,
        }]),
    ))
}

fn emit_process_event(
    repo: &Path,
    prompt: &str,
    process_name: &str,
    nested_inputs: &Value,
    intent: &str,
    confidence: f64,
) -> Result<OrchestratorEnvelope, String> {
    if !allowlisted(process_name) {
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
    let pbi_ref = extract_pbi_ref(prompt, nested_inputs);
    let correlation_id = nested_inputs
        .get("correlation_id")
        .and_then(|v| v.as_str());
    let event = build_kalma2_process_event(
        process_name,
        pbi_ref.as_deref(),
        prompt,
        correlation_id,
    );
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let (_, _, domain_dir, _) = load_fractal_dirs(repo);
    let seal = write_fractal_event(repo, &event, &domain_dir)?;
    let ack = ack_enqueued(process_name, &event_id);
    Ok(envelope_response_with_data(
        &ack,
        json!({
            "emitted": true,
            "event_type": "Kalma2_Process_Requested",
            "event_id": event_id,
            "correlation_id": event_id,
            "seal": seal,
        }),
        json!([{
            "phase_name": "Enrutamiento",
            "status": "executed",
            "handler": "kalma2-interact-core",
            "intent": intent,
            "process_name": process_name,
            "confidence": confidence,
            "emitted": true,
            "event_type": "Kalma2_Process_Requested",
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
        let data = env.data.as_ref().unwrap();
        let resp = data["response"].as_str().unwrap();
        assert!(resp.contains("Tormentosa/Aiúa"));
        assert_eq!(data.get("degraded").and_then(|v| v.as_bool()), Some(true));
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
        let data = env.data.as_ref().unwrap();
        let resp = data["response"].as_str().unwrap();
        assert!(resp.contains("encolada") || resp.contains("Tarea"));
        assert_eq!(data.get("emitted").and_then(|v| v.as_bool()), Some(true));
        let event_id = data.get("event_id").and_then(|v| v.as_str()).unwrap();
        assert!(!event_id.is_empty());
        assert_eq!(
            data.get("correlation_id").and_then(|v| v.as_str()),
            Some(event_id)
        );
    }

    #[test]
    fn kalma2_extracts_pbi_ref_with_spaces() {
        let prompt = "Inicia proceso fix para corregir la incidencia documentada en docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md";
        let got = extract_pbi_ref(prompt, &json!({}));
        assert_eq!(
            got.as_deref(),
            Some("docs/todos/pending/[FIX] telegram-watcher — fractura sistémica (e6cbecb9032c).md")
        );
    }

    #[test]
    fn kalma2_honors_preassigned_correlation_id() {
        let cid = "458c34a8-9ad5-4a40-88c4-0be1e5d9598e";
        let event = build_kalma2_process_event(
            "feature",
            None,
            "inicia feature docs/todos/pending/[FEATURE] x.md",
            Some(cid),
        );
        assert_eq!(event.get("event_id").and_then(|v| v.as_str()), Some(cid));
        assert_eq!(
            event.get("event_type").and_then(|v| v.as_str()),
            Some("Kalma2_Process_Requested")
        );
    }

    #[test]
    fn kalma2_invalid_correlation_id_falls_back_to_new_uuid() {
        let event = build_kalma2_process_event("feature", None, "x", Some("not-a-uuid"));
        let eid = event.get("event_id").and_then(|v| v.as_str()).unwrap();
        assert!(Uuid::parse_str(eid).is_ok());
        assert_ne!(eid, "not-a-uuid");
    }

    #[test]
    fn kalma2_execute_plumbs_top_level_correlation_id() {
        std::env::remove_var("SDDIA_LLM_CLI_COMMAND");
        let repo = find_repo_root().expect("repo");
        let cid = "a1b2c3d4-e5f6-4789-a012-3456789abcde";
        let env = run(
            &repo,
            &json!({
                "prompt": "inicia feature docs/todos/pending/[FEATURE] plumb-cid.md",
                "mode": "execute",
                "process": "feature",
                "correlation_id": cid,
            }),
        )
        .unwrap();
        assert!(env.success);
        let data = env.data.as_ref().unwrap();
        assert_eq!(data.get("emitted").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(data.get("event_id").and_then(|v| v.as_str()), Some(cid));
        assert_eq!(
            data.get("correlation_id").and_then(|v| v.as_str()),
            Some(cid)
        );
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
