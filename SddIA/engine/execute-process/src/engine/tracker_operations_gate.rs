//! Cerbero: alcance `tracker-operations` vs operaciones de `linear-tracker-adapter` (HU-A 03).

use super::cerbero_di_rbac::resolve_requester_policies;
use super::workspace::load_paths_config;
use crate::core::resolver::load_process_def;
use serde_json::json;
use std::fs;
use std::path::Path;

const EXECUTION_CONTEXTS_FILE: &str = "execution-contexts.md";
pub const LINEAR_TRACKER_TOOL: &str = "linear-tracker-adapter";

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

/// §2.10: «crear issues» en alcance; borrado y webhooks siguen en fuera de alcance.
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
    let webhooks_excluded = fuera.contains("webhook");
    let delete_excluded = fuera.contains("borrar");
    alcance_ok && !fuera_excludes_create && webhooks_excluded && delete_excluded
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
    fn ac_rbac_3_norm_excludes_webhooks_and_delete() {
        let repo = find_repo_root().expect("repo");
        let path = execution_contexts_path(&repo).unwrap();
        let text = fs::read_to_string(path).unwrap();
        assert!(
            tracker_section_allows_create_issues(&text),
            "§2.10 debe permitir crear y seguir excluyendo webhooks/borrado"
        );
        let section = extract_tracker_section(&text).to_lowercase();
        assert!(section.contains("webhook"));
        assert!(section.contains("borrar"));
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
    fn refine_hu_allows_create_issue_gate() {
        let repo = find_repo_root().expect("repo");
        gate_linear_tracker_operation(&repo, Some("refine-hu"), "create_issue")
            .expect("refine-hu tiene tracker-operations");
    }
}
