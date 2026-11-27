//! Handlers nativos de `pull-request-review` (G1 Prep git-manager · G2 Triaje técnico formal).

use super::capsules::invoke_git_manager;
use super::invoke_orchestrator;
use super::qa_profile::{is_docs_only_profile, PROFILE_DOCS_ONLY};
use super::verify_process_integrity;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::process::Command;

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

/// `author` contractual de `accept-pr` v2.0.0 (no está en DEFAULTABLE del resolver).
pub(crate) fn resolve_accept_pr_author(inputs: &Value, state: &Value) -> String {
    str_field(inputs, "author")
        .or_else(|| str_field(state, "author"))
        .or_else(|| str_field(inputs, "emitter_agent"))
        .or_else(|| str_field(state, "emitter_agent"))
        .unwrap_or_else(|| "integration-operator".to_string())
}

/// AC-2 (AUT-134): atestación antes de Fase 7 Handoff materialización.
pub fn maybe_emit_qa_attestation_before_handoff(
    repo: &Path,
    state: &mut Value,
    inputs: &Value,
) -> Option<PathBuf> {
    if state
        .get("qa_attestation_written")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return state
            .get("attestation_path")
            .and_then(|v| v.as_str())
            .map(PathBuf::from);
    }
    match super::qa_attestation::try_write_from_ppr(repo, state, inputs) {
        Ok(path) => {
            if let Some(obj) = state.as_object_mut() {
                obj.insert("qa_attestation_written".into(), json!(true));
                obj.insert(
                    "attestation_path".into(),
                    json!(path.to_string_lossy().to_string()),
                );
            }
            Some(path)
        }
        Err(_) => None,
    }
}

/// AC-3 (AUT-134): PPR es auditoría de solo lectura sobre PR sellado — revertir mutaciones locales.
pub fn restore_persist_ref_worktree(repo: &Path, persist_ref: &str) -> Result<Value, String> {
    let rel = persist_ref
        .trim()
        .trim_start_matches("./")
        .trim_end_matches('/');
    if rel.is_empty() {
        return Ok(json!({"skipped": true, "reason": "persist_ref_empty"}));
    }
    let restore = Command::new("git")
        .args([
            "restore",
            "--source=HEAD",
            "--worktree",
            "--",
            rel,
        ])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git restore persist_ref: {e}"))?;
    if !restore.status.success() {
        let stderr = String::from_utf8_lossy(&restore.stderr);
        return Err(format!("git restore persist_ref falló: {stderr}"));
    }
    let handoff = repo.join(rel).join("_agent_handoff.md");
    let mut removed_handoff = false;
    if handoff.is_file() {
        let porcelain = Command::new("git")
            .args(["status", "--porcelain", "--", &format!("{rel}/_agent_handoff.md")])
            .current_dir(repo)
            .output()
            .map_err(|e| format!("git status handoff: {e}"))?;
        let line = String::from_utf8_lossy(&porcelain.stdout);
        if line.trim_start().starts_with("??") || line.trim_start().starts_with("!!") {
            let _ = std::fs::remove_file(&handoff);
            removed_handoff = true;
        }
    }
    Ok(json!({
        "status": "executed",
        "handler": "ppr-restore-persist-worktree",
        "persist_ref": rel,
        "removed_untracked_handoff": removed_handoff,
    }))
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
        "Triaje documental" if is_docs_only_profile(inputs) => {
            Some(documental_triage_deterministic(repo, inputs, state))
        }
        "Triaje técnico" => Some(tech_triage(repo, inputs, state)),
        "Handoff materialización" => Some(handoff_accept_pr(repo, inputs, state)),
        _ => None,
    }
}

fn documental_triage_deterministic(
    repo: &Path,
    inputs: &Value,
    state: &mut Value,
) -> Result<Value, String> {
    let persist = str_field(inputs, "persist_ref")
        .or_else(|| str_field(inputs, "document_context"))
        .unwrap_or_else(|| "docs/features".into());
    let base = if Path::new(&persist).is_absolute() {
        Path::new(&persist).to_path_buf()
    } else {
        repo.join(&persist)
    };
    let required = ["objectives.md", "spec.md", "plan.md", "implementation.md"];
    let mut missing = Vec::new();
    for name in required {
        if !base.join(name).is_file() {
            missing.push(name);
        }
    }
    let ok = missing.is_empty();
    if let Some(obj) = state.as_object_mut() {
        obj.insert("qa_profile".into(), json!(PROFILE_DOCS_ONLY));
        if ok {
            obj.insert("verdict".into(), json!("aprobado"));
            obj.insert("argos_verdict".into(), json!("pass"));
        }
    }
    if !ok {
        return Err(format!(
            "Triaje documental docs-only: faltan artefactos en {}: {}",
            base.display(),
            missing.join(", ")
        ));
    }
    Ok(json!({
        "status": "executed",
        "handler": "ppr-documental-deterministic",
        "qa_profile": PROFILE_DOCS_ONLY,
        "note": "deterministic-doc-triage",
        "persist_ref": persist,
    }))
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

    let base_ref = str_field(inputs, "base_ref")
        .unwrap_or_else(|| super::entity_no_anemia::default_base_ref(repo));
    let (anemia_ok, anemia_detail) = anemia_triage(repo, &base_ref);

    let mut checks = json!({
        "TECH_FORMAL_EXECUTE_PROCESS": if integrity_ok { "APTO" } else { "NO_APTO" },
        "verify_process_integrity": if integrity_ok { "APTO" } else { "NO_APTO" },
        "gate_entity_no_anemia": if anemia_ok { "APTO" } else { "NO_APTO" },
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
    if !anemia_ok {
        return Err(format!(
            "Triaje técnico formal: gate-entity-no-anemia falló: {anemia_detail}"
        ));
    }

    if let Some(persist) = str_field(inputs, "persist_ref") {
        if super::validacion_runtime_parity::persist_requires_runtime_parity_smoke(repo, &persist) {
            match super::validacion_runtime_parity::check_runtime_binary_parity_gate(repo) {
                Ok(parity) if parity.get("ok").and_then(|v| v.as_bool()) == Some(true) => {
                    checks["runtime_binary_parity"] = json!("APTO");
                    if let Some(obj) = state.as_object_mut() {
                        obj.insert("runtime_binary_parity".into(), parity);
                    }
                }
                Ok(parity) => {
                    checks["runtime_binary_parity"] = json!("NO_APTO");
                    return Err(format!(
                        "Triaje técnico: runtime_binary_parity no ok: {}",
                        parity.get("stale").cloned().unwrap_or(json!([]))
                    ));
                }
                Err(e) => {
                    checks["runtime_binary_parity"] = json!("NO_APTO");
                    return Err(format!("Triaje técnico: runtime_binary_parity: {e}"));
                }
            }
        } else {
            checks["runtime_binary_parity"] = json!("SKIPPED");
        }
    }

    Ok(json!({
        "status": "executed",
        "handler": "ppr-tech-triage",
        "formal_execute_process": true,
        "TECH_FORMAL_EXECUTE_PROCESS": "APTO",
        "checks": checks,
        "verify_process_integrity": integrity_detail,
        "gate_entity_no_anemia": anemia_detail,
        "persist_ref": str_field(inputs, "persist_ref"),
    }))
}

/// Peaje anti-anemia sobre las entidades de genoma tocadas en `base_ref...HEAD`.
fn anemia_triage(repo: &Path, base_ref: &str) -> (bool, Value) {
    match super::entity_no_anemia::scan_changed(repo, base_ref) {
        Ok(rep) if rep.ok => (true, json!({"ok": true, "base_ref": base_ref, "scanned": rep.scanned})),
        Ok(rep) => (
            false,
            json!({
                "ok": false,
                "base_ref": base_ref,
                "scanned": rep.scanned,
                "violation_count": rep.violation_count,
                "violations": rep.violations.iter().take(20).collect::<Vec<_>>(),
            }),
        ),
        Err(e) => (false, json!({"ok": false, "base_ref": base_ref, "error": e})),
    }
}

/// True si el error de invoke accept-pr es bloqueo Cerbero/revoked/forbidden (L-HANDOFF-RUNTIME).
pub(crate) fn is_accept_pr_block_err(err: &str) -> bool {
    let e = err.to_lowercase();
    e.contains("revoked")
        || e.contains("forbidden")
        || e.contains("cerbero")
        || e.contains("permanently revoked")
        || e.contains("entity revoked")
}

fn handoff_accept_pr(repo: &Path, inputs: &Value, state: &mut Value) -> Result<Value, String> {
    if is_docs_only_profile(inputs) {
        if let Some(obj) = state.as_object_mut() {
            obj.insert("accept_pr_handoff".into(), json!(false));
            obj.insert("accept_pr_handoff_status".into(), json!("skipped"));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "ppr-handoff-accept-pr",
            "skipped": true,
            "reason": "qa_profile_docs_only",
            "note": "skipped-by-profile",
            "qa_profile": PROFILE_DOCS_ONLY,
            "accept_pr_handoff": false,
            "accept_pr_handoff_status": "skipped",
        }));
    }
    if env_truthy("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF") {
        if let Some(obj) = state.as_object_mut() {
            obj.insert("accept_pr_handoff".into(), json!(false));
            obj.insert("accept_pr_handoff_status".into(), json!("skipped"));
        }
        return Ok(json!({
            "status": "executed",
            "handler": "ppr-handoff-accept-pr",
            "skipped": true,
            "reason": "SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF",
            "accept_pr_handoff": false,
            "accept_pr_handoff_status": "skipped",
        }));
    }

    if let Some(persist) = str_field(inputs, "persist_ref").or_else(|| str_field(state, "persist_ref")) {
        if let Err(fault) = super::apto_coherence::validate_apto_coherence(repo, &persist) {
            if let Some(obj) = state.as_object_mut() {
                obj.insert("verdict".into(), json!("rechazado"));
                obj.insert("accept_pr_handoff".into(), json!(false));
                obj.insert("accept_pr_handoff_status".into(), json!("blocked"));
                obj.insert("accept_pr_block_reason".into(), json!(fault.to_error_string()));
            }
            return Ok(json!({
                "status": "executed",
                "handler": "ppr-handoff-accept-pr",
                "accept_pr_handoff": false,
                "accept_pr_handoff_status": "blocked",
                "friction_id": fault.friction_id,
                "block_reason": fault.to_error_string(),
            }));
        }
    }

    let verdict_ok = state
        .get("verdict")
        .and_then(|v| v.as_str())
        .map(|s| s == "aprobado")
        .unwrap_or(false)
        || truthy_flag(state, "accept_pr_handoff")
        || truthy_flag(inputs, "accept_pr_handoff");

    if !verdict_ok {
        if let Some(obj) = state.as_object_mut() {
            obj.insert("accept_pr_handoff".into(), json!(false));
            obj.insert("accept_pr_handoff_status".into(), json!("skipped"));
        }
        let lifecycle_ci = super::lifecycle_phase_action::apply_review_ci_fail_soft(
            repo,
            inputs,
            false,
            "pull-request-review",
        );
        return Ok(json!({
            "status": "executed",
            "handler": "ppr-handoff-accept-pr",
            "skipped": true,
            "reason": "verdict_not_aprobado",
            "accept_pr_handoff": false,
            "accept_pr_handoff_status": "skipped",
            "lifecycle_review_ci": lifecycle_ci,
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
        "author": resolve_accept_pr_author(inputs, state),
    });
    if let Some(url) = str_field(inputs, "pr_url") {
        child["pr_url"] = json!(url);
    }
    if let Some(p) = str_field(inputs, "persist_ref") {
        child["persist_ref"] = json!(p);
    }
    if truthy_flag(inputs, "merge_already_done") || truthy_flag(state, "merge_already_done") {
        child["merge_already_done"] = json!(true);
    }

    match invoke_orchestrator::invoke_process(repo, "accept-pr", &child) {
        Ok(data) => {
            // L-HANDOFF-STATUS: true solo si status == consumed.
            if let Some(obj) = state.as_object_mut() {
                obj.insert("accept_pr_handoff".into(), json!(true));
                obj.insert("accept_pr_handoff_status".into(), json!("consumed"));
                obj.insert("accept_pr".into(), data.clone());
            }
            Ok(json!({
                "status": "executed",
                "handler": "ppr-handoff-accept-pr",
                "accept_pr_handoff": true,
                "accept_pr_handoff_status": "consumed",
                "accept_pr": data,
            }))
        }
        Err(e) if is_accept_pr_block_err(&e) => {
            if let Some(obj) = state.as_object_mut() {
                obj.insert("accept_pr_handoff".into(), json!(false));
                obj.insert("accept_pr_handoff_status".into(), json!("blocked"));
                obj.insert("accept_pr_block_reason".into(), json!(e.clone()));
            }
            Ok(json!({
                "status": "executed",
                "handler": "ppr-handoff-accept-pr",
                "accept_pr_handoff": false,
                "accept_pr_handoff_status": "blocked",
                "block_reason": e,
            }))
        }
        Err(e) => {
            // Otros Err: señal blocked explícita sin inventar merge (L-HANDOFF-RUNTIME c).
            if let Some(obj) = state.as_object_mut() {
                obj.insert("accept_pr_handoff".into(), json!(false));
                obj.insert("accept_pr_handoff_status".into(), json!("blocked"));
                obj.insert("accept_pr_block_reason".into(), json!(e.clone()));
            }
            Ok(json!({
                "status": "executed",
                "handler": "ppr-handoff-accept-pr",
                "accept_pr_handoff": false,
                "accept_pr_handoff_status": "blocked",
                "block_reason": e,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tech_triage_blocks_when_anemia_base_unresolvable() {
        let repo = crate::core::repo::find_repo_root().expect("repo");
        let mut state = json!({});
        let inputs = json!({"base_ref": "refs/heads/aut125-no-existe"});
        let err = tech_triage(&repo, &inputs, &mut state).unwrap_err();
        assert!(err.contains("gate-entity-no-anemia"), "{err}");
        assert_eq!(state["tech_checks"]["gate_entity_no_anemia"], "NO_APTO");
    }

    #[test]
    fn tech_triage_reports_anemia_detail_on_success() {
        let repo = crate::core::repo::find_repo_root().expect("repo");
        let mut state = json!({});
        let inputs = json!({"base_ref": "HEAD"});
        let out = tech_triage(&repo, &inputs, &mut state).expect("triage");
        assert_eq!(out["checks"]["gate_entity_no_anemia"], "APTO");
        assert_eq!(out["gate_entity_no_anemia"]["ok"], true);
        assert_eq!(out["gate_entity_no_anemia"]["base_ref"], "HEAD");
        assert_eq!(out["gate_entity_no_anemia"]["scanned"], 0);
    }

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
    fn resolve_accept_pr_author_prefers_inputs_then_default() {
        let inputs = json!({"author": "racso"});
        let state = json!({});
        assert_eq!(resolve_accept_pr_author(&inputs, &state), "racso");
        let inputs = json!({});
        let state = json!({"emitter_agent": "pull-request-review"});
        assert_eq!(resolve_accept_pr_author(&inputs, &state), "pull-request-review");
        assert_eq!(
            resolve_accept_pr_author(&json!({}), &json!({})),
            "integration-operator"
        );
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
        assert_eq!(out["accept_pr_handoff"], false);
        assert_eq!(out["accept_pr_handoff_status"], "skipped");
        assert_eq!(state["accept_pr_handoff"], false);
        assert_eq!(state["accept_pr_handoff_status"], "skipped");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn t_a3_skip_verdict() {
        let dir = std::env::temp_dir().join(format!("sddia-ppr-ho-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut state = json!({"verdict": "requiere_cambios"});
        let inputs = json!({"pr_branch": "feat/x"});
        let out = handoff_accept_pr(&dir, &inputs, &mut state).unwrap();
        assert_eq!(out["accept_pr_handoff"], false);
        assert_eq!(out["accept_pr_handoff_status"], "skipped");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn t_a3_blocked_err_classifier() {
        assert!(is_accept_pr_block_err("process accept-pr is revoked in Cerbero"));
        assert!(is_accept_pr_block_err("FORBIDDEN: entity revoked"));
        assert!(is_accept_pr_block_err("cerbero gate denied"));
        assert!(!is_accept_pr_block_err("merge conflict on main"));
    }

    #[test]
    fn t_a3_no_true_pending_f5_convention() {
        let dir = std::env::temp_dir().join(format!("sddia-ppr-f5-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let prev = std::env::var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF").ok();
        std::env::remove_var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF");
        let mut state = json!({ "verdict": "aprobado" });
        let inputs = json!({ "pr_branch": "fix/kaizen-rust-compile-audit" });
        let out = handoff_accept_pr(&dir, &inputs, &mut state).unwrap();
        assert_ne!(out["accept_pr_handoff"], json!(true));
        assert_ne!(state["accept_pr_handoff"], json!(true));
        if let Some(v) = prev {
            std::env::set_var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF", v);
        } else {
            std::env::remove_var("SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF");
        }
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

    #[test]
    fn ka_gate_trunk_2_handoff_blocks_on_f3_no_apto() {
        let dir = std::env::temp_dir().join(format!("sddia-ppr-trunk-{}", uuid::Uuid::new_v4()));
        let persist = dir.join("docs/features/f-trunk");
        std::fs::create_dir_all(&persist).unwrap();
        std::fs::write(
            persist.join("validacion.md"),
            "---\nglobal: APTO\nchecks:\n  F3_TECH_GATE: NO_APTO\n---\n\n# v\n",
        )
        .unwrap();
        std::fs::write(
            persist.join("execution.md"),
            "---\nverdict: ok\nstatus: complete\n---\n\n# e\n",
        )
        .unwrap();
        let mut state = json!({ "verdict": "aprobado" });
        let inputs = json!({
            "pr_branch": "feat/f-trunk",
            "persist_ref": "docs/features/f-trunk"
        });
        let out = handoff_accept_pr(&dir, &inputs, &mut state).unwrap();
        assert_eq!(out["accept_pr_handoff"], false);
        assert_eq!(out["accept_pr_handoff_status"], "blocked");
        assert_eq!(out["friction_id"], "apto_coherence_trunk_gate");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
