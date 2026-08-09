//! Perfil de dominio activo (instancia + inputs de proceso).
//!
//! Precedencia: `execution_profile` (input) > `active_codex_ref`+defaults >
//! `.SddIA/active-domain-profile.json` > default (`git_required: true`).

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub const INSTANCE_PROFILE_REL: &str = ".SddIA/active-domain-profile.json";

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionProfile {
    pub codex_slug: Option<String>,
    pub codex_uuid: Option<String>,
    pub git_required: bool,
    pub allowed_policies: Vec<String>,
    pub source: &'static str,
}

impl Default for ExecutionProfile {
    fn default() -> Self {
        Self {
            codex_slug: None,
            codex_uuid: None,
            git_required: true,
            allowed_policies: Vec::new(),
            source: "default",
        }
    }
}

impl ExecutionProfile {
    pub fn from_value(v: &Value, source: &'static str) -> Self {
        let mut p = Self::default();
        p.source = source;
        if let Some(s) = v.get("codex_slug").and_then(|x| x.as_str()) {
            let t = s.trim();
            if !t.is_empty() && t != "null" {
                p.codex_slug = Some(t.to_string());
            }
        }
        if let Some(s) = v.get("codex_uuid").and_then(|x| x.as_str()) {
            let t = s.trim();
            if !t.is_empty() && t != "null" {
                p.codex_uuid = Some(t.to_string());
            }
        }
        if let Some(b) = v.get("git_required").and_then(|x| x.as_bool()) {
            p.git_required = b;
        }
        if let Some(arr) = v.get("allowed_policies").and_then(|x| x.as_array()) {
            p.allowed_policies = arr
                .iter()
                .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
                .filter(|s| !s.is_empty())
                .collect();
        }
        p
    }

    pub fn to_json(&self) -> Value {
        json!({
            "codex_slug": self.codex_slug,
            "codex_uuid": self.codex_uuid,
            "git_required": self.git_required,
            "allowed_policies": self.allowed_policies,
            "source": self.source,
        })
    }
}

fn load_instance_profile(repo: &Path) -> Option<ExecutionProfile> {
    let path = repo.join(INSTANCE_PROFILE_REL);
    if !path.is_file() {
        return None;
    }
    let raw = fs::read_to_string(&path).ok()?;
    let v: Value = serde_json::from_str(&raw).ok()?;
    Some(ExecutionProfile::from_value(&v, "instance"))
}

/// Resuelve el perfil de ejecución según precedencia documentada en spec.
pub fn resolve_execution_profile(repo: &Path, inputs: &Value) -> ExecutionProfile {
    if let Some(obj) = inputs.get("execution_profile") {
        if obj.is_object() {
            return ExecutionProfile::from_value(obj, "input_execution_profile");
        }
    }

    if let Some(slug) = inputs
        .get("active_codex_ref")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let mut p = load_instance_profile(repo).unwrap_or_default();
        p.codex_slug = Some(slug.to_string());
        p.source = "input_active_codex_ref";
        // active_codex_ref alone does not flip git_required; keep instance/default.
        if let Some(b) = inputs.get("git_required").and_then(|x| x.as_bool()) {
            p.git_required = b;
        }
        return p;
    }

    if let Some(p) = load_instance_profile(repo) {
        return p;
    }

    ExecutionProfile::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn precedence_input_over_instance() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        fs::create_dir_all(root.join(".SddIA")).unwrap();
        fs::write(
            root.join(INSTANCE_PROFILE_REL),
            r#"{"git_required":true,"codex_slug":"from-instance"}"#,
        )
        .unwrap();
        let inputs = json!({
            "execution_profile": {
                "git_required": false,
                "codex_slug": "from-input"
            }
        });
        let p = resolve_execution_profile(root, &inputs);
        assert!(!p.git_required);
        assert_eq!(p.codex_slug.as_deref(), Some("from-input"));
        assert_eq!(p.source, "input_execution_profile");
    }

    #[test]
    fn instance_when_no_input() {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        fs::create_dir_all(root.join(".SddIA")).unwrap();
        fs::write(
            root.join(INSTANCE_PROFILE_REL),
            r#"{"git_required":false,"allowed_policies":["filesystem-ops"]}"#,
        )
        .unwrap();
        let p = resolve_execution_profile(root, &json!({}));
        assert!(!p.git_required);
        assert_eq!(p.source, "instance");
        assert_eq!(p.allowed_policies, vec!["filesystem-ops".to_string()]);
    }

    #[test]
    fn default_git_required_true() {
        let td = tempfile::tempdir().unwrap();
        let p = resolve_execution_profile(td.path(), &json!({}));
        assert!(p.git_required);
        assert_eq!(p.source, "default");
    }

    #[test]
    fn active_codex_ref_sets_slug() {
        let td = tempfile::tempdir().unwrap();
        let inputs = json!({
            "active_codex_ref": "codex-frontend-product-splus",
            "git_required": false
        });
        let p = resolve_execution_profile(td.path(), &inputs);
        assert_eq!(p.codex_slug.as_deref(), Some("codex-frontend-product-splus"));
        assert!(!p.git_required);
        assert_eq!(p.source, "input_active_codex_ref");
    }
}
