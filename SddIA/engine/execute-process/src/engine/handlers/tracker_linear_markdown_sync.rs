//! Proceso `tracker-description-sync` — volcado de cuerpo a la descripción del ítem.

use crate::envelope::OrchestratorEnvelope;
use std::path::Path;
use regex::Regex;
use serde_json::{json, Value};
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

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let tc = super::super::project_binding::tracker_config_for_inputs(repo, process_inputs)?;
    if let (Some(issue_ref), Some(body)) = (
        str_field(process_inputs, "tracker_ref").or_else(|| str_field(process_inputs, "issue_ref")),
        str_field(process_inputs, "body"),
    ) {
        let description = scrub_secrets(&body);
        let tool_out = super::super::tracker_lifecycle_port::invoke(
            repo,
            process_inputs,
            json!({
                "operation": "update_issue_description",
                "issue_ref": issue_ref,
                "description": description,
                "project_id": tc.as_ref().and_then(|c| c.project_id.clone()),
            }),
            "tracker-description-sync",
        )?;
        return Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "synced_count": 1,
                "synced": [{ "issue_ref": issue_ref, "tool": tool_out }],
                "skipped": [],
            })),
            error: None,
            execution_report: None,
            exit_code: 0,
        });
    }
    let mut synced = Vec::new();
    let mut skipped = Vec::new();
    let project_id = tc.as_ref().and_then(|c| c.project_id.clone());
    let team_key = tc.as_ref().map(|c| c.team_key.clone());

    let push_one = |issue_ref: &str,
                    synced: &mut Vec<Value>,
                    skipped: &mut Vec<Value>|
     -> Result<(), String> {
        let fetched = super::super::tracker_lifecycle_port::invoke(
            repo,
            process_inputs,
            json!({
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_id": project_id,
            }),
            "tracker-description-sync",
        );
        let fetched = match fetched {
            Ok(v) => v,
            Err(e) => {
                skipped.push(json!({"issue_ref": issue_ref, "error": e}));
                return Ok(());
            }
        };
        let Some(desc) = fetched.get("description").and_then(|v| v.as_str()) else {
            skipped.push(json!({"issue_ref": issue_ref, "error": "sin description"}));
            return Ok(());
        };
        let description = scrub_secrets(desc);
        match super::super::tracker_lifecycle_port::invoke(
            repo,
            process_inputs,
            json!({
                "operation": "update_issue_description",
                "issue_ref": issue_ref,
                "description": description,
                "project_id": project_id,
            }),
            "tracker-description-sync",
        ) {
            Ok(tool) => synced.push(json!({"issue_ref": issue_ref, "tool": tool})),
            Err(e) => skipped.push(json!({"issue_ref": issue_ref, "error": e})),
        }
        Ok(())
    };

    if let Some(issue_ref) = str_field(process_inputs, "issue_ref")
        .or_else(|| str_field(process_inputs, "markdown_path"))
        .or_else(|| str_field(process_inputs, "tracker_ref"))
    {
        push_one(&issue_ref, &mut synced, &mut skipped)?;
    } else {
        let sync_all = process_inputs
            .get("sync_all")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        if !sync_all {
            return Err("sync_all=false sin tracker_ref".into());
        }
        let listed = super::super::tracker_lifecycle_port::invoke(
            repo,
            process_inputs,
            json!({
                "operation": "list_issues",
                "team_key": team_key,
                "project_id": project_id,
                "kind": "all",
            }),
            "tracker-description-sync",
        )?;
        if let Some(items) = listed.get("items").and_then(|v| v.as_array()) {
            for item in items {
                let Some(issue_ref) = item
                    .get("issue_ref")
                    .or_else(|| item.get("identifier"))
                    .or_else(|| item.get("path"))
                    .and_then(|v| v.as_str())
                else {
                    continue;
                };
                push_one(issue_ref, &mut synced, &mut skipped)?;
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
                "phase_name": "Volcado de descripción",
                "status": "executed",
                "handler": "tracker-description-sync",
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
