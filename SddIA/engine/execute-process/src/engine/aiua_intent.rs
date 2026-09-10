//! Handler nativo `dispatch-aiua-intent` — tendón → ECST fractal domain.

use super::crypto_broker;
use super::ecst_validation::validate_ecst_event;
use super::eda_bus::write_fractal_event;
use super::suite_execution_requested;
use chrono::Utc;
use serde_json::{json, Value};
use std::env;
use std::path::Path;
use uuid::Uuid;

const SDLC_TENDONS: &[(&str, &str)] = &[
    ("ordenar_refactorizacion", "refactorization"),
    ("iniciar_feature", "feature"),
    ("iniciar_bug_fix", "bug-fix"),
];
const SUITE_TENDON: &str = "requerir_auditoria";
const CLARIFY_TENDON: &str = "solicitar_clarificacion";

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn env_truthy(key: &str) -> bool {
    env::var(key)
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn mint_event_id(repo: &Path) -> Result<String, String> {
    if let Ok(out) = crypto_broker::run(
        repo,
        &json!({"operation": "GENERATE_UUID", "target_payload": null}),
    ) {
        if let Some(s) = out.get("crypto_response").and_then(|v| v.as_str()) {
            if Uuid::parse_str(s).is_ok() {
                return Ok(s.to_string());
            }
        }
        if let Some(nested) = out
            .get("crypto_response")
            .and_then(|v| v.get("result"))
            .and_then(|v| v.as_str())
        {
            if Uuid::parse_str(nested).is_ok() {
                return Ok(nested.to_string());
            }
        }
    }
    Ok(Uuid::new_v4().to_string())
}

/// Extrae el primer fence `aiua-intent`.
pub fn extract_aiua_intent(text: &str) -> Option<Value> {
    let marker = "```aiua-intent";
    let start = text.find(marker)?;
    let after = &text[start + marker.len()..];
    let body = after.strip_prefix('\n').unwrap_or(after);
    let end = body.find("```")?;
    let json_text = body[..end].trim();
    if json_text.is_empty() {
        return None;
    }
    let parsed: Value = serde_json::from_str(json_text).ok()?;
    let name = parsed.get("name").and_then(|v| v.as_str()).map(str::trim)?;
    if name.is_empty() {
        return None;
    }
    let args = parsed.get("args").cloned().unwrap_or_else(|| json!({}));
    Some(json!({ "name": name, "args": args }))
}

pub fn lab_mock_overlay_intent() -> Option<Value> {
    if !env_truthy("SDDIA_LAB_MOCK_OUTBOUND") {
        return None;
    }
    let raw = env::var("SDDIA_LAB_MOCK_AIUA_INTENT").ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    serde_json::from_str(trimmed).ok()
}

pub fn resolve_intent(text: &str) -> Option<Value> {
    lab_mock_overlay_intent().or_else(|| extract_aiua_intent(text))
}

pub fn is_motor_tendon(name: &str) -> bool {
    SDLC_TENDONS.iter().any(|(n, _)| *n == name) || name == SUITE_TENDON
}

fn sdlc_process(name: &str) -> Option<&'static str> {
    SDLC_TENDONS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, p)| *p)
}

fn function_call(inputs: &Value) -> Result<(String, Value), String> {
    let fc = inputs.get("function_call").unwrap_or(inputs);
    let name = str_field(fc, "name").ok_or("function_call.name obligatorio")?;
    let args = fc.get("args").cloned().unwrap_or_else(|| json!({}));
    if !args.is_object() && !args.is_null() {
        return Err("function_call.args debe ser objeto".into());
    }
    let args = if args.is_null() { json!({}) } else { args };
    Ok((name, args))
}

fn raw_text_from_args(args: &Value) -> Result<String, String> {
    let goal = str_field(args, "goal").ok_or("args.goal obligatorio")?;
    let target = str_field(args, "target_component").ok_or("args.target_component obligatorio")?;
    Ok(format!("{goal}\n\ntarget_component: {target}"))
}

fn resolve_event_id(inputs: &Value, repo: &Path) -> Result<String, String> {
    if let Some(cid) = str_field(inputs, "correlation_id") {
        Uuid::parse_str(&cid).map_err(|_| "correlation_id no es UUID v4".to_string())?;
        return Ok(cid);
    }
    mint_event_id(repo)
}

fn emit_sdlc(
    repo: &Path,
    process: &str,
    raw_text: &str,
    pbi_ref: Option<&str>,
    intent_name: &str,
    event_id: &str,
) -> Result<Value, String> {
    let mut payload = json!({
        "process": process,
        "raw_text": raw_text,
        "intent_name": intent_name,
    });
    if let Some(pbi) = pbi_ref.filter(|s| !s.is_empty()) {
        payload["pbi_ref"] = json!(pbi);
    }
    let event = json!({
        "event_id": event_id,
        "event_type": "Aiua_Process_Requested",
        "event_family": "domain",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "aiua-stimulus-processing",
        "correlation_id": event_id,
        "payload": payload,
        "delivery_state": {},
    });
    validate_ecst_event(repo, &event)?;
    let seal = write_fractal_event(repo, &event, "domain")?;
    Ok(json!({
        "success": true,
        "event_id": event_id,
        "correlation_id": event_id,
        "target_path": seal.get("target_path"),
        "event_type": "Aiua_Process_Requested",
        "intent_dispatched": intent_name,
    }))
}

/// Ejecuta `dispatch-aiua-intent`.
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let (name, args) = function_call(inputs)?;
    if name == CLARIFY_TENDON {
        return Err("solicitar_clarificacion no es motor".into());
    }
    if name == SUITE_TENDON {
        let suite_id = str_field(&args, "suite_id").ok_or("args.suite_id obligatorio")?;
        let mut suite_inputs = json!({ "suite_id": suite_id });
        if let Some(cid) = str_field(inputs, "correlation_id") {
            suite_inputs["correlation_id"] = json!(cid);
        }
        let out = suite_execution_requested::run(repo, &suite_inputs)?;
        return Ok(json!({
            "success": true,
            "event_id": out.get("event_id"),
            "target_path": out.get("target_path"),
            "event_type": "Suite_Execution_Requested",
            "intent_dispatched": name,
        }));
    }
    let process = sdlc_process(&name).ok_or_else(|| format!("tendón desconocido: {name}"))?;
    let raw_text = raw_text_from_args(&args)?;
    let pbi_ref = str_field(&args, "pbi_ref");
    let event_id = resolve_event_id(inputs, repo)?;
    emit_sdlc(
        repo,
        process,
        &raw_text,
        pbi_ref.as_deref(),
        &name,
        &event_id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::fs;

    #[test]
    fn extract_first_fence() {
        let text = "laudo\n```aiua-intent\n{\"name\":\"iniciar_feature\",\"args\":{\"goal\":\"g\",\"target_component\":\"c\"}}\n```\nmas";
        let parsed = extract_aiua_intent(text).unwrap();
        assert_eq!(parsed["name"], "iniciar_feature");
        assert_eq!(parsed["args"]["goal"], "g");
    }

    #[test]
    fn extract_rejects_broken_json_and_unknown_is_still_parseable() {
        assert!(extract_aiua_intent("```aiua-intent\n{no\n```").is_none());
        let parsed = extract_aiua_intent(
            "```aiua-intent\n{\"name\":\"no_existe\",\"args\":{}}\n```",
        )
        .unwrap();
        assert_eq!(parsed["name"], "no_existe");
        assert!(!is_motor_tendon("no_existe"));
        assert!(!is_motor_tendon(CLARIFY_TENDON));
        assert!(is_motor_tendon("ordenar_refactorizacion"));
    }

    #[test]
    fn extract_first_of_two_fences() {
        let text = "```aiua-intent\n{\"name\":\"iniciar_bug_fix\",\"args\":{\"goal\":\"a\",\"target_component\":\"b\"}}\n```\n```aiua-intent\n{\"name\":\"iniciar_feature\",\"args\":{\"goal\":\"x\",\"target_component\":\"y\"}}\n```";
        assert_eq!(extract_aiua_intent(text).unwrap()["name"], "iniciar_bug_fix");
    }

    #[test]
    fn dispatch_sdlc_writes_domain_not_pending() {
        let repo = find_repo_root().unwrap();
        let cid = Uuid::new_v4().to_string();
        let out = run(
            &repo,
            &json!({
                "function_call": {
                    "name": "ordenar_refactorizacion",
                    "args": {"goal": "secar deuda", "target_component": "engine"}
                },
                "correlation_id": cid
            }),
        )
        .expect("dispatch");
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out["event_id"], cid);
        assert_eq!(out["event_type"], "Aiua_Process_Requested");
        let target = out["target_path"].as_str().unwrap();
        assert!(target.contains(".events/domain/"), "{target}");
        assert!(!target.contains("pending"));
        let path = repo.join(target);
        let body: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(body["emitter_agent"], "aiua-stimulus-processing");
        assert_eq!(body["payload"]["process"], "refactorization");
        assert_eq!(body["correlation_id"], cid);
        let _ = fs::remove_file(path);
        let pending = repo.join(".events/pending").join(format!("{cid}.json"));
        assert!(!pending.is_file());
    }

    #[test]
    fn dispatch_rejects_unknown_and_incomplete() {
        let repo = find_repo_root().unwrap();
        assert!(run(
            &repo,
            &json!({"function_call": {"name": "no_existe", "args": {}}})
        )
        .is_err());
        assert!(run(
            &repo,
            &json!({
                "function_call": {
                    "name": "iniciar_feature",
                    "args": {"goal": "solo"}
                }
            })
        )
        .is_err());
        assert!(run(
            &repo,
            &json!({"function_call": {"name": "solicitar_clarificacion", "args": {}}})
        )
        .is_err());
    }

    #[test]
    fn dispatch_suite_missing_id_fails() {
        let repo = find_repo_root().unwrap();
        assert!(run(
            &repo,
            &json!({"function_call": {"name": "requerir_auditoria", "args": {}}})
        )
        .is_err());
        assert!(run(
            &repo,
            &json!({
                "function_call": {
                    "name": "requerir_auditoria",
                    "args": {"suite_id": "nonexistent-suite-xyz"}
                }
            })
        )
        .is_err());
    }
}
