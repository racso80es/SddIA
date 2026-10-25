//! Handler nativo `materialize-ci-chronic-failure-pbi` — PBI Kaizen por cuota CI crónica.

use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn required_display(inputs: &Value, key: &str) -> Result<String, String> {
    match inputs.get(key) {
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        Some(Value::Number(n)) => Ok(n.to_string()),
        _ => Err(format!("{key} es obligatorio")),
    }
}

pub fn slugify_job_name(job_name: &str) -> String {
    let raw: String = job_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();
    let collapsed: String = raw
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if collapsed.is_empty() {
        "job".into()
    } else {
        collapsed
    }
}

pub fn chronic_document_id(job_slug: &str) -> String {
    format!("PBI-KAIZEN-CI-CHRONIC-{}", job_slug.to_uppercase())
}

fn find_pbi_by_document_id(repo: &Path, document_id: &str) -> Option<PathBuf> {
    let body = file_tracker_adapter::dispatch(
        repo,
        &json!({
            "request": { "operation": "fetch_issue", "issue_ref": document_id }
        }),
    )
    .ok()?;
    let rel = body.get("artifact_path").and_then(|v| v.as_str())?;
    Some(repo.join(rel))
}

fn build_pbi_body(
    document_id: &str,
    uuid: &str,
    job_name: &str,
    workflow_name: &str,
    failure_count: &str,
    quota_limit: &str,
    head_sha: &str,
    html_url: &str,
    sample_check_run_id: &str,
    repository: &str,
) -> String {
    format!(
        r#"---
document_id: {document_id}
uuid: "{uuid}"
title: "[KAIZEN] CI crónica — {job_name}"
format: markdown
version: "1.0.0"
status: pending
priority: media
process: feature
type: kaizen
---

# [KAIZEN] CI crónica — {job_name}

Materializado por Cúmulo ante `CI_Chronic_Failure_Detected`. No es Kintsugi. No es DIA.

| Campo | Valor |
|-------|-------|
| `job_name` | `{job_name}` |
| `workflow_name` | `{workflow_name}` |
| `failure_count` | `{failure_count}` |
| `quota_limit` | `{quota_limit}` |
| `head_sha` | `{head_sha}` |
| `html_url` | `{html_url}` |
| `sample_check_run_id` | `{sample_check_run_id}` |
| `repository` | `{repository}` |
"#
    )
}

/// Ejecuta `materialize-ci-chronic-failure-pbi`.
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let job_name = required_display(inputs, "job_name")?;
    let workflow_name = required_display(inputs, "workflow_name")?;
    let failure_count = required_display(inputs, "failure_count")?;
    let quota_limit = required_display(inputs, "quota_limit")?;
    let sample_check_run_id = required_display(inputs, "sample_check_run_id")?;
    let sample_html_url = required_display(inputs, "sample_html_url")?;
    let repository = required_display(inputs, "repository")?;
    let head_sha = required_display(inputs, "head_sha")?;

    let slug = slugify_job_name(&job_name);
    let document_id = chronic_document_id(&slug);

    if let Some(tc) = super::project_binding::tracker_config_for_inputs(repo, inputs)? {
        let tool = super::handlers::tracker_stamp::lifecycle_tool_name(repo)?;
        let listed = super::capsules::invoke_tool_for_process(
            repo,
            &tool,
            &json!({ "request": { "operation": "list_issues", "team_key": tc.team_key, "project_id": tc.project_id } }),
            Some("materialize-ci-chronic-failure-pbi"),
        )?;
        if let Some(items) = listed.get("items").and_then(|v| v.as_array()) {
            for item in items {
                if item.get("title").and_then(|v| v.as_str()).unwrap_or("").contains(&document_id) {
                    let id = item.get("identifier").and_then(|v| v.as_str()).unwrap_or("");
                    return Ok(json!({
                        "success": true,
                        "status": "already_open_or_done",
                        "target_path": id,
                        "issue_ref": id,
                        "document_id": document_id,
                    }));
                }
            }
        }
        let created = super::capsules::invoke_tool_for_process(
            repo,
            &tool,
            &json!({
                "request": {
                    "operation": "create_issue",
                    "team_key": tc.team_key,
                    "project_id": tc.project_id,
                    "title": format!("CI crónica {document_id}"),
                    "description": format!("{workflow_name} {job_name} fallos={failure_count}"),
                    "labels": ["pbi"],
                }
            }),
            Some("materialize-ci-chronic-failure-pbi"),
        )?;
        let id = created.get("issue_ref").and_then(|v| v.as_str()).unwrap_or("");
        return Ok(json!({
            "success": true,
            "status": "materialized",
            "target_path": id,
            "issue_ref": id,
            "document_id": document_id,
        }));
    }
    if let Some(existing) = find_pbi_by_document_id(repo, &document_id) {
        let rel = existing
            .strip_prefix(repo)
            .unwrap_or(&existing)
            .to_string_lossy()
            .replace('\\', "/");
        return Ok(json!({
            "success": true,
            "status": "already_open_or_done",
            "target_path": rel,
            "document_id": document_id,
        }));
    }

    let filename = format!("[KAIZEN] CI crónica — {slug}.md");
    let uuid = Uuid::new_v4().to_string();
    let body = build_pbi_body(
        &document_id,
        &uuid,
        &job_name,
        &workflow_name,
        &failure_count,
        &quota_limit,
        &head_sha,
        &sample_html_url,
        &sample_check_run_id,
        &repository,
    );
    let created = super::tracker_lifecycle_port::invoke(
        repo,
        inputs,
        json!({
            "operation": "create_issue",
            "title": filename,
            "description": body,
        }),
        "materialize-ci-chronic-failure-pbi",
    )?;
    let rel = created
        .get("artifact_path")
        .and_then(|v| v.as_str())
        .unwrap_or(filename.as_str())
        .to_string();

    Ok(json!({
        "success": true,
        "status": "materialized",
        "target_path": rel,
        "document_id": document_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn inputs() -> Value {
        json!({
            "job_name": "sddia-index-integrity",
            "workflow_name": "sddia-index-qa",
            "failure_count": 3,
            "quota_limit": 3,
            "sample_check_run_id": 4242,
            "sample_html_url": "https://github.com/racso80es/SddIA/runs/4242",
            "repository": "racso80es/SddIA",
            "head_sha": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        })
    }

    #[test]
    fn slugify_job_name_normalizes() {
        assert_eq!(slugify_job_name("sddia-index-integrity"), "sddia-index-integrity");
        assert_eq!(slugify_job_name("Foo Bar!!"), "foo-bar");
        assert_eq!(slugify_job_name("   "), "job");
    }

    #[test]
    fn ci_chronic_materialize_creates_and_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let first = run(repo, &inputs()).expect("create");
        assert_eq!(first["success"], true);
        assert_eq!(first["status"], "materialized");
        let path = repo.join(first["target_path"].as_str().unwrap());
        assert!(path.is_file());
        let raw = fs::read_to_string(&path).unwrap();
        assert!(raw.contains("PBI-KAIZEN-CI-CHRONIC-SDDIA-INDEX-INTEGRITY"));
        assert!(!raw.contains("PENDING_AUDIT_DOC_"));
        assert!(!raw.contains("fractura sistémica"));

        let second = run(repo, &inputs()).expect("idem");
        assert_eq!(second["status"], "already_open_or_done");
        let pending = repo.join("docs/todos/pending");
        let count = fs::read_dir(&pending).unwrap().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn ci_chronic_materialize_idempotent_in_done() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("docs/todos/done")).unwrap();
        fs::write(
            repo.join("docs/todos/done/archived.md"),
            "---\ndocument_id: PBI-KAIZEN-CI-CHRONIC-SDDIA-INDEX-INTEGRITY\n---\n",
        )
        .unwrap();
        let out = run(repo, &inputs()).expect("done");
        assert_eq!(out["status"], "already_open_or_done");
        assert!(!repo.join("docs/todos/pending").exists() || {
            fs::read_dir(repo.join("docs/todos/pending"))
                .map(|rd| rd.count() == 0)
                .unwrap_or(true)
        });
    }
}
