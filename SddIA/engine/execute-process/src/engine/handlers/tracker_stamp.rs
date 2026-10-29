//! Handler `tracker-stamp` — sello de ciclo ante eventos de dominio (fail-soft D5).

use super::super::actions::try_run_native;
use super::super::capsules::invoke_tool_for_process;
use super::super::done_gate;
use super::super::project_binding::{self, DoneGate, TrackerConfig};
use super::super::tracker_pbi_meta;
use super::tracker_sync_replay::should_discard_transition;
use crate::envelope::OrchestratorEnvelope;
use sddia_io::outbound_lab::lab_mock_outbound_enabled;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn file_sink(repo: &Path, payload: &Value) -> Result<bool, String> {
    Ok(matches!(
        super::super::tracker_lifecycle_port::resolve_target(repo, payload)?,
        super::super::tracker_lifecycle_port::LifecycleTarget::File
    ))
}

pub(crate) fn lifecycle_tool_name_for(repo: &Path, inputs: &Value) -> Result<String, String> {
    match super::super::tracker_lifecycle_port::resolve_target(repo, inputs)? {
        super::super::tracker_lifecycle_port::LifecycleTarget::Remote { tool } => Ok(tool),
        super::super::tracker_lifecycle_port::LifecycleTarget::File
        | super::super::tracker_lifecycle_port::LifecycleTarget::Unconfigured => {
            Ok("file-tracker-adapter".into())
        }
    }
}

pub(crate) fn lifecycle_tool_name(repo: &Path) -> Result<String, String> {
    let phase = json!({
        "requires_capability": [{
            "id": "tracker:lifecycle",
            "contract": "tracker.lifecycle",
            "version": ">=1.0.0"
        }]
    });
    let bindings = super::super::capability_di_resolver::resolve_phase_bindings(repo, &phase)
        .map_err(|e| e.message)?;
    let provider = bindings
        .first()
        .map(|b| b.provider.clone())
        .ok_or_else(|| "sin fila tracker:lifecycle".to_string())?;
    let name = provider
        .split_once(':')
        .map(|(_, n)| n)
        .unwrap_or(provider.as_str())
        .trim();
    if name.is_empty() {
        return Err("provider tracker:lifecycle vacío".into());
    }
    Ok(name.to_string())
}

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn load_event(repo: &Path, inputs: &Value) -> Result<Value, String> {
    super::super::ecst_input_loader::load_ecst_from_inputs(repo, inputs)
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
    state_map.insert("todo".into(), "Todo".into());
    state_map.insert("in_progress".into(), "In Progress".into());
    state_map.insert("in_review".into(), "In Review".into());
    state_map.insert("done".into(), "Done".into());
    state_map.insert("cancelled".into(), "Cancelled".into());
    project_binding::TrackerConfig {
        team_key,
        project_id: None,
        state_map,
        label_hu: "hu".into(),
        label_pbi: "pbi".into(),
        done_gate: DoneGate::Git,
    }
}

fn tracker_for_payload(repo: &Path, payload: &Value, issue_ref: &str) -> Result<Option<TrackerConfig>, String> {
    let mut inputs = json!({});
    if let Some(slug) = str_field(payload, "project_slug") {
        inputs["project_slug"] = json!(slug);
    }
    match super::super::project_binding::tracker_config_for_inputs(repo, &inputs)? {
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

fn tool_fetch_issue(repo: &Path, issue_ref: &str, project_id: Option<&str>) -> Result<Value, String> {
    invoke_tool_for_process(
        repo,
        &lifecycle_tool_name(repo)?,
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_id": project_id,
            }
        }),
        Some("tracker-stamp"),
    )
}

fn tool_update_description(
    repo: &Path,
    issue_ref: &str,
    description: &str,
    tc: &TrackerConfig,
    project_slug: Option<&str>,
) -> Result<(), String> {
    let mut req = json!({
        "operation": "update_issue_description",
        "issue_ref": issue_ref,
        "description": description,
    });
    if let Some(pid) = tc.project_id.as_deref() {
        req["project_id"] = json!(pid);
    }
    if let Some(slug) = project_slug {
        req["project_slug"] = json!(slug);
    }
    invoke_tool_for_process(
        repo,
        &lifecycle_tool_name(repo)?,
        &json!({ "request": req }),
        Some("tracker-stamp"),
    )
    .map(|_| ())
}

fn sync_item_tracker_column_remote(
    repo: &Path,
    payload: &Value,
    issue_ref: &str,
    column: &str,
    tc: &TrackerConfig,
) -> Result<(), String> {
    let fetch = tool_fetch_issue(repo, issue_ref, tc.project_id.as_deref())?;
    let desc = fetch
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if desc.trim().is_empty() {
        return Ok(());
    }
    let patched = super::super::item_lifecycle_frontmatter::seal_markdown(
        desc,
        &super::super::item_lifecycle_frontmatter::yaml_patch_tracker_column(column),
    )?;
    if patched == desc {
        return Ok(());
    }
    tool_update_description(
        repo,
        issue_ref,
        &patched,
        tc,
        str_field(payload, "project_slug").as_deref(),
    )
}

fn patch_local_markdown_column(path: &Path, column: &str) -> Result<(), String> {
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let patched = super::super::item_lifecycle_frontmatter::seal_markdown(
        &raw,
        &super::super::item_lifecycle_frontmatter::yaml_patch_tracker_column(column),
    )?;
    fs::write(path, patched).map_err(|e| e.to_string())
}

fn resolve_local_item_path(repo: &Path, payload: &Value) -> Option<PathBuf> {
    if let Some(pbi) = str_field(payload, "pbi_ref") {
        if let Some(slug) = str_field(payload, "project_slug") {
            if let Ok(Some(bound)) =
                project_binding::bind(repo, &json!({ "project_slug": slug }))
            {
                if let Ok(path) = project_binding::resolve_doc_path(&bound.project_root, &pbi) {
                    if path.is_file() {
                        return Some(path);
                    }
                }
            }
        }
        let direct = repo.join(pbi.trim_start_matches("./"));
        if direct.is_file() {
            return Some(direct);
        }
    }
    if let Some(tr) = str_field(payload, "tracker_ref") {
        if let Some(path) =
            super::tracker_markdown_apply::resolve_markdown_by_tracker_ref(repo, &tr)
        {
            if path.is_file() {
                return Some(path);
            }
        }
    }
    if let Some(hu) = str_field(payload, "hu_ref") {
        let direct = repo.join(hu.trim_start_matches("./"));
        if direct.is_file() {
            return Some(direct);
        }
    }
    if let Some(persist) = str_field(payload, "persist_ref") {
        if let Some(pbi) = tracker_pbi_meta::resolve_pbi_path_from_persist(repo, &persist) {
            let direct = repo.join(pbi.trim_start_matches("./"));
            if direct.is_file() {
                return Some(direct);
            }
        }
    }
    None
}

fn stamp_file_sink_event(
    repo: &Path,
    event_type: &str,
    payload: &Value,
) -> Result<(bool, Option<String>), String> {
    use super::super::item_lifecycle_frontmatter::{
        COLUMN_BACKLOG, COLUMN_CANCELLED, COLUMN_DONE, COLUMN_IN_PROGRESS, COLUMN_IN_REVIEW,
        COLUMN_TODO,
    };
    let column = match event_type {
        "PBI_Forged" => COLUMN_BACKLOG,
        "PBI_Refined" | "HU_Refined" => COLUMN_TODO,
        "Work_Initiated" => COLUMN_IN_PROGRESS,
        "PullRequest_Presented" => COLUMN_IN_REVIEW,
        "PullRequest_Merged" | "Delivery_Committed" => COLUMN_DONE,
        "PBI_Cancelled" => COLUMN_CANCELLED,
        _ => return Ok((false, Some(format!("no-op: event_type {event_type} (file)")))),
    };
    let path = resolve_local_item_path(repo, payload).ok_or_else(|| {
        "warn: sumidero file sin ruta de ítem".to_string()
    })?;
    patch_local_markdown_column(&path, column)?;
    Ok((true, None))
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
        &lifecycle_tool_name(repo)?,
        &json!({
            "request": {
                "operation": "update_issue_state",
                "issue_ref": issue_ref,
                "state_name": state_name,
                "team_key": tc.team_key,
                "project_id": tc.project_id,
            }
        }),
        Some("tracker-stamp"),
    )?;
    Ok(())
}

fn tool_comment(repo: &Path, issue_ref: &str, body: &str, project_id: Option<&str>) -> Result<(), String> {
    invoke_tool_for_process(
        repo,
        &lifecycle_tool_name(repo)?,
        &json!({
            "request": {
                "operation": "create_comment",
                "issue_ref": issue_ref,
                "body": body,
                "project_id": project_id,
            }
        }),
        Some("tracker-stamp"),
    )?;
    Ok(())
}

fn maybe_promote_hu_when_work_starts(
    repo: &Path,
    tc: &TrackerConfig,
    pbi_fetch: &Value,
) -> Result<(), String> {
    let parent = pbi_fetch
        .get("parent")
        .and_then(|v| v.as_str())
        .ok_or("parent ausente en PBI")?;
    let hu_fetch = tool_fetch_issue(repo, parent, tc.project_id.as_deref())?;
    let hu_state = linear_to_canonical(
        hu_fetch.get("state").and_then(|v| v.as_str()).unwrap_or(""),
        tc,
    );
    if hu_state == "in_progress" {
        return Ok(());
    }
    if hu_state == "backlog" || hu_state == "todo" {
        tool_update_state(repo, tc, parent, "in_progress")?;
    }
    Ok(())
}

fn update_state_if_not_discarded(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    fetch: &Value,
    canonical: &str,
) -> Result<(), String> {
    let current = linear_to_canonical(
        fetch.get("state").and_then(|v| v.as_str()).unwrap_or(""),
        tc,
    );
    if should_discard_transition(&current, canonical) {
        return Ok(());
    }
    tool_update_state(repo, tc, issue_ref, canonical)
}

fn yaml_field_in_repo(repo: &Path, rel: &str, key: &str) -> Option<String> {
    let path = repo.join(rel.trim_start_matches("./"));
    let text = fs::read_to_string(path).ok()?;
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    for line in rest[..end].lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix(&format!("{key}:")) {
            let v = val.trim().trim_matches('"');
            if !v.is_empty() {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn resolve_issue_ref(payload: &Value, repo: &Path) -> Option<String> {
    if let Some(tr) = str_field(payload, "tracker_ref") {
        return Some(tr);
    }
    if let Some(pbi) = str_field(payload, "pbi_ref") {
        return tracker_pbi_meta::tracker_ref_from_pbi(repo, &pbi);
    }
    None
}

fn archive_cancelled_pbi(repo: &Path, payload: &Value) -> Result<(), String> {
    let pbi_ref = str_field(payload, "pbi_ref").ok_or("pbi_ref requerido para archivar")?;
    let slug = str_field(payload, "project_slug").ok_or("project_slug requerido")?;
    let bound = project_binding::bind(repo, &json!({"project_slug": slug}))?
        .ok_or("proyecto no vinculado")?;
    let src = project_binding::resolve_doc_path(&bound.project_root, &pbi_ref)?;
    if !src.is_file() {
        return Ok(());
    }
    let done_dir =
        project_binding::resolve_doc_path(&bound.project_root, &bound.docs.todos_done)?;
    fs::create_dir_all(&done_dir).map_err(|e| e.to_string())?;
    let file_name = src
        .file_name()
        .ok_or("pbi_ref sin nombre de archivo")?
        .to_string_lossy()
        .to_string();
    let dest = done_dir.join(file_name);
    let raw = fs::read_to_string(&src).map_err(|e| e.to_string())?;
    let patched = if raw.trim_start().starts_with("---") {
        super::super::item_lifecycle_frontmatter::seal_markdown(
            &raw,
            &super::super::item_lifecycle_frontmatter::yaml_patch_cancelled(),
        )?
    } else {
        format!("---\ntracker_column: cancelled\nrefinement: refined\n---\n\n{raw}")
    };
    fs::write(&dest, patched).map_err(|e| e.to_string())?;
    if dest != src {
        let _ = fs::remove_file(&src);
    }
    Ok(())
}

fn maybe_complete_hu_when_children_done(
    repo: &Path,
    tc: &TrackerConfig,
    hu_ref: &str,
) -> Result<(), String> {
    let hu_fetch = tool_fetch_issue(repo, hu_ref, tc.project_id.as_deref())?;
    let children = hu_fetch
        .get("children")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if children.is_empty() {
        return Ok(());
    }
    let mut any_done = false;
    for child in children {
        let child_ref = child
            .as_str()
            .or_else(|| child.get("identifier").and_then(|v| v.as_str()))
            .ok_or("child identifier inválido")?;
        let cf = tool_fetch_issue(repo, child_ref, tc.project_id.as_deref())?;
        let st = linear_to_canonical(cf.get("state").and_then(|v| v.as_str()).unwrap_or(""), tc);
        if st == "done" {
            any_done = true;
            continue;
        }
        if st == "cancelled" {
            continue;
        }
        return Ok(());
    }
    if any_done {
        tool_update_state(repo, tc, hu_ref, "done")?;
    }
    Ok(())
}

fn stamp_done_with_hu_roll_up(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    pbi_fetch: &Value,
    payload: &Value,
    comment: &str,
    source: &str,
    commit_sha: &str,
) -> Result<bool, String> {
    let run = |f: &dyn Fn() -> Result<(), String>| -> Result<(), String> {
        if let Err(e) = f() {
            emit_sync_failed(
                repo,
                source,
                issue_ref,
                "update_issue_state",
                "LINEAR_GRAPHQL_ERROR",
                payload,
                Some("done"),
                None,
            );
            return Err(format!("warn: {e}"));
        }
        Ok(())
    };
    run(&|| tool_update_state(repo, tc, issue_ref, "done"))?;
    if let Some(hu_ref) = pbi_fetch.get("parent").and_then(|v| v.as_str()) {
        if let Err(e) = maybe_complete_hu_when_children_done(repo, tc, hu_ref) {
            emit_sync_failed(
                repo,
                source,
                hu_ref,
                "update_issue_state",
                "LINEAR_GRAPHQL_ERROR",
                payload,
                Some("done"),
                None,
            );
            return Err(format!("warn: {e}"));
        }
    }
    run(&|| tool_comment(repo, issue_ref, comment, tc.project_id.as_deref()))?;
    let _ = done_gate::write_tracker_done_proof(repo, issue_ref, commit_sha, pbi_fetch);
    let _ = sync_item_tracker_column_remote(
        repo,
        payload,
        issue_ref,
        super::super::item_lifecycle_frontmatter::COLUMN_DONE,
        tc,
    );
    Ok(true)
}

fn stamp_todo_transition(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    fetch: &Value,
    payload: &Value,
    source: &str,
    comment: &str,
) -> Result<(bool, Option<String>), String> {
    if tc.state_map.get("todo").is_none() {
        return Ok((false, Some("warn: state_map sin todo".into())));
    }
    let run = |f: &dyn Fn() -> Result<(), String>| -> Result<(), String> {
        if let Err(e) = f() {
            emit_sync_failed(
                repo,
                source,
                issue_ref,
                "update_issue_state",
                "LINEAR_GRAPHQL_ERROR",
                payload,
                Some("todo"),
                None,
            );
            return Err(format!("warn: {e}"));
        }
        Ok(())
    };
    run(&|| update_state_if_not_discarded(repo, tc, issue_ref, fetch, "todo"))?;
    run(&|| tool_comment(repo, issue_ref, comment, tc.project_id.as_deref()))?;
    let _ = sync_item_tracker_column_remote(
        repo,
        payload,
        issue_ref,
        super::super::item_lifecycle_frontmatter::COLUMN_TODO,
        tc,
    );
    Ok((true, None))
}

fn stamp_event(repo: &Path, event: &Value) -> Result<(bool, Option<String>), String> {
    let event_type = str_field(event, "event_type").unwrap_or_default();
    let payload = event
        .get("payload")
        .and_then(|p| p.as_object())
        .ok_or("payload inválido")?;
    let payload_val = Value::Object(payload.clone());
    let source = "tracker-stamp";
    if file_sink(repo, &payload_val)? {
        return stamp_file_sink_event(repo, &event_type, &payload_val);
    }

    if event_type == "HU_Refined" {
        let issue_ref = str_field(&payload_val, "tracker_ref");
        if issue_ref.is_none() {
            return Ok((false, Some("no-op: sin tracker_ref".into())));
        }
        let issue_ref = issue_ref.unwrap();
        let tc = match tracker_for_payload(repo, &payload_val, &issue_ref)? {
            Some(t) => t,
            None => return Ok((false, Some("no-op: proyecto sin tracker".into()))),
        };
        let fetch = tool_fetch_issue(repo, &issue_ref, tc.project_id.as_deref())?;
        let hu_ref = str_field(&payload_val, "hu_ref").unwrap_or_default();
        let doc = yaml_field_in_repo(repo, &hu_ref, "document_id").unwrap_or_default();
        let ver = yaml_field_in_repo(repo, &hu_ref, "version").unwrap_or_else(|| "1.0.0".into());
        let comment = format!("HU refinada: {doc} v{ver}");
        return stamp_todo_transition(repo, &tc, &issue_ref, &fetch, &payload_val, source, &comment);
    }

    if event_type == "PBI_Cancelled" {
        let issue_ref = resolve_issue_ref(&payload_val, repo);
        if issue_ref.is_none() {
            return Ok((false, Some("no-op: sin tracker_ref ni pbi_ref resoluble".into())));
        }
        let issue_ref = issue_ref.unwrap();
        let tc = match tracker_for_payload(repo, &payload_val, &issue_ref)? {
            Some(t) => t,
            None => return Ok((false, Some("no-op: proyecto sin tracker".into()))),
        };
        if tc.state_map.get("cancelled").is_none() {
            return Ok((false, Some("warn: state_map sin cancelled".into())));
        }
        let pbi_fetch = tool_fetch_issue(repo, &issue_ref, tc.project_id.as_deref())?;
        let reason = str_field(&payload_val, "reason").unwrap_or_else(|| "sin motivo".into());
        let run = |f: &dyn Fn() -> Result<(), String>| -> Result<(), String> {
            if let Err(e) = f() {
                emit_sync_failed(
                    repo,
                    source,
                    &issue_ref,
                    "update_issue_state",
                    "LINEAR_GRAPHQL_ERROR",
                    &payload_val,
                    Some("cancelled"),
                    None,
                );
                return Err(format!("warn: {e}"));
            }
            Ok(())
        };
        run(&|| {
            update_state_if_not_discarded(repo, &tc, &issue_ref, &pbi_fetch, "cancelled")
        })?;
        run(&|| tool_comment(repo, &issue_ref, &format!("PBI cancelado: {reason}"), tc.project_id.as_deref()))?;
        let _ = archive_cancelled_pbi(repo, &payload_val);
        return Ok((true, None));
    }

    let issue_ref = resolve_issue_ref(&payload_val, repo);
    if issue_ref.is_none() {
        return Ok((false, Some("no-op: sin tracker_ref".into())));
    }
    let issue_ref = issue_ref.unwrap();

    let tc = match tracker_for_payload(repo, &payload_val, &issue_ref)? {
        Some(t) => t,
        None => return Ok((false, Some("no-op: proyecto sin tracker".into()))),
    };

    let stamped;

    let pbi_fetch = tool_fetch_issue(repo, &issue_ref, tc.project_id.as_deref())?;

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
        "PBI_Refined" => {
            let pbi_ref = str_field(&payload_val, "pbi_ref").unwrap_or_default();
            let doc = yaml_field_in_repo(repo, &pbi_ref, "document_id").unwrap_or_default();
            let ver = yaml_field_in_repo(repo, &pbi_ref, "version").unwrap_or_else(|| "1.0.0".into());
            let comment = format!("PBI refinado: {doc} v{ver}");
            return stamp_todo_transition(
                repo,
                &tc,
                &issue_ref,
                &pbi_fetch,
                &payload_val,
                source,
                &comment,
            );
        }
        "PBI_Forged" => {
            run(
                repo,
                "update_issue_state",
                Some("backlog"),
                None,
                &|| update_state_if_not_discarded(repo, &tc, &issue_ref, &pbi_fetch, "backlog"),
            )?;
            let doc = str_field(&payload_val, "document_id").unwrap_or_default();
            run(
                repo,
                "create_comment",
                None,
                Some("pbi-forged"),
                &|| tool_comment(repo, &issue_ref, &format!("PBI forjado: {doc}"), tc.project_id.as_deref()),
            )?;
            let _ = sync_item_tracker_column_remote(
                repo,
                &payload_val,
                &issue_ref,
                super::super::item_lifecycle_frontmatter::COLUMN_BACKLOG,
                &tc,
            );
            stamped = true;
        }
        "Work_Initiated" => {
            let pbi_state = linear_to_canonical(
                pbi_fetch.get("state").and_then(|v| v.as_str()).unwrap_or(""),
                &tc,
            );
            if pbi_state != "in_progress" {
                run(
                    repo,
                    "update_issue_state",
                    Some("in_progress"),
                    None,
                    &|| tool_update_state(repo, &tc, &issue_ref, "in_progress"),
                )?;
            }
            if let Err(e) = maybe_promote_hu_when_work_starts(repo, &tc, &pbi_fetch) {
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
                        tc.project_id.as_deref(),
                    )
                },
            )?;
            let _ = sync_item_tracker_column_remote(
                repo,
                &payload_val,
                &issue_ref,
                super::super::item_lifecycle_frontmatter::COLUMN_IN_PROGRESS,
                &tc,
            );
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
                &|| tool_comment(repo, &issue_ref, &format!("PR presentado: {pr}"), tc.project_id.as_deref()),
            )?;
            let _ = sync_item_tracker_column_remote(
                repo,
                &payload_val,
                &issue_ref,
                super::super::item_lifecycle_frontmatter::COLUMN_IN_REVIEW,
                &tc,
            );
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
                &|| tool_comment(repo, &issue_ref, &format!("Auditoría PR: {resolution}"), tc.project_id.as_deref()),
            )?;
            stamped = true;
        }
        "PullRequest_Merged" => {
            let merge = str_field(&payload_val, "merge_commit_hash")
                .or_else(|| str_field(&payload_val, "hash_signature"))
                .unwrap_or_default();
            stamped = stamp_done_with_hu_roll_up(
                repo,
                &tc,
                &issue_ref,
                &pbi_fetch,
                &payload_val,
                &format!("Merge: `{merge}`"),
                source,
                &merge,
            )?;
        }
        "Delivery_Committed" => {
            let mode = str_field(&payload_val, "delivery_mode").unwrap_or_default();
            if mode != "trunk_direct" {
                return Ok((
                    false,
                    Some(
                        "no-op: Delivery_Committed fuera de trunk_direct (Done solo tras merge en branch_pr)"
                            .into(),
                    ),
                ));
            }
            let sha = str_field(&payload_val, "commit_sha").unwrap_or_default();
            stamped = stamp_done_with_hu_roll_up(
                repo,
                &tc,
                &issue_ref,
                &pbi_fetch,
                &payload_val,
                &format!("Delivery commit: `{sha}`"),
                source,
                &sha,
            )?;
        }
        other => return Ok((false, Some(format!("no-op: event_type {other}")))),
    }

    Ok((stamped, None))
}

fn payload_expects_tracker_stamp(payload: &Value) -> bool {
    ["pbi_ref", "hu_ref", "persist_ref"]
        .iter()
        .any(|k| {
            payload
                .get(*k)
                .and_then(|v| v.as_str())
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
        })
}

/// Estado de entrega para `route-domain-event` (AUT-46).
pub(crate) fn classify_stamp_delivery(
    stamped: bool,
    note: Option<&str>,
    event: &Value,
) -> (String, i32) {
    if stamped {
        return ("success".into(), 0);
    }
    let note = note.unwrap_or("");
    if note.starts_with("warn:") {
        return ("failed".into(), 1);
    }
    if !note.starts_with("no-op:") {
        return ("failed".into(), 1);
    }
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    if note.contains("sin tracker_ref") {
        if payload_expects_tracker_stamp(&payload) {
            return ("failed".into(), 1);
        }
        return ("skipped-no-tracker-ref".into(), 0);
    }
    if note.contains("proyecto sin tracker") {
        return ("skipped-no-tracker-config".into(), 0);
    }
    if note.contains("trunk_direct") || note.contains("branch_pr") {
        return ("skipped-delivery-mode".into(), 0);
    }
    if note.contains("event_type") {
        return ("skipped-event-type".into(), 0);
    }
    if note.contains("pbi_ref") || note.contains("resoluble") {
        return ("failed".into(), 1);
    }
    ("skipped-no-op".into(), 0)
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event = load_event(repo, process_inputs)?;
    if let Err(reason) = super::super::ecst_input_loader::verify_ecst_matches_bus(repo, &event) {
        let delivery_status = "failed-bus-mismatch".to_string();
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "stamped": false,
                "delivery_status": delivery_status,
                "reason": reason,
            })),
            error: Some(reason),
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Sello de ciclo",
                    "status": "failed",
                    "handler": "tracker-stamp",
                }]
            })),
            exit_code: 1,
        });
    }
    match stamp_event(repo, &event) {
        Ok((stamped, note)) => {
            let (delivery_status, exit_code) =
                classify_stamp_delivery(stamped, note.as_deref(), &event);
            let ok = delivery_status == "success";
            Ok(OrchestratorEnvelope {
                success: ok,
                status_code: exit_code,
                data: Some(json!({
                    "stamped": stamped,
                    "note": note,
                    "delivery_status": delivery_status,
                })),
                error: note.as_ref().filter(|n| n.starts_with("warn:")).cloned(),
                execution_report: Some(json!({
                    "phases": [{
                        "phase_name": "Sello de ciclo",
                        "status": if ok { "executed" } else if delivery_status.starts_with("skipped") { "skipped" } else { "failed" },
                        "handler": "tracker-stamp",
                    }]
                })),
                exit_code,
            })
        }
        Err(warn) if warn.starts_with("warn:") => Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "stamped": false,
                "warn": warn,
                "delivery_status": "failed",
            })),
            error: Some(warn.clone()),
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Sello de ciclo",
                    "status": "failed",
                    "handler": "tracker-stamp",
                }]
            })),
            exit_code: 1,
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
    fn classify_no_op_without_tracker_ref_is_skipped() {
        let event = json!({
            "event_type": "Work_Initiated",
            "payload": { "branch": "feat/x" }
        });
        let (status, code) = classify_stamp_delivery(false, Some("no-op: sin tracker_ref"), &event);
        assert_eq!(status, "skipped-no-tracker-ref");
        assert_eq!(code, 0);
    }

    #[test]
    fn classify_no_op_missing_ref_with_pbi_is_failed() {
        let event = json!({
            "event_type": "PBI_Refined",
            "payload": { "pbi_ref": "docs/x.md" }
        });
        let (status, code) = classify_stamp_delivery(false, Some("no-op: sin tracker_ref"), &event);
        assert_eq!(status, "failed");
        assert_eq!(code, 1);
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
    fn stamp_ignores_missing_pbi_label() {
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
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
        });
    }

    #[test]
    fn stamp_ignores_hu_parent_mismatch() {
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
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
        });
    }

    #[test]
    fn ac2_pbi_refined_moves_to_todo() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "PBI_Refined",
                "payload": {
                    "tracker_ref": "LAB-PBI-OK",
                    "pbi_ref": "docs/todos/pending/x.md",
                }
            });
            let out = stamp_event(&repo, &event);
            assert!(matches!(out, Ok((true, _))), "got {:?}", out);
        });
    }

    #[test]
    fn ac2b_hu_refined_moves_to_todo() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "HU_Refined",
                "payload": {
                    "tracker_ref": "LAB-HU-TODO",
                    "hu_ref": "docs/todos/historias/hu.md",
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
        });
    }

    #[test]
    fn ac3_delivery_committed_done() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "Delivery_Committed",
                "payload": {
                    "tracker_ref": "LAB-PBI-MERGE-LAST",
                    "commit_sha": "deadbeef",
                    "delivery_mode": "trunk_direct",
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
        });
    }

    #[test]
    fn delivery_committed_without_trunk_direct_no_ops() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "Delivery_Committed",
                "payload": {
                    "tracker_ref": "LAB-PBI-MERGE-LAST",
                    "commit_sha": "deadbeef",
                    "delivery_mode": "branch_pr",
                }
            });
            let (stamped, note) = stamp_event(&repo, &event).unwrap();
            assert!(!stamped);
            assert!(note.unwrap_or_default().contains("trunk_direct"));
        });
    }

    #[test]
    fn ac4_pbi_cancelled() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "PBI_Cancelled",
                "payload": {
                    "tracker_ref": "LAB-PBI-OK",
                    "reason": "descartado",
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
        });
    }

    #[test]
    fn stamp_ignores_fix_label_without_pbi() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "Work_Initiated",
                "payload": {
                    "tracker_ref": "LAB-PBI-FIXONLY",
                    "branch": "feat/x",
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).unwrap();
            assert!(stamped);
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

    fn write_file(dir: &std::path::Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, body).unwrap();
    }

    #[test]
    fn ac2_state_stamp_uses_live_op_and_schema() {
        with_lab(|| {
            std::env::remove_var("SDDIA_LAB_SKIP_CAPABILITY_DI");
            let repo = find_repo_root().expect("repo");
            let event = json!({
                "event_type": "PBI_Refined",
                "payload": {
                    "tracker_ref": "LAB-PBI-OK",
                    "pbi_ref": "docs/todos/pending/x.md",
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).expect("sello");
            assert!(stamped);
            let tool = lifecycle_tool_name(&repo).expect("provider");
            let raw = super::super::super::capsules::invoke_tool_capsule_json(
                &repo,
                &tool,
                &json!({
                    "request": {
                        "operation": "update_issue_state",
                        "issue_ref": "LAB-PBI-OK",
                        "state_name": "Todo",
                        "team_key": "LAB"
                    }
                }),
                true,
            )
            .expect("stdout");
            assert_eq!(raw.exit_code, 0);
            let phase = json!({
                "requires_capability": [{
                    "id": "tracker:lifecycle",
                    "contract": "tracker.lifecycle",
                    "version": ">=1.0.0"
                }]
            });
            let bindings =
                super::super::super::capability_di_resolver::resolve_phase_bindings(&repo, &phase)
                    .expect("binding");
            super::super::super::capability_di_output_validator::validate_output_payload(
                &repo,
                &bindings[0],
                &raw.body,
            )
            .expect("schema");
        });
    }

    #[test]
    fn file_provider_stamps_column_on_pbi_refined() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_file(
            root,
            "SddIA/core/cumulo.paths.json",
            r#"{ "instance": { "projects": ".SddIA/projects" } }"#,
        );
        write_file(
            root,
            ".SddIA/projects/demo.md",
            "---\nmanifest_ref: .SddIA/project.md\nproject_root: .\n---\n",
        );
        write_file(
            root,
            ".SddIA/project.md",
            "---\ntracker:\n  provider: file\n---\n",
        );
        write_file(
            root,
            "docs/todos/pending/x.md",
            "---\ntracker_column: backlog\nrefinement: pending\n---\n\n# x\n",
        );
        let event = json!({
            "event_type": "PBI_Refined",
            "payload": {
                "project_slug": "demo",
                "tracker_ref": "OSC-1",
                "pbi_ref": "docs/todos/pending/x.md"
            }
        });
        let (stamped, note) = stamp_event(root, &event).expect("file");
        assert!(stamped);
        assert!(note.is_none());
        let text = std::fs::read_to_string(root.join("docs/todos/pending/x.md")).unwrap();
        assert!(text.contains("tracker_column: todo"));
    }

    #[test]
    fn work_initiated_records_persist_ref_without_done() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let log = repo.join(".SddIA/state/tracker-outbound/LAB-PBI-OK.jsonl");
            let before = std::fs::read_to_string(&log).unwrap_or_default();
            let event = json!({
                "event_type": "Work_Initiated",
                "payload": {
                    "tracker_ref": "LAB-PBI-OK",
                    "branch": "feat/persist-ref",
                    "persist_ref": "docs/features/tracker-lifecycle-persist-ref",
                    "expected_hu_tracker_ref": "LAB-HU-1"
                }
            });
            let (stamped, _) = stamp_event(&repo, &event).expect("sello");
            assert!(stamped);
            let after = std::fs::read_to_string(&log).unwrap_or_default();
            let fresh = after.trim_start_matches(&before);
            assert!(fresh.contains("\"operation\":\"update_issue_state\""));
            assert!(fresh.contains("\"operation\":\"create_comment\""));
            assert!(!fresh.contains("\"done\""));
        });
    }

    #[test]
    fn file_provider_work_initiated_stamps_column_without_capsule() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write_file(
            root,
            "SddIA/core/cumulo.paths.json",
            r#"{ "instance": { "projects": ".SddIA/projects" } }"#,
        );
        write_file(
            root,
            ".SddIA/projects/demo.md",
            "---\nmanifest_ref: .SddIA/project.md\nproject_root: .\n---\n",
        );
        write_file(
            root,
            ".SddIA/project.md",
            "---\ntracker:\n  provider: file\n---\n",
        );
        write_file(
            root,
            "docs/features/x/objectives.md",
            "---\npbi_ref: docs/todos/pending/wi.md\n---\n\n# obj\n",
        );
        write_file(
            root,
            "docs/todos/pending/wi.md",
            "---\ntracker_column: todo\nrefinement: refined\n---\n\n# wi\n",
        );
        let event = json!({
            "event_type": "Work_Initiated",
            "payload": {
                "project_slug": "demo",
                "tracker_ref": "OSC-1",
                "branch": "feat/x",
                "persist_ref": "docs/features/x"
            }
        });
        let (stamped, note) = stamp_event(root, &event).expect("file");
        assert!(stamped);
        assert!(note.is_none());
        let text = std::fs::read_to_string(root.join("docs/todos/pending/wi.md")).unwrap();
        assert!(text.contains("tracker_column: in_progress"));
        assert!(!root.join(".SddIA/state/tracker-outbound/OSC-1.jsonl").is_file());
    }

    #[test]
    fn forge_processes_keep_tracker_context_out() {
        let repo = find_repo_root().expect("repo");
        for rel in [
            "SddIA/library/codexes/codex-software-engineering/process/feature.md",
            "SddIA/library/codexes/codex-software-engineering/process/bug-fix.md",
            "SddIA/library/codexes/codex-software-engineering/process/refactorization.md",
        ] {
            let text = std::fs::read_to_string(repo.join(rel)).expect(rel);
            assert!(!text.contains("tracker-operations"), "{rel}");
            assert!(!text.contains("tool:"), "{rel}");
        }
    }
}
