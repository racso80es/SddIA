//! Handler nativo `policy-validator` — dictamen Cerbero vs `execution-contexts.md` (D-P6T.1).

use super::workspace::load_paths_config;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const EXECUTION_CONTEXTS_FILE: &str = "execution-contexts.md";

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn resolve_normative_ref(repo: &Path) -> Result<(String, PathBuf), String> {
    let cfg = load_paths_config(repo)?;
    let norms_rel = cfg
        .get("directories")
        .and_then(|d| d.get("norms"))
        .and_then(|v| v.as_str())
        .ok_or("directories.norms ausente en cumulo.paths.json")?;
    let norms_dir = repo.join(norms_rel.trim().trim_matches('/'));
    let norm_path = norms_dir.join(EXECUTION_CONTEXTS_FILE);
    if !norm_path.is_file() {
        return Err(format!(
            "execution-contexts.md inaccesible: {}",
            norm_path.display()
        ));
    }
    let normative_ref = format!(
        "{}/{}",
        norms_rel.trim().trim_matches('/'),
        EXECUTION_CONTEXTS_FILE
    );
    Ok((normative_ref, norm_path))
}

/// Extrae identificadores de contexto desde encabezados `### 2.N. \`context-id\``.
pub fn parse_allowed_contexts(markdown: &str) -> Vec<String> {
    let mut contexts = Vec::new();
    for line in markdown.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("### 2.") {
            continue;
        }
        let Some(backtick_start) = trimmed.find('`') else {
            continue;
        };
        let rest = &trimmed[backtick_start + 1..];
        let Some(backtick_end) = rest.find('`') else {
            continue;
        };
        let ctx = rest[..backtick_end].trim();
        if !ctx.is_empty() {
            contexts.push(ctx.to_string());
        }
    }
    contexts
}

fn load_allowed_contexts(repo: &Path) -> Result<(Vec<String>, String), String> {
    let (normative_ref, path) = resolve_normative_ref(repo)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let contexts = parse_allowed_contexts(&text);
    if contexts.is_empty() {
        return Err("matriz de contextos vacía en execution-contexts.md".into());
    }
    Ok((contexts, normative_ref))
}

fn profile_includes(profile: &str, flag: &str) -> bool {
    profile == flag || profile == "BOTH"
}

fn validate_required_secrets(secrets: &Value, findings: &mut Vec<String>) {
    let Some(arr) = secrets.as_array() else {
        findings.push("required_secrets debe ser array".into());
        return;
    };
    if arr.is_empty() {
        findings.push("required_secrets no puede estar vacío si se declara".into());
        return;
    }
    let mut seen = HashSet::new();
    for item in arr {
        let Some(name) = item.as_str().map(str::trim).filter(|s| !s.is_empty()) else {
            findings.push("required_secrets: cada elemento debe ser string no vacío".into());
            continue;
        };
        if !seen.insert(name.to_string()) {
            findings.push(format!("required_secrets: duplicado '{name}'"));
        }
    }
}

fn validate_agent_policies(
    inputs: &Value,
    allowed: &HashSet<String>,
    findings: &mut Vec<String>,
    checked: &mut Vec<String>,
) {
    let Some(policies) = inputs.get("allowed_policies") else {
        findings.push("allowed_policies es obligatorio para perfil AGENT_POLICIES".into());
        return;
    };
    let Some(arr) = policies.as_array() else {
        findings.push("allowed_policies debe ser array".into());
        return;
    };
    if arr.is_empty() {
        findings.push("allowed_policies no puede estar vacío".into());
        return;
    }
    for item in arr {
        let Some(policy) = item.as_str().map(str::trim).filter(|s| !s.is_empty()) else {
            findings.push("allowed_policies: elemento inválido (string requerido)".into());
            continue;
        };
        checked.push(policy.to_string());
        if !allowed.contains(policy) {
            findings.push(format!(
                "allowed_policies: contexto no catalogado '{policy}'"
            ));
        }
    }
}

fn validate_tool_domain(
    inputs: &Value,
    allowed: &HashSet<String>,
    findings: &mut Vec<String>,
    checked: &mut Vec<String>,
) {
    let Some(ctx) = str_field(inputs, "tool_context") else {
        findings.push("tool_context es obligatorio para perfil TOOL_DOMAIN".into());
        return;
    };
    checked.push(ctx.clone());
    if !allowed.contains(&ctx) {
        findings.push(format!("tool_context no catalogado: '{ctx}'"));
    }
    if inputs.get("required_secrets").is_some() {
        validate_required_secrets(inputs.get("required_secrets").unwrap(), findings);
    }
}

/// Ejecuta `policy-validator` (paridad contrato `policy-validator.md` v1.0.0).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let profile = str_field(inputs, "validation_profile")
        .ok_or("validation_profile es obligatorio (AGENT_POLICIES | TOOL_DOMAIN | BOTH)")?;
    if profile != "AGENT_POLICIES" && profile != "TOOL_DOMAIN" && profile != "BOTH" {
        return Err(format!("validation_profile no soportado: {profile}"));
    }

    let (context_list, normative_ref) = load_allowed_contexts(repo)?;
    let allowed: HashSet<String> = context_list.iter().cloned().collect();
    let mut findings = Vec::new();
    let mut contexts_checked = Vec::new();

    if profile_includes(&profile, "AGENT_POLICIES") {
        validate_agent_policies(inputs, &allowed, &mut findings, &mut contexts_checked);
    }
    if profile_includes(&profile, "TOOL_DOMAIN") {
        validate_tool_domain(inputs, &allowed, &mut findings, &mut contexts_checked);
    }

    let valid = findings.is_empty();
    if !valid {
        return Err(findings.join("; "));
    }

    let mut dictamen = json!({
        "valid": true,
        "exitCode": 0,
        "findings": [],
        "contexts_checked": contexts_checked,
        "normative_ref": normative_ref,
    });
    if let Some(origin) = str_field(inputs, "domain_origin") {
        dictamen["domain_origin"] = json!(origin);
    }

    Ok(json!({
        "success": true,
        "dictamen": dictamen,
        "valid": true,
        "exitCode": 0,
        "findings": [],
        "contexts_checked": contexts_checked,
        "normative_ref": normative_ref,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn parses_contexts_from_norm_file() {
        let repo = find_repo_root().unwrap();
        let (_, path) = resolve_normative_ref(&repo).unwrap();
        let text = fs::read_to_string(path).unwrap();
        let ctx = parse_allowed_contexts(&text);
        assert!(ctx.contains(&"ecosystem-evolution".to_string()));
        assert!(ctx.contains(&"chaos-engineering".to_string()));
        assert!(ctx.len() >= 9);
    }

    #[test]
    fn policy_validator_tool_domain_ok() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({
            "validation_profile": "TOOL_DOMAIN",
            "tool_context": "chaos-engineering",
            "domain_origin": "core"
        });
        let out = run(&repo, &inputs).expect("valid tool");
        assert_eq!(out.get("valid"), Some(&json!(true)));
    }

    #[test]
    fn policy_validator_rejects_unknown_context() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({
            "validation_profile": "TOOL_DOMAIN",
            "tool_context": "not-a-real-context"
        });
        assert!(run(&repo, &inputs).is_err());
    }

    #[test]
    fn policy_validator_agent_policies_ok() {
        let repo = find_repo_root().unwrap();
        let inputs = json!({
            "validation_profile": "AGENT_POLICIES",
            "allowed_policies": ["filesystem-ops", "ecosystem-evolution"]
        });
        let out = run(&repo, &inputs).expect("valid agent");
        assert_eq!(out.get("valid"), Some(&json!(true)));
    }
}
