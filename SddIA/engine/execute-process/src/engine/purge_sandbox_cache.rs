//! Handler nativo `purge-sandbox-cache`: dos tiempos sobre `tool:ephemeral-cache-purger`.

use super::capsules::invoke_tool_capsule_json;
use regex::Regex;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::OnceLock;

pub const ALLOW_RE: &str = r"^/tmp/cursor-sandbox-cache(/[a-f0-9]{16,64}(/.*)?)?$";

fn allow_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(ALLOW_RE).expect("ALLOW_RE"))
}

pub fn assert_candidates_in_jail(paths: &[String]) -> Result<(), String> {
    for p in paths {
        let t = p.trim_end_matches('/');
        if !allow_re().is_match(t) && !allow_re().is_match(p) {
            return Err(format!(
                "SECURITY_VIOLATION_PATH_OUT_OF_BOUNDS: candidato fuera de jail: {p}"
            ));
        }
    }
    Ok(())
}

fn request_from_inputs(inputs: &Value, simulate: bool) -> Value {
    let mut req = serde_json::Map::new();
    req.insert("simulate".into(), json!(simulate));
    if let Some(td) = inputs.get("target_dir").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        req.insert("target_dir".into(), json!(td));
    }
    if let Some(h) = inputs.get("older_than_hours") {
        req.insert("older_than_hours".into(), h.clone());
    }
    if let Some(p) = inputs.get("purge_sandbox_root") {
        req.insert("purge_sandbox_root".into(), p.clone());
    }
    json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": "ephemeral-cache-purger"
        },
        "request": req
    })
}

fn candidate_list(body: &Value) -> Vec<String> {
    body.get("result")
        .and_then(|r| r.get("candidate_targets"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let simulate_only = inputs
        .get("simulate_only")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dry_payload = request_from_inputs(inputs, true);
    let dry = invoke_tool_capsule_json(repo, "ephemeral-cache-purger", &dry_payload, false)?;
    if dry.exit_code != 0 {
        return Err(format!(
            "dry-run falló exitCode={}: {}",
            dry.exit_code,
            dry.body.get("error").and_then(|v| v.as_str()).unwrap_or("sin error")
        ));
    }
    let candidates = candidate_list(&dry.body);
    assert_candidates_in_jail(&candidates)?;
    if simulate_only {
        return Ok(json!({
            "success": true,
            "dry_run": dry.body.get("result").cloned().unwrap_or(dry.body.clone()),
            "purge": Value::Null
        }));
    }
    let purge_payload = request_from_inputs(inputs, false);
    let purge = invoke_tool_capsule_json(repo, "ephemeral-cache-purger", &purge_payload, false)?;
    if purge.exit_code != 0 {
        return Err(format!(
            "purga falló exitCode={}: {}",
            purge.exit_code,
            purge.body.get("error").and_then(|v| v.as_str()).unwrap_or("sin error")
        ));
    }
    Ok(json!({
        "success": true,
        "dry_run": dry.body.get("result").cloned().unwrap_or(dry.body.clone()),
        "purge": purge.body.get("result").cloned().unwrap_or(purge.body.clone())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_accepts_cargo_target() {
        let ok = vec![
            "/tmp/cursor-sandbox-cache/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee/cargo-target".into(),
        ];
        assert!(assert_candidates_in_jail(&ok).is_ok());
    }

    #[test]
    fn gate_rejects_home() {
        let bad = vec!["/home/user/.cache".into()];
        assert!(assert_candidates_in_jail(&bad).is_err());
    }
}
