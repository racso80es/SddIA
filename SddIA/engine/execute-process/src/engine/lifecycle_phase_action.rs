//! AUT-75 — sello de estado del ítem como acción de fase (no vía suscriptor de eventos).

use super::handlers::tracker_stamp;
use super::item_lifecycle_frontmatter::{
    self, yaml_patch_forge_closed, yaml_patch_forge_started, yaml_patch_pr_opened,
    yaml_patch_review_ci_green, yaml_patch_review_ci_red, yaml_patch_validation_apto,
    yaml_patch_validation_no_apto, COLUMN_IN_PROGRESS, COLUMN_IN_VALIDATION, COLUMN_IN_REVIEW,
    COLUMN_VALIDATED, COLUMN_DONE,
};
use super::project_binding::{self, TrackerConfig};
use super::tracker_lifecycle_port;
use super::tracker_pbi_meta;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleMilestone {
    ForgeStarted,
    ForgeClosed,
    ValidationApto,
    ValidationNoApto,
    PrOpened,
    ReviewCiGreen,
    ReviewCiRed,
}

const SOURCE: &str = "lifecycle-phase-action";

fn str_field(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn iso_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

pub fn persist_ref_from_inputs(repo: &Path, inputs: &Value) -> Option<PathBuf> {
    let persist = str_field(inputs, "persist_ref")?;
    let p = if Path::new(&persist).is_absolute() {
        PathBuf::from(&persist)
    } else {
        repo.join(&persist)
    };
    Some(p)
}

pub fn persist_forge_artifacts_ready(repo: &Path, inputs: &Value) -> bool {
    persist_ref_from_inputs(repo, inputs)
        .map(|dir| {
            dir.join("implementation.md").is_file() && dir.join("execution.md").is_file()
        })
        .unwrap_or(false)
}

fn read_validacion_global(repo: &Path, inputs: &Value) -> Option<String> {
    let path = persist_ref_from_inputs(repo, inputs)?.join("validacion.md");
    let text = fs::read_to_string(path).ok()?;
    let mut lines = text.lines();
    if lines.next()? != "---" {
        return None;
    }
    for line in lines {
        if line == "---" {
            break;
        }
        if let Some(v) = line.strip_prefix("global:") {
            return Some(v.trim().to_string());
        }
    }
    None
}

pub fn build_cycle_payload(repo: &Path, inputs: &Value) -> Value {
    let mut payload = json!({});
    for key in [
        "persist_ref",
        "branch_name",
        "branch",
        "pbi_ref",
        "project_slug",
        "tracker_ref",
        "pr_url",
    ] {
        if let Some(v) = inputs.get(key) {
            payload[key] = v.clone();
        }
    }
    if payload.get("tracker_ref").is_none() {
        if let Some(pbi) = str_field(inputs, "pbi_ref") {
            if let Some(tr) = tracker_pbi_meta::resolve_tracker_ref_from_pbi_ref(repo, &pbi) {
                payload["tracker_ref"] = json!(tr);
            }
        }
    }
    if payload.get("project_slug").is_none() && project_binding::repo_is_core(repo) {
        payload["project_slug"] = json!("core");
    }
    payload
}

fn milestone_column_and_patch(milestone: LifecycleMilestone, actor: &str) -> (&'static str, YamlValue) {
    let date = iso_date();
    let actor = actor.trim();
    match milestone {
        LifecycleMilestone::ForgeStarted => (COLUMN_IN_PROGRESS, yaml_patch_forge_started()),
        LifecycleMilestone::ForgeClosed => (
            COLUMN_IN_VALIDATION,
            yaml_patch_forge_closed(&date, actor),
        ),
        LifecycleMilestone::ValidationApto => (
            COLUMN_VALIDATED,
            yaml_patch_validation_apto(&date, actor),
        ),
        LifecycleMilestone::ValidationNoApto => (
            COLUMN_IN_PROGRESS,
            yaml_patch_validation_no_apto(&date, actor),
        ),
        LifecycleMilestone::PrOpened => (COLUMN_IN_REVIEW, yaml_patch_pr_opened(&date, actor)),
        LifecycleMilestone::ReviewCiGreen => (COLUMN_DONE, yaml_patch_review_ci_green(&date, actor)),
        LifecycleMilestone::ReviewCiRed => (COLUMN_IN_PROGRESS, yaml_patch_review_ci_red(&date, actor)),
    }
}

fn state_key_for_column(column: &str) -> &str {
    match column {
        item_lifecycle_frontmatter::COLUMN_IN_PROGRESS => "in_progress",
        item_lifecycle_frontmatter::COLUMN_IN_VALIDATION => "in_validation",
        item_lifecycle_frontmatter::COLUMN_VALIDATED => "validated",
        item_lifecycle_frontmatter::COLUMN_IN_REVIEW => "in_review",
        item_lifecycle_frontmatter::COLUMN_DONE => "done",
        _ => column,
    }
}

fn resolve_local_item_path(repo: &Path, payload: &Value) -> Option<PathBuf> {
    if let Some(pbi) = str_field(payload, "pbi_ref") {
        if looks_like_opaque_ref(&pbi) {
            if let Some(tr) = str_field(payload, "tracker_ref") {
                if let Some(p) =
                    super::handlers::tracker_markdown_apply::resolve_markdown_by_tracker_ref(repo, &tr)
                {
                    if p.is_file() {
                        return Some(p);
                    }
                }
            }
        } else {
            let direct = repo.join(pbi.trim_start_matches("./"));
            if direct.is_file() {
                return Some(direct);
            }
        }
    }
    if let Some(tr) = str_field(payload, "tracker_ref") {
        if let Some(p) =
            super::handlers::tracker_markdown_apply::resolve_markdown_by_tracker_ref(repo, &tr)
        {
            if p.is_file() {
                return Some(p);
            }
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

fn looks_like_opaque_ref(s: &str) -> bool {
    tracker_pbi_meta::looks_like_issue_identifier(s)
}

fn patch_local_path(path: &Path, patch: &YamlValue) -> Result<(), String> {
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let patched = item_lifecycle_frontmatter::seal_markdown(&raw, patch)?;
    fs::write(path, patched).map_err(|e| e.to_string())
}

fn apply_file(
    repo: &Path,
    payload: &Value,
    patch: &YamlValue,
    state_key: &str,
    tc: Option<&TrackerConfig>,
) -> Result<Value, String> {
    if let Some(tc) = tc {
        if tc.state_map.get(state_key).is_none() {
            return Ok(json!({
                "success": false,
                "error": format!("state_map sin {state_key}"),
            }));
        }
    }
    let path = resolve_local_item_path(repo, payload)
        .ok_or_else(|| "ítem local no resuelto".to_string())?;
    patch_local_path(&path, patch)?;
    Ok(json!({"success": true, "path": path.to_string_lossy()}))
}

fn tool_fetch_issue(repo: &Path, issue_ref: &str, tc: &TrackerConfig) -> Result<Value, String> {
    super::capsules::invoke_tool_for_process(
        repo,
        &tracker_stamp::lifecycle_tool_name(repo)?,
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_id": tc.project_id,
            }
        }),
        Some(SOURCE),
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
    super::capsules::invoke_tool_for_process(
        repo,
        &tracker_stamp::lifecycle_tool_name(repo)?,
        &json!({ "request": req }),
        Some(SOURCE),
    )
    .map(|_| ())
}

fn tool_update_state(
    repo: &Path,
    tc: &TrackerConfig,
    issue_ref: &str,
    state_key: &str,
    process_name: &str,
) -> Result<(), String> {
    let req = tracker_stamp::update_issue_state_request(tc, issue_ref, state_key)?;
    super::capsules::invoke_tool_for_process(
        repo,
        &tracker_stamp::lifecycle_tool_name(repo)?,
        &json!({ "request": req }),
        Some(process_name),
    )
    .map(|_| ())
}

fn apply_remote(
    repo: &Path,
    payload: &Value,
    patch: &YamlValue,
    state_key: &str,
    tc: &TrackerConfig,
    comment: &str,
    process_name: &str,
) -> Result<Value, String> {
    if tc.state_map.get(state_key).is_none() {
        return Ok(json!({
            "success": false,
            "error": format!("state_map sin {state_key}"),
        }));
    }
    let issue_ref = str_field(payload, "tracker_ref")
        .or_else(|| str_field(payload, "pbi_ref"))
        .ok_or("tracker_ref ausente")?;
    let fetch = tool_fetch_issue(repo, &issue_ref, tc)?;
    let desc = fetch
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let sealed = if desc.trim().is_empty() {
        return Ok(json!({"success": false, "error": "descripción vacía"}));
    } else {
        item_lifecycle_frontmatter::seal_markdown(desc, patch)?
    };
    tool_update_description(
        repo,
        &issue_ref,
        &sealed,
        tc,
        str_field(payload, "project_slug").as_deref(),
    )?;
    if let Err(e) = tool_update_state(repo, tc, &issue_ref, state_key, process_name) {
        return Ok(json!({"success": false, "error": e}));
    }
    let _ = super::capsules::invoke_tool_for_process(
        repo,
        &tracker_stamp::lifecycle_tool_name(repo)?,
        &json!({
            "request": {
                "operation": "create_comment",
                "issue_ref": issue_ref,
                "body": comment,
                "project_id": tc.project_id,
                "project_slug": str_field(payload, "project_slug"),
            }
        }),
        Some(SOURCE),
    );
    Ok(json!({"success": true, "tracker_ref": issue_ref}))
}

pub fn apply_milestone(
    repo: &Path,
    inputs: &Value,
    milestone: LifecycleMilestone,
    actor: &str,
) -> Value {
    let payload = build_cycle_payload(repo, inputs);
    let (_column, patch) = milestone_column_and_patch(milestone, actor);
    let state_key = state_key_for_column(_column);
    let comment = format!("lifecycle: {milestone:?} ({actor})");
    let process_name = if actor.trim().is_empty() {
        SOURCE
    } else {
        actor
    };
    let target = tracker_lifecycle_port::resolve_target(repo, &payload);
    let result = match target {
        Ok(tracker_lifecycle_port::LifecycleTarget::File) => {
            apply_file(repo, &payload, &patch, state_key, None)
        }
        Ok(tracker_lifecycle_port::LifecycleTarget::Remote { .. }) => {
            match project_binding::tracker_config_for_inputs(repo, &payload) {
                Ok(Some(tc)) => apply_remote(
                    repo,
                    &payload,
                    &patch,
                    state_key,
                    &tc,
                    &comment,
                    process_name,
                ),
                Ok(None) => Ok(json!({"success": false, "error": "sin tracker config"})),
                Err(e) => Err(e),
            }
        }
        Ok(tracker_lifecycle_port::LifecycleTarget::Unconfigured) => {
            Ok(json!({"success": false, "error": "tracker no configurado"}))
        }
        Err(e) => Err(e),
    };
    match result {
        Ok(v) => {
            let mut out = v;
            if let Some(obj) = out.as_object_mut() {
                obj.insert("milestone".into(), json!(format!("{milestone:?}")));
            }
            out
        }
        Err(e) => json!({"success": false, "error": e, "milestone": format!("{milestone:?}")}),
    }
}

pub fn apply_forge_started_fail_soft(repo: &Path, inputs: &Value, actor: &str) -> Value {
    apply_milestone(repo, inputs, LifecycleMilestone::ForgeStarted, actor)
}

pub fn maybe_apply_forge_closed(repo: &Path, inputs: &Value, actor: &str) -> Option<Value> {
    if !persist_forge_artifacts_ready(repo, inputs) {
        return None;
    }
    Some(apply_milestone(
        repo,
        inputs,
        LifecycleMilestone::ForgeClosed,
        actor,
    ))
}

pub fn maybe_apply_validation_from_validacion(repo: &Path, inputs: &Value, actor: &str) -> Option<Value> {
    let global = read_validacion_global(repo, inputs)?;
    let milestone = if global.eq_ignore_ascii_case("APTO") {
        LifecycleMilestone::ValidationApto
    } else {
        LifecycleMilestone::ValidationNoApto
    };
    Some(apply_milestone(repo, inputs, milestone, actor))
}

pub fn apply_pr_opened_fail_soft(repo: &Path, inputs: &Value, actor: &str) -> Value {
    apply_milestone(repo, inputs, LifecycleMilestone::PrOpened, actor)
}

pub fn apply_review_ci_fail_soft(repo: &Path, inputs: &Value, ci_green: bool, actor: &str) -> Value {
    let m = if ci_green {
        LifecycleMilestone::ReviewCiGreen
    } else {
        LifecycleMilestone::ReviewCiRed
    };
    apply_milestone(repo, inputs, m, actor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_file(root: &Path, rel: &str, body: &str) {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::File::create(&p)
            .unwrap()
            .write_all(body.as_bytes())
            .unwrap();
    }

    fn file_project(root: &Path) {
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
            "---\ntracker:\n  provider: file\n  state_map:\n    backlog: Backlog\n    todo: Todo\n    in_progress: In Progress\n    in_validation: In Validation\n    validated: Validated\n    in_review: In Review\n    done: Done\n---\n",
        );
    }

    #[test]
    fn aut75_forge_closed_when_execution_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        file_project(root);
        write_file(
            root,
            "docs/todos/pending/wi.md",
            "---\ntracker_column: in_progress\nrefinement: refined\n---\n\n# wi\n",
        );
        write_file(
            root,
            "docs/fixes/x/objectives.md",
            "---\npbi_ref: docs/todos/pending/wi.md\n---\n\n# o\n",
        );
        write_file(root, "docs/fixes/x/implementation.md", "# impl\n");
        write_file(root, "docs/fixes/x/execution.md", "# exec\n");
        let inputs = json!({
            "project_slug": "demo",
            "persist_ref": "docs/fixes/x",
            "pbi_ref": "docs/todos/pending/wi.md",
        });
        assert!(persist_forge_artifacts_ready(root, &inputs));
        let out = maybe_apply_forge_closed(root, &inputs, "argos-preflight").unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        let text = fs::read_to_string(root.join("docs/todos/pending/wi.md")).unwrap();
        assert!(text.contains("tracker_column: in_validation"));
        assert!(text.contains("forge_by:"));
        assert!(text.contains("argos-preflight"));
    }

    #[test]
    fn aut75_argos_apto_sets_validated() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        file_project(root);
        write_file(
            root,
            "docs/todos/pending/wi.md",
            "---\ntracker_column: in_validation\n---\n\n# wi\n",
        );
        write_file(
            root,
            "docs/fixes/x/validacion.md",
            "---\nglobal: APTO\n---\n\n# v\n",
        );
        let inputs = json!({
            "project_slug": "demo",
            "persist_ref": "docs/fixes/x",
            "pbi_ref": "docs/todos/pending/wi.md",
        });
        let out = maybe_apply_validation_from_validacion(root, &inputs, "argos").unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        let text = fs::read_to_string(root.join("docs/todos/pending/wi.md")).unwrap();
        assert!(text.contains("tracker_column: validated"));
        assert!(text.contains("validated_status: apto"));
    }

    #[test]
    fn aut75_argos_no_apto_returns_in_progress() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        file_project(root);
        write_file(
            root,
            "docs/todos/pending/wi.md",
            "---\ntracker_column: in_validation\n---\n\n# wi\n",
        );
        write_file(
            root,
            "docs/fixes/x/validacion.md",
            "---\nglobal: NO_APTO\n---\n\n# v\n",
        );
        let inputs = json!({
            "project_slug": "demo",
            "persist_ref": "docs/fixes/x",
            "pbi_ref": "docs/todos/pending/wi.md",
        });
        let out = maybe_apply_validation_from_validacion(root, &inputs, "argos").unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        let text = fs::read_to_string(root.join("docs/todos/pending/wi.md")).unwrap();
        assert!(text.contains("tracker_column: in_progress"));
        assert!(text.contains("validated_status: no apto"));
    }

    #[test]
    fn aut75_stamp_subscriber_does_not_move_state() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        file_project(root);
        write_file(
            root,
            "docs/todos/pending/wi.md",
            "---\ntracker_column: todo\n---\n\n# wi\n",
        );
        for event_type in [
            "Work_Initiated",
            "Validation_Started",
            "PullRequest_Presented",
            "PullRequest_Merged",
        ] {
            let event = json!({
                "event_type": event_type,
                "payload": {
                    "project_slug": "demo",
                    "tracker_ref": "OSC-1",
                    "pbi_ref": "docs/todos/pending/wi.md",
                    "branch": "feat/x",
                    "persist_ref": "docs/fixes/x",
                }
            });
            let (stamped, note) = super::super::handlers::tracker_stamp::stamp_event(root, &event)
                .expect("stamp");
            assert!(!stamped, "{event_type}");
            assert!(note.as_deref().unwrap_or("").contains("sellado en origen"));
        }
        let text = fs::read_to_string(root.join("docs/todos/pending/wi.md")).unwrap();
        assert!(text.contains("tracker_column: todo"));
    }

    #[test]
    fn pr_opened_remote_request_uses_linear_state_name() {
        let mut state_map = std::collections::BTreeMap::new();
        state_map.insert("in_review".into(), "In Review".into());
        let tc = TrackerConfig {
            team_key: "AUT".into(),
            project_id: None,
            state_map,
            label_hu: "HU".into(),
            label_pbi: "Pbi".into(),
            done_gate: project_binding::DoneGate::Linear,
        };
        let req = tracker_stamp::update_issue_state_request(&tc, "AUT-76", "in_review").unwrap();
        assert_eq!(req["state_name"], "In Review");
        assert_eq!(req["team_key"], "AUT");
        assert!(req.get("state_key").is_none());
    }
}
