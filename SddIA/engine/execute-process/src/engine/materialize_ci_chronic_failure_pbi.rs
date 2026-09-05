//! Handler nativo `materialize-ci-chronic-failure-pbi` — PBI Kaizen por cuota CI crónica.

use serde_json::{json, Value};
use std::fs;
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

fn document_id_in_frontmatter(raw: &str, want: &str) -> bool {
    let Some(rest) = raw.strip_prefix("---") else {
        return false;
    };
    let Some(end) = rest.find("\n---") else {
        return false;
    };
    for line in rest[..end].lines() {
        let trimmed = line.trim();
        if let Some(val) = trimmed.strip_prefix("document_id:") {
            let got = val.trim().trim_matches('"').trim_matches('\'');
            if got == want {
                return true;
            }
        }
    }
    false
}

fn find_pbi_by_document_id(repo: &Path, document_id: &str) -> Option<PathBuf> {
    for dir in ["docs/todos/pending", "docs/todos/done"] {
        let path = repo.join(dir);
        if !path.is_dir() {
            continue;
        }
        let Ok(rd) = fs::read_dir(&path) else {
            continue;
        };
        let mut files: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
        files.sort();
        for p in files {
            if p.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let Ok(raw) = fs::read_to_string(&p) else {
                continue;
            };
            if document_id_in_frontmatter(&raw, document_id) {
                return Some(p);
            }
        }
    }
    None
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

    let pending = repo.join("docs/todos/pending");
    fs::create_dir_all(&pending).map_err(|e| e.to_string())?;
    let filename = format!("[KAIZEN] CI crónica — {slug}.md");
    let target = pending.join(&filename);
    let rel = target
        .strip_prefix(repo)
        .unwrap_or(&target)
        .to_string_lossy()
        .replace('\\', "/");

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
    fs::write(&target, body).map_err(|e| e.to_string())?;

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
