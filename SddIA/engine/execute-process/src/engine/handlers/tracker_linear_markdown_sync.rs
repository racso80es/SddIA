//! Proceso `tracker-linear-markdown-sync` — volcado de cuerpo Markdown a descripción Linear.

use super::super::capsules::invoke_tool_for_process;
use crate::core::parser::load_frontmatter_yaml;
use crate::envelope::OrchestratorEnvelope;
use regex::Regex;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn scrub_secrets(raw: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(
            r"(?m)^(?P<k>(?i:LINEAR_API_TOKEN|LINEAR_API_KEY|GEMINI_API_KEY|CURSOR_API_KEY|SDDIA_EMAIL_IMAP_SECRET|GH_TOKEN|GITHUB_TOKEN))\s*=\s*.*$",
        )
        .expect("regex")
    });
    let mut out = re
        .replace_all(raw, |caps: &regex::Captures| {
            format!("{}=[REDACTED]", &caps["k"])
        })
        .into_owned();
    static LIN: OnceLock<Regex> = OnceLock::new();
    let lin = LIN.get_or_init(|| Regex::new(r"lin_api_[A-Za-z0-9]+").expect("regex"));
    out = lin.replace_all(&out, "lin_api_[REDACTED]").into_owned();
    out
}

fn tracker_ref_from_md(path: &Path) -> Option<String> {
    let fm = load_frontmatter_yaml(path).ok()?;
    fm.get("tracker_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn scan_dirs(repo: &Path) -> Vec<PathBuf> {
    let rels = [
        "docs/todos/pending",
        "docs/todos/done",
        "Documentacion/PBI/Realizado",
    ];
    let mut out = Vec::new();
    for rel in rels {
        let dir = repo.join(rel);
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in rd.filter_map(|e| e.ok()) {
            let p = ent.path();
            if p.extension().and_then(|x| x.to_str()) == Some("md") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn sync_one(repo: &Path, md_path: &Path, issue_ref: &str) -> Result<Value, String> {
    let raw = fs::read_to_string(md_path).map_err(|e| e.to_string())?;
    let description = scrub_secrets(&raw);
    let rel = md_path
        .strip_prefix(repo)
        .unwrap_or(md_path)
        .to_string_lossy()
        .replace('\\', "/");
    let tool_out = invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "update_issue_description",
                "issue_ref": issue_ref,
                "description": description,
            }
        }),
        Some("tracker-linear-markdown-sync"),
    )?;
    Ok(json!({
        "markdown_path": rel,
        "issue_ref": issue_ref,
        "description_bytes": description.len(),
        "tool": tool_out,
    }))
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let mut synced = Vec::new();
    let mut skipped = Vec::new();

    if let (Some(md_rel), Some(issue_ref)) = (
        str_field(process_inputs, "markdown_path"),
        str_field(process_inputs, "issue_ref"),
    ) {
        let md_path = repo.join(md_rel.trim_start_matches("./"));
        if !md_path.is_file() {
            return Err(format!("markdown_path no encontrado: {}", md_path.display()));
        }
        synced.push(sync_one(repo, &md_path, &issue_ref)?);
    } else {
        let sync_all = process_inputs
            .get("sync_all")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        if !sync_all {
            return Err("sync_all=false sin markdown_path+issue_ref".into());
        }
        for md in scan_dirs(repo) {
            let Some(issue_ref) = tracker_ref_from_md(&md) else {
                continue;
            };
            match sync_one(repo, &md, &issue_ref) {
                Ok(v) => synced.push(v),
                Err(e) => skipped.push(json!({
                    "markdown_path": md.strip_prefix(repo).unwrap_or(&md).to_string_lossy().replace('\\', "/"),
                    "issue_ref": issue_ref,
                    "error": e,
                })),
            }
        }
    }

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "synced_count": synced.len(),
            "skipped_count": skipped.len(),
            "synced": synced,
            "skipped": skipped,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [{
                "phase_name": "Volcado Markdown → Linear",
                "status": "executed",
                "handler": "tracker-linear-markdown-sync",
            }]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrub_redacts_env_lines() {
        let raw = "LINEAR_API_TOKEN=secret\nbody\nlin_api_abc123\n";
        let s = scrub_secrets(raw);
        assert!(!s.contains("secret"));
        assert!(!s.contains("abc123"));
        assert!(s.contains("[REDACTED]"));
    }
}
