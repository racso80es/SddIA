//! Cerbero: alcance `tracker-operations` vs el provider resuelto de `tracker:lifecycle`.

use super::cerbero_di_rbac::resolve_requester_policies;
use super::workspace::load_paths_config;
use crate::core::resolver::load_process_def;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const EXECUTION_CONTEXTS_FILE: &str = "execution-contexts.md";

pub fn operation_requires_tracker_context(operation: &str) -> bool {
    operation == "create_issue"
}

fn execution_contexts_path(repo: &Path) -> Result<std::path::PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    let norms_rel = cfg
        .get("directories")
        .and_then(|d| d.get("norms"))
        .and_then(|v| v.as_str())
        .ok_or("directories.norms ausente en cumulo.paths.json")?;
    let path = repo.join(norms_rel.trim().trim_matches('/')).join(EXECUTION_CONTEXTS_FILE);
    if !path.is_file() {
        return Err(format!("execution-contexts.md inaccesible: {}", path.display()));
    }
    Ok(path)
}

/// §2.10: «crear issues» en alcance; borrado sigue en fuera de alcance (webhooks → 1.4.0 en alcance).
pub fn tracker_section_allows_create_issues(markdown: &str) -> bool {
    let section = extract_tracker_section(markdown);
    let alcance_ok = section
        .lines()
        .any(|l| l.contains("**Alcance:**") && l.to_lowercase().contains("crear"));
    let fuera = section
        .lines()
        .find(|l| l.contains("**Fuera de alcance:**"))
        .map(|l| l.to_lowercase())
        .unwrap_or_default();
    let fuera_excludes_create = fuera.contains("crear");
    let delete_excluded = fuera.contains("borrar");
    alcance_ok && !fuera_excludes_create && delete_excluded
}

/// AC-WH-1: webhooks firmados en alcance; borrado y administración siguen excluidos.
pub fn tracker_section_allows_signed_webhooks(markdown: &str) -> bool {
    let section = extract_tracker_section(markdown).to_lowercase();
    let alcance = section
        .lines()
        .find(|l| l.contains("**alcance:**"))
        .unwrap_or("");
    let fuera = section
        .lines()
        .find(|l| l.contains("**fuera de alcance:**"))
        .unwrap_or("");
    alcance.contains("webhook") && !fuera.contains("webhook") && fuera.contains("borrar")
}

fn extract_tracker_section(markdown: &str) -> String {
    let mut in_section = false;
    let mut buf = String::new();
    for line in markdown.lines() {
        if line.trim().starts_with("### 2.10.") {
            in_section = true;
            buf.push_str(line);
            buf.push('\n');
            continue;
        }
        if in_section && line.trim().starts_with("### 2.") {
            break;
        }
        if in_section {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    buf
}

/// Gate Cerbero antes de invocar la cápsula Linear.
fn request_inner<'a>(payload: &'a Value) -> Option<&'a Value> {
    payload.get("request").or(Some(payload))
}

/// Done en Linear solo vía `tracker-stamp` (PullRequest_Merged | Delivery_Committed trunk_direct)
/// o replay fail-soft (`tracker-sync-replay`).
pub fn gate_linear_done_state_transition(
    process_name: Option<&str>,
    operation: &str,
    payload: &Value,
) -> Result<(), String> {
    if operation != "update_issue_state" {
        return Ok(());
    }
    let inner = request_inner(payload).ok_or("TRACKER_DONE_GUARD: sin request")?;
    let state_name = inner
        .get("state_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if !state_name.eq_ignore_ascii_case("done") {
        return Ok(());
    }
    let proc = process_name.unwrap_or("");
    if proc == "tracker-stamp" || proc == "tracker-sync-replay" {
        return Ok(());
    }
    Err(format!(
        "TRACKER_DONE_GUARD: update_issue_state Done prohibido para proceso '{proc}'"
    ))
}

pub fn gate_linear_tracker_operation(
    repo: &Path,
    process_name: Option<&str>,
    operation: &str,
) -> Result<(), String> {
    if !operation_requires_tracker_context(operation) {
        return Ok(());
    }
    let text = fs::read_to_string(execution_contexts_path(repo)?).map_err(|e| e.to_string())?;
    if !tracker_section_allows_create_issues(&text) {
        return Err(
            "CERBERO_RBAC_DENIED: create_issue no está en el alcance de execution-contexts §2.10"
                .into(),
        );
    }
    let process_name = process_name.ok_or_else(|| {
        "CERBERO_RBAC_DENIED: create_issue requiere process_name del solicitante".to_string()
    })?;
    let (_, process_def, _) = load_process_def(repo, process_name)?;
    let policies = resolve_requester_policies(&process_def, &json!({}));
    if !policies.iter().any(|p| p == "tracker-operations") {
        return Err(format!(
            "CERBERO_RBAC_DENIED: proceso '{process_name}' sin context tracker-operations (operación {operation})"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn done_state_transition_blocked_outside_tracker_stamp() {
        let err = gate_linear_done_state_transition(
            Some("bug-fix"),
            "update_issue_state",
            &json!({"request": {"operation": "update_issue_state", "state_name": "Done"}}),
        )
        .unwrap_err();
        assert!(err.contains("TRACKER_DONE_GUARD"));
        assert!(
            gate_linear_done_state_transition(
                Some("tracker-stamp"),
                "update_issue_state",
                &json!({"request": {"state_name": "Done"}}),
            )
            .is_ok()
        );
    }

    #[test]
    fn ac_rbac_3_norm_excludes_delete_keeps_create() {
        let repo = find_repo_root().expect("repo");
        let path = execution_contexts_path(&repo).unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert!(
            tracker_section_allows_create_issues(&text),
            "§2.10 debe permitir crear y seguir excluyendo borrado"
        );
        let section = extract_tracker_section(&text).to_lowercase();
        assert!(section.contains("borrar"));
    }

    #[test]
    fn ac_wh_1_norm_includes_signed_webhooks() {
        let repo = find_repo_root().expect("repo");
        let path = execution_contexts_path(&repo).unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert!(
            tracker_section_allows_signed_webhooks(&text),
            "§2.10 1.4.0 debe incluir webhooks firmados y excluir borrado"
        );
        let section_lc = extract_tracker_section(&text).to_lowercase();
        let fuera = section_lc
            .lines()
            .find(|l| l.contains("**fuera de alcance:**"))
            .unwrap_or("");
        assert!(!fuera.contains("webhook"));
        assert!(fuera.contains("borrar") || fuera.contains("administración"));
    }

    #[test]
    fn ac_rbac_1_tracker_process_allows_create_issue() {
        let repo = find_repo_root().expect("repo");
        gate_linear_tracker_operation(&repo, Some("tracker-backlog-query"), "create_issue")
            .expect("tracker-backlog-query tiene tracker-operations");
    }

    #[test]
    fn ac_rbac_2_feature_rejects_create_issue() {
        let repo = find_repo_root().expect("repo");
        let err = gate_linear_tracker_operation(&repo, Some("feature"), "create_issue").unwrap_err();
        assert!(err.contains("CERBERO_RBAC_DENIED"));
    }

    #[test]
    fn ac_rbac_1_stamp_process_allows_create_issue_gate() {
        let repo = find_repo_root().expect("repo");
        gate_linear_tracker_operation(&repo, Some("tracker-stamp"), "create_issue")
            .expect("tracker-stamp tiene tracker-operations");
    }

    #[test]
    fn refine_item_allows_create_issue_gate() {
        let repo = find_repo_root().expect("repo");
        gate_linear_tracker_operation(&repo, Some("refine-item"), "create_issue")
            .expect("refine-item tiene tracker-operations");
    }

    #[test]
    fn forge_pbi_allows_create_issue_gate() {
        let repo = find_repo_root().expect("repo");
        gate_linear_tracker_operation(&repo, Some("forge-pbi"), "create_issue")
            .expect("forge-pbi debe tener tracker-operations tras 1.1.0");
    }
}
