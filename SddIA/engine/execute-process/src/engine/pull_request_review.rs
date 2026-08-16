//! Handlers nativos de `pull-request-review` (G1 Prep git-manager · G2 Triaje técnico formal).

use super::capsules::invoke_git_manager;
use super::invoke_orchestrator;
use super::verify_process_integrity;
use serde_json::{json, Value};
use std::path::Path;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

/// `pr_branch` (genoma PPR) con fallback `branch_name` (alias runtime Kalma2).
fn resolve_pr_branch(inputs: &Value) -> Option<String> {
    str_field(inputs, "pr_branch").or_else(|| str_field(inputs, "branch_name"))
}

fn truthy_flag(v: &Value, key: &str) -> bool {
    match v.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on"),
        _ => false,
    }
}

/// Fricción no causal de sub-fase PPR (API externa, timeout, bridge git soft).
pub(crate) fn is_ppr_fail_soft_friction(err: &str) -> bool {
    let e = err.to_lowercase();
    e.contains("timeout")
        || e.contains("timed out")
        || e.contains("rate limit")
        || e.contains("api")
        || e.contains("soft")
        || e.contains("bridge")
        || e.contains("lectura")
        || e.contains("evidence")
        || e.contains("network")
        || e.contains("temporarily")
}

/// Ejecuta fase nativa de PPR. `None` = fase no cubierta (seguir residual).
pub fn execute_pull_request_review_phase(
    repo: &Path,
    phase_name: &str,
    inputs: &Value,
    state: &mut Value,
) -> Option<Result<Value, String>> {
    match phase_name {
        "Preparación de rama" => Some(prep_branch(repo, inputs, state)),
        "Triaje técnico" => Some(tech_triage(repo, inputs, state)),
        "Handoff materialización" => Some(handoff_accept_pr(repo, inputs, state)),
        _ => None,
    }
}

fn prep_branch(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    let branch = resolve_pr_branch(inputs)
        .ok_or_else(|| "pr_branch (o branch_name) obligatorio para Preparación de rama".to_string())?;

    let mut steps = Vec::new();

    let fetch = invoke_git_manager(repo, "fetch", &json!({"remote": "origin", "prune": false}))?;
    steps.push(json!({"op": "fetch", "result": fetch}));

    let checkout = invoke_git_manager(
        repo,
        "checkout",
        &json!({"branch_name": branch, "create_if_not_exists": false}),
    )?;
    steps.push(json!({"op": "checkout", "branch": branch, "result": checkout}));

    // L-FAILSOFT-OLA1: lectura puntual post-checkout no colapsa la fase lineal.
    let (status, status_fail_soft) = match invoke_git_manager(repo, "status", &json!({})) {
        Ok(s) => (s, false),
        Err(e) => (
            json!({
                "ok": false,
                "error": e,
                "note": "git status soft; checkout OK",
            }),
            true,
        ),
    };
    steps.push(json!({"op": "status", "result": status.clone(), "fail_soft": status_fail_soft}));

    if let Some(obj) = state.as_object_mut() {
        obj.insert("pr_branch".into(), json!(branch));
        obj.insert("branch_name".into(), json!(branch));
        obj.insert("git_manager_invoked".into(), json!(true));
        obj.insert("git_prep_status".into(), status.clone());
    }

    let mut out = json!({
        "status": if status_fail_soft { "failed" } else { "executed" },
        "handler": "ppr-prep-branch",
        "git_manager_invoked": true,
        "pr_branch": branch,
        "branch_name": branch,
        "git_steps": steps,
        "git_status": status,
    });
    if status_fail_soft {
        out["fail_soft"] = json!(true);
        out["error"] = json!("git status soft friction post-checkout (ola1)");
    }
    Ok(out)
}

fn tech_triage(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    // Formal F3: verify-process-integrity in-process (paridad sddia-qa; sin proxy documental).
    let integrity = verify_process_integrity::verify(repo);
    let (integrity_ok, integrity_detail) = match &integrity {
        Ok(()) => (true, json!({"ok": true})),
        Err(errs) => (false, json!({"ok": false, "errors": errs})),
    };

    let mut checks = json!({
        "TECH_FORMAL_EXECUTE_PROCESS": if integrity_ok { "APTO" } else { "NO_APTO" },
        "verify_process_integrity": if integrity_ok { "APTO" } else { "NO_APTO" },
    });

    // Sensor DIA: script Python podado → SKIPPED_ABSENT (no bloquea peaje).
    let dia_script = repo.join("SddIA/scripts/qa/audit-doc-parity.py");
    if dia_script.is_file() {
        checks["DIA_SENSOR"] = json!("PRESENT_UNWIRED");
    } else {
        checks["DIA_SENSOR"] = json!("SKIPPED_ABSENT");
    }

    if let Some(obj) = state.as_object_mut() {
        obj.insert("tech_triage_formal".into(), json!(true));
        obj.insert("tech_checks".into(), checks.clone());
    }

    if !integrity_ok {
        return Err(format!(
            "Triaje técnico formal: verify-process-integrity falló: {integrity_detail}"
        ));
    }

    Ok(json!({
        "status": "executed",
        "handler": "ppr-tech-triage",
        "formal_execute_process": true,
        "TECH_FORMAL_EXECUTE_PROCESS": "APTO",
        "checks": checks,
        "verify_process_integrity": integrity_detail,
        "persist_ref": str_field(inputs, "persist_ref"),
    }))
}

fn handoff_accept_pr(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    if env_truthy("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF") {
        return Ok(json!({
            "status": "executed",
            "handler": "ppr-handoff-accept-pr",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF",
            "accept_pr_handoff": false,
        }));
    }

    let verdict_ok = state
        .get("verdict")
        .and_then(|v| v.as_str())
        .map(|s| s == "aprobado")
        .unwrap_or(false)
        || truthy_flag(state, "accept_pr_handoff")
        || truthy_flag(inputs, "accept_pr_handoff");

    if !verdict_ok {
        return Ok(json!({
            "status": "executed",
            "handler": "ppr-handoff-accept-pr",
            "skipped": true,
            "reason": "verdict_not_aprobado",
            "accept_pr_handoff": false,
        }));
    }

    let branch = resolve_pr_branch(inputs)
        .or_else(|| {
            state
                .get("pr_branch")
                .or_else(|| state.get("branch_name"))
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .ok_or_else(|| "pr_branch obligatorio para Handoff materialización".to_string())?;

    let mut child = json!({
        "source_branch": branch,
        "correlation_id": inputs.get("correlation_id").cloned().unwrap_or(json!("")),
    });
    if let Some(url) = str_field(inputs, "pr_url") {
        child["pr_url"] = json!(url);
    }
    if truthy_flag(inputs, "merge_already_done") || truthy_flag(state, "merge_already_done") {
        child["merge_already_done"] = json!(true);
    }

    let data = invoke_orchestrator::invoke_process(repo, "accept-pr", &child)?;
    if let Some(obj) = state.as_object_mut() {
        obj.insert("accept_pr_handoff".into(), json!(true));
        obj.insert("accept_pr".into(), data.clone());
    }

    Ok(json!({
        "status": "executed",
        "handler": "ppr-handoff-accept-pr",
        "accept_pr_handoff": true,
        "accept_pr": data,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_pr_branch_prefers_pr_branch() {
        let v = json!({"pr_branch": "feat/a", "branch_name": "feat/b"});
        assert_eq!(resolve_pr_branch(&v).as_deref(), Some("feat/a"));
    }

    #[test]
    fn resolve_pr_branch_falls_back_to_branch_name() {
        let v = json!({"branch_name": "fix/x"});
        assert_eq!(resolve_pr_branch(&v).as_deref(), Some("fix/x"));
    }

    #[test]
    fn handoff_skips_without_verdict() {
        let dir = std::env::temp_dir().join(format!("sddia-ppr-ho-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut state = json!({});
        let inputs = json!({"pr_branch": "feat/x"});
        let out = handoff_accept_pr(&dir, &inputs, &mut state).unwrap();
        assert_eq!(out["skipped"], true);
        assert_eq!(out["reason"], "verdict_not_aprobado");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ppr_fail_soft_friction_patterns() {
        assert!(is_ppr_fail_soft_friction("GitHub API timeout"));
        assert!(is_ppr_fail_soft_friction("soft bridge evidence"));
        assert!(!is_ppr_fail_soft_friction(
            "Triaje técnico formal: verify-process-integrity falló"
        ));
    }
}
