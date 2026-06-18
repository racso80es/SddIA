//! Handler nativo `emit-suite-execution-requested` (D-P6T.1).

use super::crypto_broker;
use super::ecst_validation::validate_ecst_event;
use super::eda_bus::write_fractal_event;
use chrono::Utc;
use serde_json::{json, Value};
use std::path::Path;
use uuid::Uuid;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn optional_str(v: &Value, key: &str) -> Option<String> {
    match v.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.trim().is_empty() => None,
        Some(Value::String(s)) => Some(s.trim().to_string()),
        _ => None,
    }
}

fn mint_event_id(repo: &Path) -> Result<String, String> {
    let out = crypto_broker::run(
        repo,
        &json!({"operation": "GENERATE_UUID", "target_payload": null}),
    )?;
    if let Some(s) = out.get("crypto_response").and_then(|v| v.as_str()) {
        return Ok(s.to_string());
    }
    if let Some(nested) = out
        .get("crypto_response")
        .and_then(|v| v.get("result"))
        .and_then(|v| v.as_str())
    {
        return Ok(nested.to_string());
    }
    Ok(Uuid::new_v4().to_string())
}

/// Ejecuta `emit-suite-execution-requested` (paridad `execute-action.py`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let suite_id = str_field(inputs, "suite_id").ok_or("suite_id es obligatorio (string)")?;
    let suite_path = repo.join("SddIA/suites").join(format!("{suite_id}.md"));
    if !suite_path.is_file() {
        return Err(format!("Suite no encontrada: {suite_id}"));
    }

    let event_id = mint_event_id(repo)?;
    let mut payload = json!({"suite_id": suite_id});
    if let Some(asset_id) = optional_str(inputs, "asset_id") {
        payload["asset_id"] = json!(asset_id);
    }
    if let Some(strategy) = optional_str(inputs, "execution_strategy") {
        if strategy == "fail_fast" || strategy == "run_all" {
            payload["execution_strategy"] = json!(strategy);
        }
    }

    let mut event = json!({
        "event_id": event_id,
        "event_type": "Suite_Execution_Requested",
        "event_family": "domain",
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "emitter_agent": "emit-suite-execution-requested",
        "payload": payload,
        "delivery_state": {},
    });
    if let Some(corr) = optional_str(inputs, "correlation_id") {
        event["correlation_id"] = json!(corr);
    }

    validate_ecst_event(repo, &event)?;
    let seal = write_fractal_event(repo, &event, "domain")?;

    Ok(json!({
        "success": true,
        "event_id": seal.get("event_id"),
        "target_path": seal.get("target_path"),
        "event_type": "Suite_Execution_Requested",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::fs;

    #[test]
    fn emit_suite_execution_requested_writes_domain_event() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({"suite_id": "fail-fast-lab"});
        let result = run(&repo, &inputs).expect("emit suite");
        assert_eq!(result.get("success"), Some(&json!(true)));
        let target = result.get("target_path").and_then(|v| v.as_str()).unwrap();
        let path = repo.join(target);
        assert!(path.is_file(), "domain event missing: {target}");
        let body: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(
            body.get("event_type").and_then(|v| v.as_str()),
            Some("Suite_Execution_Requested")
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn emit_suite_execution_requested_rejects_missing_suite() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({"suite_id": "nonexistent-suite-xyz"});
        assert!(run(&repo, &inputs).is_err());
    }
}
