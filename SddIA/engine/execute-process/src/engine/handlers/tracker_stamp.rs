//! Handler `tracker-stamp` — sello Linear ante eventos de dominio (fail-soft D5).

use super::super::actions::try_run_native;
use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{self, TrackerConfig};
use super::super::tracker_pbi_meta;
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

fn lab_tracker_fallback(issue_ref: &str) -> TrackerConfig {
    let team_key = issue_ref
        .split('-')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("BX")
        .to_string();
    let mut state_map = std::collections::BTreeMap::new();
    state_map.insert("backlog".into(), "Backlog".into());
    state_map.insert("in_progress".into(), "In Progress".into());
    state_map.insert("in_review".into(), "In Review".into());
    state_map.insert("done".into(), "Done".into());
    project_binding::TrackerConfig {
        team_key,
        project_id: None,
        state_map,
        label_hu: "hu".into(),
        label_pbi: "pbi".into(),
    }
}

fn tracker_for_payload(repo: &Path, payload: &Value, issue_ref: &str) -> Result<Option<TrackerConfig>, String> {
    let mut inputs = json!({});
    if let Some(slug) = str_field(payload, "project_slug") {
        inputs["project_slug"] = json!(slug);
    }
    match project_binding::tracker_config_for_inputs(repo, &inputs)? {
        Some(t) => Ok(Some(t)),
        None if lab_mock_outbound_enabled() => Ok(Some(lab_tracker_fallback(issue_ref))),
        None => Ok(None),
    }
}

fn linear_to_canonical(linear_state: &str, tc: &TrackerConfig) -> String {
    for (canonical, name) in &tc.state_map {
        if name.eq_ignore_ascii_case(linear_state) {
            return canonical.clone();
        }
    }
    linear_state.to_string()
}

fn labels_contains(fetch: &Value, label: &str) -> bool {
    fetch
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .any(|n| n.eq_ignore_ascii_case(label))
        })
        .unwrap_or(false)
}

fn expected_hu_for_payload(repo: &Path, payload: &Value) -> Option<String> {
    if let Some(h) = str_field(payload, "expected_hu_tracker_ref") {
        return Some(h);
    }
    if let Some(persist) = str_field(payload, "persist_ref") {
        if let Some(pbi) = tracker_pbi_meta::resolve_pbi_path_from_persist(repo, &persist) {
            return tracker_pbi_meta::expected_hu_tracker_ref_from_pbi(repo, &pbi);
        }
    }
    None
}

fn tool_fetch_issue(repo: &Path, issue_ref: &str) -> Result<Value, String> {
    invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
            }
        }),
        Some("tracker-stamp"),
    )
}

fn preflight_pbi(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    payload: &Value,
) -> Result<Value, String> {
    let fetch = tool_fetch_issue(repo, issue_ref)?;
    if !labels_contains(&fetch, &tc.label_pbi) {
        return Err("warn: AC-18 — issue sin label PBI".into());
    }
    if let Some(expected) = expected_hu_for_payload(repo, payload) {
        let parent = fetch.get("parent").and_then(|v| v.as_str());
        if parent != Some(expected.as_str()) {
            return Err("warn: AC-18 — parent no coincide con HU esperada".into());
        }
    }
    Ok(fetch)
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
    invoke_tool_for_process(
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
        Some("tracker-stamp"),
    )?;
    Ok(())
}

fn tool_comment(repo: &Path, issue_ref: &str, body: &str) -> Result<(), String> {
    invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "create_comment",
                "issue_ref": issue_ref,
                "body": body,
            }
        }),
        Some("tracker-stamp"),
    )?;
    Ok(())
}

fn maybe_promote_hu_from_backlog(
    repo: &Path,
    tc: &TrackerConfig,
    pbi_fetch: &Value,
) -> Result<(), String> {
    let parent = pbi_fetch
        .get("parent")
        .and_then(|v| v.as_str())
        .ok_or("parent ausente en PBI")?;
    let hu_fetch = tool_fetch_issue(repo, parent)?;
    let hu_state = linear_to_canonical(
        hu_fetch.get("state").and_then(|v| v.as_str()).unwrap_or(""),
        tc,
    );
    if hu_state == "backlog" {
        tool_update_state(repo, tc, parent, "in_progress")?;
    }
    Ok(())
}

fn maybe_complete_hu_when_children_done(
    repo: &Path,
    tc: &TrackerConfig,
    hu_ref: &str,
) -> Result<(), String> {
    let hu_fetch = tool_fetch_issue(repo, hu_ref)?;
    let children = hu_fetch
        .get("children")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if children.is_empty() {
        return Ok(());
    }
    for child in children {
        let child_ref = child
            .as_str()
            .or_else(|| child.get("identifier").and_then(|v| v.as_str()))
            .ok_or("child identifier inválido")?;
        let cf = tool_fetch_issue(repo, child_ref)?;
        let st = linear_to_canonical(cf.get("state").and_then(|v| v.as_str()).unwrap_or(""), tc);
        if st != "done" {
            return Ok(());
        }
    }
    tool_update_state(repo, tc, hu_ref, "done")?;
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

    let tc = match tracker_for_payload(repo, &payload_val, &issue_ref)? {
        Some(t) => t,
        None => return Ok((false, Some("no-op: proyecto sin tracker".into()))),
    };

    let source = "tracker-stamp";
    let mut stamped = false;

    let pbi_fetch = match preflight_pbi(repo, &tc, &issue_ref, &payload_val) {
        Ok(f) => f,
        Err(w) if w.starts_with("warn:") => return Ok((false, Some(w))),
        Err(e) => return Err(e),
    };

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
            if let Err(e) = maybe_promote_hu_from_backlog(repo, &tc, &pbi_fetch) {
                if !e.starts_with("parent ausente") {
                    emit_sync_failed(
                        repo,
                        source,
                        &issue_ref,
                        "update_issue_state",
                        "LINEAR_GRAPHQL_ERROR",
                        &payload_val,
                        Some("in_progress"),
                        None,
                    );
                    return Err(format!("warn: {e}"));
                }
            }
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
            let resolution =
                str_field(&payload_val, "resolution").unwrap_or_else(|| "UNKNOWN".into());
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
            if let Some(hu_ref) = pbi_fetch.get("parent").and_then(|v| v.as_str()) {
                if let Err(e) = maybe_complete_hu_when_children_done(repo, &tc, hu_ref) {
                    emit_sync_failed(
                        repo,
                        source,
                        hu_ref,
                        "update_issue_state",
                        "LINEAR_GRAPHQL_ERROR",
                        &payload_val,
                        Some("done"),
                        None,
                    );
                    return Err(format!("warn: {e}"));
                }
            }
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
    use crate::core::repo::find_repo_root;
    use std::sync::{Mutex, OnceLock};

    fn lab_guard() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    fn with_lab<F: FnOnce()>(f: F) {
        let _g = lab_guard();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::remove_var("LINEAR_API_TOKEN");
        f();
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
    }

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

    #[test]
    fn ac18_bad_label_no_writes() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "Work_Initiated",
                "payload": {
                    "tracker_ref": "LAB-PBI-BADLABEL",
                    "branch": "feat/x",
                    "persist_ref": "docs/features/demo",
                }
            });
            let (stamped, note) = stamp_event(&repo, &event).unwrap();
            assert!(!stamped);
            assert!(note.unwrap_or_default().contains("AC-18"));
        });
    }

    #[test]
    fn ac18_bad_parent_no_writes() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "Work_Initiated",
                "payload": {
                    "tracker_ref": "LAB-PBI-BADPARENT",
                    "branch": "feat/x",
                    "expected_hu_tracker_ref": "LAB-HU-1",
                }
            });
            let (stamped, note) = stamp_event(&repo, &event).unwrap();
            assert!(!stamped);
            assert!(note.unwrap_or_default().contains("AC-18"));
        });
    }

    #[test]
    fn lab_cycle_work_to_merged_stamps() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            for (etype, extra) in [
                (
                    "Work_Initiated",
                    json!({
                        "tracker_ref": "LAB-PBI-CYCLE",
                        "branch": "feat/lab",
                        "expected_hu_tracker_ref": "LAB-HU-1",
                    }),
                ),
                (
                    "PullRequest_Presented",
                    json!({
                        "tracker_ref": "LAB-PBI-CYCLE",
                        "branch": "feat/lab",
                        "pr_url": "https://github.com/o/r/pull/1",
                    }),
                ),
                (
                    "PullRequest_Merged",
                    json!({
                        "tracker_ref": "LAB-PBI-MERGE-LAST",
                        "merge_commit_hash": "abc123deadbeef",
                    }),
                ),
            ] {
                let event = json!({ "event_type": etype, "payload": extra });
                let (stamped, _) = stamp_event(&repo, &event).unwrap();
                assert!(stamped, "expected stamp for {etype}");
            }
        });
    }
}
