//! Autoridad de Códice de Dominio para process software-lifecycle (ABSTRACT-02).

use super::domain_profile::{resolve_execution_profile, ExecutionProfile};
use super::project_binding;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub const SOFTWARE_CODEX_SLUG: &str = "codex-software-engineering";

/// Membresía fallback (alineada a frontmatter del códice).
pub const SOFTWARE_PROCESS_MEMBERSHIP: &[&str] = &[
    "feature",
    "bug-fix",
    "refactorization",
    "pull-request-review",
    "accept-pr",
    "delivery-close-cycle",
];

pub const DENY_CODE: &str = "DOMAIN_AUTHORITY_DENIED";

#[derive(Debug, Clone, PartialEq)]
pub struct DomainAuthorityDenial {
    pub code: &'static str,
    pub message: String,
    pub process_name: String,
    pub profile_source: String,
}

impl DomainAuthorityDenial {
    pub fn to_error_string(&self) -> String {
        format!("{}: {}", self.code, self.message)
    }
}

fn membership_from_codex_file(repo: &Path) -> Option<Vec<String>> {
    let path = repo.join("SddIA/library/codexes/codex-software-engineering.md");
    if !path.is_file() {
        return None;
    }
    let raw = fs::read_to_string(&path).ok()?;
    let (fm, _) = split_frontmatter(&raw)?;
    let v: Value = serde_yaml::from_str(&fm).ok()?;
    let arr = v.get("process_membership")?.as_array()?;
    let list: Vec<String> = arr
        .iter()
        .filter_map(|x| x.as_str().map(|s| s.trim().to_string()))
        .filter(|s| !s.is_empty())
        .collect();
    if list.is_empty() {
        None
    } else {
        Some(list)
    }
}

fn split_frontmatter(raw: &str) -> Option<(String, String)> {
    let mut lines = raw.lines();
    if lines.next()? != "---" {
        return None;
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Some((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    None
}

pub fn process_membership(repo: &Path) -> Vec<String> {
    membership_from_codex_file(repo).unwrap_or_else(|| {
        SOFTWARE_PROCESS_MEMBERSHIP
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    })
}

pub fn is_software_process(repo: &Path, process_name: &str) -> bool {
    process_membership(repo)
        .iter()
        .any(|p| p == process_name)
}

/// Regla D4: slug software → allow; otro slug → deny; None+git_required → allow legado; else deny.
pub fn has_software_authority(profile: &ExecutionProfile) -> bool {
    match profile.codex_slug.as_deref() {
        Some(s) if s == SOFTWARE_CODEX_SLUG => true,
        Some(_) => false,
        None if profile.git_required => true,
        None => false,
    }
}

fn has_external_project_slug(inputs: &Value) -> bool {
    inputs
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .is_some_and(|s| !s.is_empty())
}

fn external_project_forge_allowed(
    repo: &Path,
    inputs: &Value,
    profile: &ExecutionProfile,
) -> Result<bool, DomainAuthorityDenial> {
    let bound = match project_binding::bind(repo, inputs) {
        Ok(Some(b)) => b,
        Ok(None) => {
            return Err(DomainAuthorityDenial {
                code: DENY_CODE,
                message: "project_slug presente pero binding vacío".into(),
                process_name: String::new(),
                profile_source: profile.source.to_string(),
            });
        }
        Err(e) => {
            return Err(DomainAuthorityDenial {
                code: DENY_CODE,
                message: e,
                process_name: String::new(),
                profile_source: profile.source.to_string(),
            });
        }
    };
    if bound.codex_slug != SOFTWARE_CODEX_SLUG {
        return Ok(false);
    }
    if !profile.software_forge {
        return Ok(false);
    }
    if profile
        .codex_slug
        .as_deref()
        .is_some_and(|s| s == "codex-kalma2-assistant")
    {
        return Ok(false);
    }
    Ok(true)
}

pub fn assert_process_allowed(
    repo: &Path,
    process_name: &str,
    inputs: &Value,
) -> Result<(), DomainAuthorityDenial> {
    if !is_software_process(repo, process_name) {
        return Ok(());
    }
    let profile = resolve_execution_profile(repo, inputs);
    if has_external_project_slug(inputs) {
        match external_project_forge_allowed(repo, inputs, &profile) {
            Ok(true) => return Ok(()),
            Ok(false) => {}
            Err(mut deny) => {
                deny.process_name = process_name.to_string();
                return Err(deny);
            }
        }
    } else if has_software_authority(&profile) {
        return Ok(());
    }
    Err(DomainAuthorityDenial {
        code: DENY_CODE,
        message: format!(
            "process '{process_name}' requiere Códice/perfil software-engineering \
             (codex_slug={SOFTWARE_CODEX_SLUG} o legado git_required=true sin otro códice); \
             profile_source={}, git_required={}, codex_slug={:?}",
            profile.source,
            profile.git_required,
            profile.codex_slug
        ),
        process_name: process_name.to_string(),
        profile_source: profile.source.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn write_codex(root: &Path, membership: &[&str]) {
        fs::create_dir_all(root.join("SddIA/library/codexes")).unwrap();
        let mem = membership
            .iter()
            .map(|s| format!("  - {s}"))
            .collect::<Vec<_>>()
            .join("\n");
        let body = format!(
            r#"---
uuid: "11111111-2222-4333-8444-555555555555"
name: "SddIA Codex Software Engineering"
version: "1.0.0"
nature: "domain-codex"
author: "tekton"
target_environment: ["software-engineering"]
certification_grade: "Pendiente"
process_membership:
{mem}
composition: []
---

# vibe
"#
        );
        fs::write(
            root.join("SddIA/library/codexes/codex-software-engineering.md"),
            body,
        )
        .unwrap();
    }

    #[test]
    fn allow_legacy_git_required_without_slug() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        let inputs = json!({});
        assert!(assert_process_allowed(td.path(), "feature", &inputs).is_ok());
    }

    #[test]
    fn deny_when_git_not_required_and_no_software_slug() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        let inputs = json!({
            "execution_profile": { "git_required": false }
        });
        let err = assert_process_allowed(td.path(), "feature", &inputs).unwrap_err();
        assert_eq!(err.code, DENY_CODE);
    }

    #[test]
    fn allow_explicit_software_slug_even_if_git_false() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        let inputs = json!({
            "execution_profile": {
                "git_required": false,
                "codex_slug": "codex-software-engineering"
            }
        });
        assert!(assert_process_allowed(td.path(), "feature", &inputs).is_ok());
    }

    #[test]
    fn deny_other_codex_slug() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        let inputs = json!({
            "execution_profile": {
                "git_required": true,
                "codex_slug": "codex-frontend-product-splus"
            }
        });
        let err = assert_process_allowed(td.path(), "feature", &inputs).unwrap_err();
        assert_eq!(err.code, DENY_CODE);
    }

    #[test]
    fn external_project_requires_software_forge_and_codex() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        fs::create_dir_all(td.path().join(".SddIA/projects")).unwrap();
        let client = td.path().join("client");
        fs::create_dir_all(client.join(".git")).unwrap();
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let manifest = format!(
            r#"---
id: demo
uuid: "{uuid}"
git_remote: https://example.invalid/x.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.0.0"
codex_slug: codex-software-engineering
docs_layout:
  features: docs/features
  fixes: docs/fixes
  todos_pending: docs/todos/pending
  todos_done: docs/todos/done
---
"#
        );
        fs::write(client.join(".SddIA/project.md"), manifest).unwrap();
        let index = format!(
            r#"---
id: demo
uuid: "{uuid}"
project_root: {root}
manifest_ref: .SddIA/project.md
codex_slug: codex-software-engineering
status: active
---
"#,
            root = client.display()
        );
        fs::write(td.path().join(".SddIA/projects/demo.md"), index).unwrap();
        fs::create_dir_all(td.path().join(".SddIA")).unwrap();
        fs::write(
            td.path().join(".SddIA/active-domain-profile.json"),
            r#"{"codex_slug":"codex-software-engineering","git_required":true,"software_forge":true}"#,
        )
        .unwrap();
        let inputs = json!({"project_slug": "demo"});
        assert!(assert_process_allowed(td.path(), "feature", &inputs).is_ok());

        fs::write(
            td.path().join(".SddIA/active-domain-profile.json"),
            r#"{"codex_slug":"codex-software-engineering","git_required":true,"software_forge":false}"#,
        )
        .unwrap();
        let err = assert_process_allowed(td.path(), "feature", &inputs).unwrap_err();
        assert_eq!(err.code, DENY_CODE);
    }

    #[test]
    fn kalma2_assistant_denied_even_with_project_slug() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        fs::create_dir_all(td.path().join(".SddIA/projects")).unwrap();
        let client = td.path().join("client");
        fs::create_dir_all(client.join(".git")).unwrap();
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        fs::write(
            client.join(".SddIA/project.md"),
            format!(
                r#"---
id: demo
uuid: "{uuid}"
git_remote: https://example.invalid/x.git
default_branch: main
delivery_mode: branch_pr
contract_version: "1.0.0"
codex_slug: codex-software-engineering
docs_layout:
  features: docs/features
  fixes: docs/fixes
  todos_pending: docs/todos/pending
  todos_done: docs/todos/done
---
"#
            ),
        )
        .unwrap();
        fs::write(
            td.path().join(".SddIA/projects/demo.md"),
            format!(
                r#"---
id: demo
uuid: "{uuid}"
project_root: {root}
manifest_ref: .SddIA/project.md
codex_slug: codex-software-engineering
status: active
---
"#,
                root = client.display()
            ),
        )
        .unwrap();
        fs::create_dir_all(td.path().join(".SddIA")).unwrap();
        fs::write(
            td.path().join(".SddIA/active-domain-profile.json"),
            r#"{"codex_slug":"codex-kalma2-assistant","git_required":false,"software_forge":true}"#,
        )
        .unwrap();
        let inputs = json!({"project_slug": "demo"});
        let err = assert_process_allowed(td.path(), "feature", &inputs).unwrap_err();
        assert_eq!(err.code, DENY_CODE);
    }

    #[test]
    fn non_member_process_always_allowed() {
        let td = tempfile::tempdir().unwrap();
        write_codex(td.path(), &["feature"]);
        let inputs = json!({
            "execution_profile": { "git_required": false }
        });
        assert!(assert_process_allowed(td.path(), "entity-manager", &inputs).is_ok());
    }
}
