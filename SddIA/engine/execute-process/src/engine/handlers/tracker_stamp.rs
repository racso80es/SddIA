//! Handler `tracker-stamp` — sello Linear ante eventos de dominio (fail-soft D5).

use super::super::actions::try_run_native;
use super::super::capsules::invoke_tool;
use super::super::project_binding::{self, TrackerConfig};
use crate::envelope::OrchestratorEnvelope;
use sddia_io::outbound_lab::lab_mock_outbound_enabled;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_event(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let rel = str_field(inputs, "event_file_path").ok_or("event_file_path requerido")?;
    let path = repo.join(rel.trim_start_matches("./"));
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn tracker_for_payload(repo: &Path, payload: &Value) -> Result<Option<TrackerConfig>, String> {
    let mut inputs = json!({});
    if let Some(slug) = str_field(payload, "project_slug") {
        inputs["project_slug"] = json!(slug);
    }
    project_binding::tracker_config_for_inputs(repo, &inputs)
}

fn emit_sync_failed(
    repo: &Path,
    source: &str,
    issue_ref: &str,
    operation: &str,
    error_code: &str,
    payload: &Value,
    target_state: Option<&str>,
    comment_kind: Option<&str>,
) {
    let mut emit_in = json!({
        "issue_ref": issue_ref,
        "operation": operation,
        "error_code": error_code,
        "source_process": source,
    });
    if let Some(ts) = target_state {
        emit_in["target_state"] = json!(ts);
    }
    if let Some(ck) = comment_kind {
        emit_in["comment_kind"] = json!(ck);
    }
    for key in ["project_slug", "pr_url", "commit_sha"] {
        if let Some(v) = str_field(payload, key) {
            emit_in[key] = json!(v);
        }
    }
    let _ = try_run_native(repo, "emit-tracker-sync-failed", &emit_in);
}

fn tool_update_state(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    canonical: &str,
) -> Result<(), String> {
    let state_name = tc
        .state_map
        .get(canonical)
        .cloned()
        .ok_or_else(|| format!("state_map sin {canonical}"))?;
    invoke_tool(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "update_issue_state",
                "issue_ref": issue_ref,
                "state_name": state_name,
                "team_key": tc.team_key,
            }
        }),
    )?;
    Ok(())
}

fn tool_comment(repo: &Path, issue_ref: &str, body: &str) -> Result<(), String> {
    invoke_tool(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "create_comment",
                "issue_ref": issue_ref,
                "body": body,
            }
        }),
    )?;
    Ok(())
}

fn stamp_event(repo: &Path, event: &Value) -> Result<(bool, Option<String>), String> {
    let event_type = str_field(event, "event_type").unwrap_or_default();
    let payload = event
        .get("payload")
        .and_then(|p| p.as_object())
        .ok_or("payload inválido")?;
    let payload_val = Value::Object(payload.clone());

    let tracker_ref = str_field(&payload_val, "tracker_ref");
    if tracker_ref.is_none() {
        return Ok((false, Some("no-op: sin tracker_ref".into())));
    }
    let issue_ref = tracker_ref.unwrap();

    let tc = match tracker_for_payload(repo, &payload_val)? {
        Some(t) => t,
        None if lab_mock_outbound_enabled() => {
            let team = issue_ref.split('-').next().unwrap_or("BX").to_string();
            let mut map = std::collections::BTreeMap::new();
            map.insert("backlog".into(), "Backlog".into());
            map.insert("in_progress".into(), "In Progress".into());
            map.insert("in_review".into(), "In Review".into());
            map.insert("done".into(), "Done".into());
            project_binding::TrackerConfig {
                team_key: team,
                project_id: None,
                state_map: map,
                label_hu: "hu".into(),
                label_pbi: "pbi".into(),
            }
        }
        None => return Ok((false, Some("no-op: proyecto sin tracker".into()))),
    };

    let source = "tracker-stamp";
    let mut stamped = false;

    let run = |repo: &Path,
               op: &str,
               target: Option<&str>,
               kind: Option<&str>,
               f: &dyn Fn() -> Result<(), String>| {
        if let Err(e) = f() {
            let code = if e.contains("LINEAR_") {
                e.split(':').next().unwrap_or("LINEAR_GRAPHQL_ERROR").to_string()
            } else {
                "LINEAR_GRAPHQL_ERROR".into()
            };
            emit_sync_failed(repo, source, &issue_ref, op, &code, &payload_val, target, kind);
            return Err(format!("warn: {e}"));
        }
        Ok(())
    };

    match event_type.as_str() {
        "PBI_Forged" => {
            run(
                repo,
                "update_issue_state",
                Some("backlog"),
                None,
                &|| tool_update_state(repo, &tc, &issue_ref, "backlog"),
            )?;
            let doc = str_field(&payload_val, "document_id").unwrap_or_default();
            run(
                repo,
                "create_comment",
                None,
                Some("pbi-forged"),
                &|| tool_comment(repo, &issue_ref, &format!("PBI forjado: {doc}")),
            )?;
            stamped = true;
        }
        "Work_Initiated" => {
            run(
                repo,
                "update_issue_state",
                Some("in_progress"),
                None,
                &|| tool_update_state(repo, &tc, &issue_ref, "in_progress"),
            )?;
            let branch = str_field(&payload_val, "branch").unwrap_or_default();
            let persist = str_field(&payload_val, "persist_ref").unwrap_or_default();
            run(
                repo,
                "create_comment",
                None,
                Some("work-initiated"),
                &|| {
                    tool_comment(
                        repo,
                        &issue_ref,
                        &format!("Work iniciado — rama `{branch}` · persist `{persist}`"),
                    )
                },
            )?;
            stamped = true;
        }
        "PullRequest_Presented" => {
            run(
                repo,
                "update_issue_state",
                Some("in_review"),
                None,
                &|| tool_update_state(repo, &tc, &issue_ref, "in_review"),
            )?;
            let pr = str_field(&payload_val, "pr_url").unwrap_or_default();
            run(
                repo,
                "create_comment",
                None,
                Some("pr-presented"),
                &|| tool_comment(repo, &issue_ref, &format!("PR presentado: {pr}")),
            )?;
            stamped = true;
        }
        "PullRequest_Audited" => {
            let resolution = str_field(&payload_val, "resolution").unwrap_or_else(|| "UNKNOWN".into());
            run(
                repo,
                "create_comment",
                None,
                Some("pr-audited"),
                &|| tool_comment(repo, &issue_ref, &format!("Auditoría PR: {resolution}")),
            )?;
            stamped = true;
        }
        "PullRequest_Merged" => {
            run(
                repo,
                "update_issue_state",
                Some("done"),
                None,
                &|| tool_update_state(repo, &tc, &issue_ref, "done"),
            )?;
            let merge = str_field(&payload_val, "merge_commit_hash")
                .or_else(|| str_field(&payload_val, "hash_signature"))
                .unwrap_or_default();
            run(
                repo,
                "create_comment",
                None,
                Some("pr-merged"),
                &|| tool_comment(repo, &issue_ref, &format!("Merge: `{merge}`")),
            )?;
            stamped = true;
        }
        other => return Ok((false, Some(format!("no-op: event_type {other}")))),
    }

    Ok((stamped, None))
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event = load_event(repo, process_inputs)?;
    match stamp_event(repo, &event) {
        Ok((stamped, note)) => Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "stamped": stamped,
                "note": note,
            })),
            error: note.as_ref().filter(|n| n.starts_with("warn:")).cloned(),
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Sello Linear",
                    "status": "executed",
                    "handler": "tracker-stamp",
                }]
            })),
            exit_code: 0,
        }),
        Err(warn) if warn.starts_with("warn:") => Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({ "stamped": false, "warn": warn })),
            error: Some(warn),
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Sello Linear",
                    "status": "warn",
                    "handler": "tracker-stamp",
                }]
            })),
            exit_code: 0,
        }),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_op_without_tracker_ref() {
        let event = json!({
            "event_type": "Work_Initiated",
            "payload": { "branch": "feat/x" }
        });
        let repo = std::path::Path::new("/tmp");
        let (stamped, note) = stamp_event(repo, &event).unwrap_or((false, None));
        assert!(!stamped);
        assert!(note.unwrap_or_default().contains("tracker_ref"));
    }
}
