//! Handler `tracker-markdown-apply` — réplica local del ítem remoto.

use super::super::actions::try_run_native;
use super::super::capsules::{invoke_git_manager_for, invoke_tool_for_process};
use super::super::project_binding::{self, DoneGate, TrackerConfig};
use super::super::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::{DateTime, Utc};
use sddia_io::outbound_lab::lab_mock_outbound_enabled;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

pub const PROCESS_NAME: &str = "tracker-markdown-apply";

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn skip_git_persist() -> bool {
    std::env::var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn sync_branch_name(project_slug: &str) -> String {
    format!("tracker-sync/{project_slug}")
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("MARKDOWN_APPLY: sin frontmatter".into());
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Ok((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    Err("MARKDOWN_APPLY: frontmatter sin cierre".into())
}

fn parse_fm(text: &str) -> Result<YamlValue, String> {
    let (fm, _) = split_frontmatter(text)?;
    serde_yaml::from_str(&fm).map_err(|e| format!("MARKDOWN_APPLY: yaml {e}"))
}

fn fm_str(fm: &YamlValue, key: &str) -> Option<String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn seal_markdown(raw: &str, patch: &YamlValue) -> Result<String, String> {
    let (fm_text, body) = split_frontmatter(raw)?;
    let mut fm: YamlValue = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    if let Some(map) = patch.as_mapping() {
        for (k, v) in map {
            if let Some(key) = k.as_str() {
                fm[key] = v.clone();
            }
        }
    }
    let new_fm = serde_yaml::to_string(&fm).map_err(|e| e.to_string())?;
    let body = body.trim_start_matches('\n');
    Ok(format!("---\n{new_fm}---\n\n{body}"))
}

fn parse_rfc3339(s: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn occurred_not_after_stored(stored: &str, incoming: &str) -> bool {
    match (
        parse_rfc3339(stored),
        parse_rfc3339(incoming),
    ) {
        (Some(a), Some(b)) => b <= a,
        _ => stored == incoming,
    }
}

pub fn resolve_markdown_by_tracker_ref(repo: &Path, issue_ref: &str) -> Option<PathBuf> {
    let body = file_tracker_adapter::dispatch(
        repo,
        &json!({
            "request": { "operation": "fetch_issue", "issue_ref": issue_ref }
        }),
    )
    .ok()?;
    let rel = body.get("artifact_path").and_then(|v| v.as_str())?;
    Some(repo.join(rel))
}

fn resolve_hu_ref_by_tracker(repo: &Path, parent_tracker: &str) -> Option<String> {
    let body = file_tracker_adapter::dispatch(
        repo,
        &json!({
            "request": { "operation": "fetch_issue", "issue_ref": parent_tracker }
        }),
    )
    .ok()?;
    body.get("artifact_path")
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn linear_priority_to_fm(p: i64) -> &'static str {
    match p {
        1 => "urgente",
        2 => "alta",
        3 => "media",
        4 => "baja",
        _ => "sin-prioridad",
    }
}

fn labels_has_editable(fetch: &Value, editable_label: &str) -> bool {
    fetch
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str())
                .any(|n| n.eq_ignore_ascii_case(editable_label))
        })
        .unwrap_or(false)
}

fn tool_fetch_issue(repo: &Path, issue_ref: &str, project_id: Option<&str>) -> Result<Value, String> {
    let tool = super::tracker_stamp::lifecycle_tool_name(repo)?;
    invoke_tool_for_process(
        repo,
        &tool,
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_id": project_id,
            }
        }),
        Some(PROCESS_NAME),
    )
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
    TrackerConfig {
        team_key,
        project_id: None,
        state_map,
        label_hu: "hu".into(),
        label_pbi: "pbi".into(),
        done_gate: DoneGate::Git,
    }
}

fn linear_to_canonical(linear_state: &str, tc: &TrackerConfig) -> String {
    for (canonical, name) in &tc.state_map {
        if name.eq_ignore_ascii_case(linear_state) {
            return canonical.clone();
        }
    }
    linear_state.to_ascii_lowercase().replace(' ', "_")
}

fn editable_label_default() -> String {
    "sddia-editable".into()
}

fn repo_local_edit_after(fm: &YamlValue) -> Option<String> {
    fm_str(fm, "tracker_local_edit_after")
}

fn repo_diverged(fm: &YamlValue, linear_updated_at: Option<&str>) -> bool {
    let Some(local) = repo_local_edit_after(fm) else {
        return false;
    };
    let Some(lu) = linear_updated_at.filter(|s| !s.is_empty()) else {
        return true;
    };
    match (parse_rfc3339(&local), parse_rfc3339(lu)) {
        (Some(loc), Some(lin)) => loc > lin,
        _ => local.as_str() > lu,
    }
}

fn emit_diverged(
    repo: &Path,
    issue_ref: &str,
    project_slug: &str,
    correlation_id: &str,
) -> Result<(), String> {
    let _ = try_run_native(
        repo,
        "emit-tracker-sync-failed",
        &json!({
            "issue_ref": issue_ref,
            "operation": "update_issue_state",
            "error_code": "TRACKER_STATE_DIVERGED",
            "source_process": PROCESS_NAME,
            "project_slug": project_slug,
            "correlation_id": correlation_id,
        }),
    )?;
    Ok(())
}

fn emit_pbi_cancelled(
    repo: &Path,
    issue_ref: &str,
    project_slug: &str,
    correlation_id: &str,
) -> Result<String, String> {
    materialize_pending_domain_event(
        repo,
        "PBI_Cancelled",
        PROCESS_NAME,
        json!({
            "tracker_ref": issue_ref,
            "reason": "linear:cancelled",
            "project_slug": project_slug,
            "correlation_id": correlation_id,
        }),
    )
}

fn persist_git(
    repo: &Path,
    project_slug: &str,
    default_branch: &str,
    rel_paths: &[String],
) -> Result<Value, String> {
    let branch = sync_branch_name(project_slug);
    if branch == default_branch {
        return Err("MARKDOWN_APPLY: rama sync coincide con default_branch".into());
    }
    if skip_git_persist() {
        return Ok(json!({
            "git_skipped": true,
            "branch": branch,
            "files": rel_paths,
        }));
    }
    let checkout = invoke_git_manager_for(
        repo,
        repo,
        "checkout",
        &json!({
            "branch": branch,
            "create": true,
        }),
    )?;
    if checkout.get("success") == Some(&json!(false)) {
        return Err(format!(
            "git-manager checkout: {}",
            checkout.get("error").and_then(|v| v.as_str()).unwrap_or("?")
        ));
    }
    let commit = invoke_git_manager_for(
        repo,
        repo,
        "commit",
        &json!({
            "message": format!("tracker-sync: apply {}", rel_paths.join(", ")),
            "files": rel_paths,
        }),
    )?;
    Ok(json!({
        "branch": branch,
        "default_branch": default_branch,
        "commit": commit,
    }))
}

#[derive(Debug)]
pub struct ApplyOutcome {
    pub applied: bool,
    pub warn: Option<String>,
    pub markdown_rel: Option<String>,
    pub git: Option<Value>,
    pub events: Vec<String>,
}

pub fn apply_tracker_issue_changed(repo: &Path, event: &Value) -> Result<ApplyOutcome, String> {
    let payload = event
        .get("payload")
        .filter(|p| p.is_object())
        .ok_or("payload ECST ausente")?;
    let issue_ref = str_field(payload, "issue_ref").ok_or("issue_ref requerido")?;
    let occurred_at = str_field(payload, "occurred_at").ok_or("occurred_at requerido")?;
    let project_slug = str_field(payload, "project_slug").unwrap_or_else(|| "core".into());
    let correlation_id = str_field(payload, "correlation_id")
        .or_else(|| str_field(event, "event_id"))
        .unwrap_or_else(|| "unknown".into());
    let linear_updated_at = str_field(payload, "linear_updated_at");

    let md_path = resolve_markdown_by_tracker_ref(repo, &issue_ref);
    if md_path.is_none() {
        return Ok(ApplyOutcome {
            applied: false,
            warn: Some(format!("warn: sin markdown para tracker_ref {issue_ref}")),
            markdown_rel: None,
            git: None,
            events: vec![],
        });
    }
    let md_path = md_path.unwrap();
    let raw = fs::read_to_string(&md_path).map_err(|e| e.to_string())?;
    let fm = parse_fm(&raw)?;
    if let Some(last) = fm_str(&fm, "tracker_sync_last_event") {
        if occurred_not_after_stored(&last, &occurred_at) {
            return Ok(ApplyOutcome {
                applied: false,
                warn: Some("warn: evento idempotente descartado".into()),
                markdown_rel: None,
                git: None,
                events: vec![],
            });
        }
    }
    if repo_diverged(&fm, linear_updated_at.as_deref()) {
        let _ = emit_diverged(repo, &issue_ref, &project_slug, &correlation_id);
        return Ok(ApplyOutcome {
            applied: false,
            warn: Some("warn: TRACKER_STATE_DIVERGED".into()),
            markdown_rel: Some(match md_path.strip_prefix(repo) {
                Ok(p) => p.to_string_lossy().replace('\\', "/"),
                Err(_) => md_path.to_string_lossy().replace('\\', "/"),
            }),
            git: None,
            events: vec![],
        });
    }

    let inputs = json!({ "project_slug": project_slug });
    let tc = match super::super::project_binding::tracker_config_for_inputs(repo, &inputs)? {
        Some(tc) => Some(tc),
        None if lab_mock_outbound_enabled() => Some(lab_tracker_fallback(&issue_ref)),
        None => None,
    };
    let tc = match tc {
        Some(t) => t,
        None => {
            return Ok(ApplyOutcome {
                applied: false,
                warn: Some("warn: sin tracker en manifiesto".into()),
                markdown_rel: None,
                git: None,
                events: vec![],
            });
        }
    };

    let fetch = tool_fetch_issue(repo, &issue_ref, tc.project_id.as_deref())?;
    let linear_state = fetch
        .get("state")
        .and_then(|v| v.as_str())
        .unwrap_or("Backlog");
    let canonical = payload
        .get("changes")
        .and_then(|c| c.as_array())
        .and_then(|arr| {
            arr.iter().find_map(|ch| {
                if ch.get("field") != Some(&json!("state")) {
                    return None;
                }
                ch.get("to")
                    .and_then(|v| v.as_str())
                    .map(|s| linear_to_canonical(s, &tc))
            })
        })
        .unwrap_or_else(|| linear_to_canonical(linear_state, &tc));

    let mut patch_map = serde_yaml::Mapping::new();
    patch_map.insert(
        YamlValue::String("tracker_sync_last_event".into()),
        YamlValue::String(occurred_at.clone()),
    );
    if let Some(lu) = &linear_updated_at {
        patch_map.insert(
            YamlValue::String("tracker_sync_linear_updated_at".into()),
            YamlValue::String(lu.clone()),
        );
    }

    let mut moved_to_done = false;
    let mut events = Vec::new();
    let mut body_replace: Option<String> = None;
    let mut warns = Vec::new();

    if let Some(title) = fetch.get("title").and_then(|v| v.as_str()) {
        patch_map.insert(
            YamlValue::String("title".into()),
            YamlValue::String(title.to_string()),
        );
    }
    if let Some(p) = fetch.get("priority").and_then(|v| v.as_i64()) {
        patch_map.insert(
            YamlValue::String("priority".into()),
            YamlValue::String(linear_priority_to_fm(p).into()),
        );
    }
    if let Some(parent) = fetch.get("parent").and_then(|v| v.as_str()) {
        if let Some(href) = resolve_hu_ref_by_tracker(repo, parent) {
            patch_map.insert(
                YamlValue::String("historia_ref".into()),
                YamlValue::String(href),
            );
        }
    }

    let editable = editable_label_default();
    if labels_has_editable(&fetch, &editable) {
        if let Some(desc) = fetch.get("description").and_then(|v| v.as_str()) {
            body_replace = Some(desc.to_string());
        }
    } else if payload
        .get("changes")
        .and_then(|c| c.as_array())
        .map(|arr| arr.iter().any(|c| c.get("field") == Some(&json!("description"))))
        .unwrap_or(false)
    {
        warns.push("warn: description sin label editable — no-op cuerpo".into());
    }

    match canonical.as_str() {
        "cancelled" => {
            patch_map.insert(
                YamlValue::String("status".into()),
                YamlValue::String("cancelado".into()),
            );
            patch_map.insert(
                YamlValue::String("tracker_state".into()),
                YamlValue::String("cancelled".into()),
            );
            moved_to_done = true;
            events.push(emit_pbi_cancelled(
                repo,
                &issue_ref,
                &project_slug,
                &correlation_id,
            )?);
        }
        "done" => {
            patch_map.insert(
                YamlValue::String("tracker_state".into()),
                YamlValue::String("done".into()),
            );
            let dg = tc.done_gate;
            if super::super::tracker_lifecycle_port::local_replica_closes(dg) {
                patch_map.insert(
                    YamlValue::String("status".into()),
                    YamlValue::String("done".into()),
                );
                moved_to_done = true;
            }
        }
        other => {
            patch_map.insert(
                YamlValue::String("tracker_state".into()),
                YamlValue::String(other.to_string()),
            );
        }
    }

    let patch = YamlValue::Mapping(patch_map);
    let mut sealed = seal_markdown(&raw, &patch)?;
    if let Some(body) = body_replace {
        let (fm_text, _) = split_frontmatter(&sealed)?;
        sealed = format!("---\n{fm_text}---\n\n{}", body.trim_start());
    }
    let mut final_path = md_path.clone();
    // La réplica ya resuelta vive en el bucket del Core (`resolve_markdown_by_tracker_ref`
    // no lleva `project_slug`). El slug elige el tracker remoto y la rama sync.
    if moved_to_done {
        let state_name = if canonical == "cancelled" { "cancelled" } else { "done" };
        file_tracker_adapter::dispatch(
            repo,
            &json!({
                "request": {
                    "operation": "update_issue_state",
                    "issue_ref": issue_ref,
                    "state_name": state_name,
                }
            }),
        )?;
        if let Some(moved) = resolve_markdown_by_tracker_ref(repo, &issue_ref) {
            final_path = moved;
        }
    }
    file_tracker_adapter::dispatch(
        repo,
        &json!({
            "request": {
                "operation": "update_issue_description",
                "issue_ref": issue_ref,
                "description": sealed,
            }
        }),
    )?;

    let rel = final_path
        .strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();

    let default_branch = project_binding::bind(repo, &inputs)
        .ok()
        .flatten()
        .map(|b| b.default_branch)
        .unwrap_or_else(|| "main".into());
    let git = persist_git(repo, &project_slug, &default_branch, &[rel.clone()])?;

    Ok(ApplyOutcome {
        applied: true,
        warn: warns.first().cloned(),
        markdown_rel: Some(rel),
        git: Some(git),
        events,
    })
}

fn load_event(repo: &Path, inputs: &Value) -> Result<Value, String> {
    super::super::ecst_input_loader::load_ecst_from_inputs(repo, inputs)
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event = load_event(repo, process_inputs)?;
    if event.get("event_type").and_then(|v| v.as_str()) != Some("Tracker_Issue_Changed") {
        return Err("event_type debe ser Tracker_Issue_Changed".into());
    }
    let outcome = apply_tracker_issue_changed(repo, &event)?;
    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "applied": outcome.applied,
            "warn": outcome.warn,
            "markdown_path": outcome.markdown_rel,
            "git": outcome.git,
            "events": outcome.events,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [{
                "phase_name": "markdown-apply",
                "status": "executed",
                "handler": PROCESS_NAME,
            }]
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;
    use std::env;
    use uuid::Uuid;

    fn with_lab<F: FnOnce()>(f: F) {
        let prev = env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        let prev_git = env::var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT").ok();
        env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        env::set_var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT", "1");
        f();
        if let Some(v) = prev {
            env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v);
        } else {
            env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        }
        if let Some(v) = prev_git {
            env::set_var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT", v);
        } else {
            env::remove_var("SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT");
        }
    }

    fn cleanup_test_pbis(repo: &Path) {
        let dir = repo.join("docs/todos/pending");
        if let Ok(rd) = fs::read_dir(&dir) {
            for ent in rd.flatten() {
                let name = ent.file_name().to_string_lossy().to_string();
                if name.starts_with("PBI-TEST-") && name.ends_with(".md") {
                    let _ = fs::remove_file(ent.path());
                }
            }
        }
        let done = repo.join("docs/todos/done");
        if let Ok(rd) = fs::read_dir(&done) {
            for ent in rd.flatten() {
                let name = ent.file_name().to_string_lossy().to_string();
                if name.starts_with("PBI-TEST-") && name.ends_with(".md") {
                    let _ = fs::remove_file(ent.path());
                }
            }
        }
    }

    fn fixture_pbi(repo: &Path, tracker_ref: &str, extra_fm: &str) -> PathBuf {
        cleanup_test_pbis(repo);
        let dir = repo.join("docs/todos/pending");
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(repo.join(".events/pending")).unwrap();
        let path = dir.join(format!("PBI-TEST-{}.md", Uuid::new_v4()));
        let content = format!(
            "---\ndocument_id: PBI-TEST\ntracker_ref: \"{tracker_ref}\"\ntitle: \"Old\"\nstatus: pending\n{extra_fm}---\n\n# Body\n\nOriginal.\n"
        );
        fs::write(&path, content).unwrap();
        path
    }

    fn tracker_changed_event(issue_ref: &str, linear_updated_at: &str) -> Value {
        let occurred_at = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        json!({
            "event_id": Uuid::new_v4().to_string(),
            "event_type": "Tracker_Issue_Changed",
            "payload": {
                "correlation_id": Uuid::new_v4().to_string(),
                "occurred_at": occurred_at,
                "linear_updated_at": linear_updated_at,
                "project_slug": "core",
                "issue_ref": issue_ref,
                "source": "poll",
                "changes": [{"field": "title", "from": "Old", "to": "New"}],
            }
        })
    }

    #[test]
    fn resolve_finds_pending_fixture() {
        let repo = find_repo_root().expect("repo");
        let path = fixture_pbi(&repo, "LAB-MD-RESOLVE-SMOKE", "");
        let raw = fs::read_to_string(&path).unwrap();
        assert!(
            parse_fm(&raw).is_ok(),
            "parse_fm: {:?}",
            parse_fm(&raw).err()
        );
        assert_eq!(
            fm_str(&parse_fm(&raw).unwrap(), "tracker_ref").as_deref(),
            Some("LAB-MD-RESOLVE-SMOKE")
        );
        assert!(
            resolve_markdown_by_tracker_ref(&repo, "LAB-MD-RESOLVE-SMOKE").is_some(),
            "repo={}",
            repo.display()
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn ac14_title_updates_frontmatter_not_filename() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let path = fixture_pbi(&repo, "LAB-MD-APPLY-TITLE", "");
            let fname = path.file_name().unwrap().to_owned();
            let event = tracker_changed_event("LAB-MD-APPLY-TITLE", "2026-10-12T09:00:00Z");
            let out = apply_tracker_issue_changed(&repo, &event).unwrap();
            assert!(out.applied, "warn={:?}", out.warn);
            let text = fs::read_to_string(&path).unwrap();
            assert!(text.contains("lab mock LAB-MD-APPLY-TITLE"));
            assert_eq!(path.file_name(), Some(fname.as_ref()));
            let _ = fs::remove_file(&path);
        });
    }

    #[test]
    fn ac14_cancelled_moves_and_emits_pbi_cancelled() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let path = fixture_pbi(&repo, "LAB-PBI-CANCEL", "");
            let mut event = tracker_changed_event("LAB-PBI-CANCEL", "2026-10-12T10:00:00Z");
            event["payload"]["changes"] = json!([{
                "field": "state",
                "from": "In Progress",
                "to": "Cancelled",
            }]);
            let out = apply_tracker_issue_changed(&repo, &event).unwrap();
            assert!(out.applied);
            assert!(!out.events.is_empty());
            let done_path = repo.join("docs/todos/done").join(path.file_name().unwrap());
            assert!(done_path.is_file());
            let text = fs::read_to_string(&done_path).unwrap();
            assert!(text.contains("status: cancelado"));
            let _ = fs::remove_file(&done_path);
        });
    }

    #[test]
    fn ac15_diverged_keeps_file_and_warns() {
        with_lab(|| {
            let repo = find_repo_root().expect("repo");
            let extra = "tracker_local_edit_after: \"2026-10-12T12:00:00Z\"\n";
            let path = fixture_pbi(&repo, "LAB-MD-APPLY-DIV", extra);
            let before = fs::read_to_string(&path).unwrap();
            let event = tracker_changed_event("LAB-MD-APPLY-DIV", "2026-10-12T10:00:00Z");
            let out = apply_tracker_issue_changed(&repo, &event).unwrap();
            assert!(!out.applied);
            assert!(out.warn.as_deref().unwrap_or("").contains("DIVERGED"));
            let after = fs::read_to_string(&path).unwrap();
            assert_eq!(before, after);
            let _ = fs::remove_file(&path);
        });
    }

    #[test]
    fn ac16_sync_branch_not_default() {
        with_lab(|| {
            let branch = sync_branch_name("core");
            assert_eq!(branch, "tracker-sync/core");
            assert_ne!(branch, "main");
            let repo = find_repo_root().expect("repo");
            let path = fixture_pbi(&repo, "LAB-MD-APPLY-GIT", "");
            let event = tracker_changed_event("LAB-MD-APPLY-GIT", "2026-10-12T11:00:00Z");
            let out = apply_tracker_issue_changed(&repo, &event).unwrap();
            assert!(out.applied, "warn={:?}", out.warn);
            let git = out.git.expect("git plan");
            assert_eq!(git.get("branch"), Some(&json!("tracker-sync/core")));
            assert_ne!(git.get("branch"), Some(&json!("main")));
            let _ = fs::remove_file(path);
        });
    }
}
