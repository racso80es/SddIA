//! Handler nativo `purge-repo-retention`: dos tiempos sobre `tool:repo-retention-purger`.

use super::capsules::invoke_tool_capsule_json;
use repo_retention_purger::{assert_candidate_jail, canonical_repo_root};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn request_from_inputs(inputs: &Value, simulate: bool) -> Value {
    let mut req = serde_json::Map::new();
    req.insert("simulate".into(), json!(simulate));
    if let Some(r) = inputs.get("repo_root").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        req.insert("repo_root".into(), json!(r));
    }
    if let Some(d) = inputs.get("plan_digest").and_then(|v| v.as_str()).filter(|s| !s.is_empty()) {
        req.insert("plan_digest".into(), json!(d));
    }
    if let Some(h) = inputs.get("older_than_hours") {
        req.insert("older_than_hours".into(), h.clone());
    }
    if let Some(s) = inputs.get("skip_live_check") {
        req.insert("skip_live_check".into(), s.clone());
    }
    json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": "repo-retention-purger"
        },
        "request": req
    })
}

fn candidate_paths(body: &Value) -> Vec<String> {
    body.get("result")
        .and_then(|r| r.get("candidates"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| {
                    let kept = c.get("kept").and_then(|v| v.as_bool()).unwrap_or(false);
                    if kept {
                        return None;
                    }
                    c.get("path").and_then(|v| v.as_str()).map(str::to_string)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn repo_root_from_dry(repo: &Path, dry: &super::capsules::CapsuleInvokeResult) -> PathBuf {
    if let Some(s) = dry
        .body
        .get("result")
        .and_then(|r| r.get("repo_root"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return PathBuf::from(s);
    }
    canonical_repo_root(repo).unwrap_or_else(|_| repo.to_path_buf())
}

pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let simulate_only = inputs
        .get("simulate_only")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let dry_payload = request_from_inputs(inputs, true);
    let dry = invoke_tool_capsule_json(repo, "repo-retention-purger", &dry_payload, false)?;
    if dry.exit_code != 0 {
        return Err(format!(
            "dry-run falló exitCode={}: {}",
            dry.exit_code,
            dry.body.get("error").and_then(|v| v.as_str()).unwrap_or("sin error")
        ));
    }
    let repo_root_path = repo_root_from_dry(repo, &dry);
    for p in candidate_paths(&dry.body) {
        assert_candidate_jail(&repo_root_path, Path::new(&p)).map_err(|e| {
            let (code, msg) = e.code_and_msg();
            format!("{code}: {msg}")
        })?;
    }
    if simulate_only {
        return Ok(json!({
            "success": true,
            "dry_run": dry.body.get("result").cloned().unwrap_or(dry.body.clone()),
            "purge": Value::Null
        }));
    }
    let digest = dry
        .body
        .get("result")
        .and_then(|r| r.get("plan_digest"))
        .cloned()
        .or_else(|| inputs.get("plan_digest").cloned());
    let mut apply_inputs = inputs.clone();
    if let Some(d) = digest {
        if let Some(obj) = apply_inputs.as_object_mut() {
            obj.insert("plan_digest".into(), d);
        }
    }
    let purge_payload = request_from_inputs(&apply_inputs, false);
    let purge = invoke_tool_capsule_json(repo, "repo-retention-purger", &purge_payload, false)?;
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
