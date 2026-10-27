//! Orquestación `refine-item`.

use super::dor;
use super::prepare::{
    ensure_canonical_markdown, item_type_from_fm, parse_properties, FetchSnapshot,
};
use crate::engine::capsules::invoke_tool_for_process;
use crate::engine::item_lifecycle_frontmatter;
use crate::engine::project_binding::{self, TrackerConfig};
use crate::engine::tracker_lifecycle_port;
use crate::engine::agent_runtime::invoke_agent_phase;
use crate::engine::item_lifecycle_frontmatter::{
    yaml_patch_refinement_refined, FM_REFINEMENT, REFINEMENT_PENDING,
};
use crate::engine::route_domain_core::materialize_pending_domain_event;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const PROCESS: &str = "refine-item";
const PHASE_MAYEUTA: &str = "Refinamiento Mayeuta";

fn str_field(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn workspace_dir(repo: &Path, execution_id: &str) -> PathBuf {
    repo.join(".SddIA/workspaces/refine-item").join(execution_id)
}

fn read_verdict(ws: &Path) -> Result<Value, String> {
    let path = ws.join("verdict.json");
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| format!("verdict.json ilegible: {e}"))
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("REFINE_INVALID: sin frontmatter".into());
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
    Err("REFINE_INVALID: frontmatter sin cierre".into())
}

fn fm_str(fm: &YamlValue, key: &str) -> Option<String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn title_from_item(fm: &YamlValue, body: &str) -> String {
    if let Some(t) = fm_str(fm, "title") {
        return t;
    }
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("# ") {
            return t.trim_start_matches('#').trim().to_string();
        }
    }
    "HU".to_string()
}

fn backlog_state_name(tc: &TrackerConfig) -> Option<String> {
    tc.state_map.get("backlog").cloned()
}

fn try_create_hu_issue(
    repo: &Path,
    tc: &TrackerConfig,
    title: &str,
    description: &str,
) -> Result<String, String> {
    let mut request = json!({
        "operation": "create_issue",
        "team_key": tc.team_key,
        "title": title,
        "description": description,
        "labels": [tc.label_hu.clone()],
    });
    if let Some(pid) = &tc.project_id {
        request["project_id"] = json!(pid);
    }
    if let Some(state) = backlog_state_name(tc) {
        request["state_name"] = json!(state);
    }
    let tool = super::super::tracker_stamp::lifecycle_tool_name(repo)?;
    let tool_body = invoke_tool_for_process(
        repo,
        &tool,
        &json!({ "request": request }),
        Some(PROCESS),
    )?;
    tool_body
        .get("issue_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| "LINEAR_CREATE_ISSUE_EMPTY: sin issue_ref".into())
}

fn emit_refined(
    repo: &Path,
    event_type: &str,
    ref_value: &str,
    project_slug: &str,
    tracker_ref: Option<&str>,
) -> Result<String, String> {
    let event_id = Uuid::new_v4().to_string();
    let occurred_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let ref_field = if event_type == "HU_Refined" {
        "hu_ref"
    } else {
        "pbi_ref"
    };
    let mut payload = json!({
        "event_id": event_id,
        "correlation_id": event_id,
        "source_process": PROCESS,
        ref_field: ref_value,
        "occurred_at": occurred_at,
        "project_slug": project_slug,
    });
    if let Some(tr) = tracker_ref.filter(|s| !s.is_empty()) {
        payload["tracker_ref"] = json!(tr);
    }
    materialize_pending_domain_event(repo, event_type, PROCESS, payload)
}

fn fetch_issue(
    repo: &Path,
    inputs: &Value,
    item_ref: &str,
    project_slug: &str,
) -> Result<Value, String> {
    tracker_lifecycle_port::invoke(
        repo,
        inputs,
        json!({
            "operation": "fetch_issue",
            "issue_ref": item_ref,
            "project_slug": project_slug,
        }),
        PROCESS,
    )
}

fn snapshot_from_fetch(body: &Value) -> Result<FetchSnapshot, String> {
    let description = body
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let identifier = body
        .get("identifier")
        .or_else(|| body.get("issue_ref"))
        .and_then(|v| v.as_str())
        .ok_or("fetch sin identifier")?
        .to_string();
    let state_name = body
        .pointer("/state")
        .and_then(|v| v.as_str())
        .or_else(|| body.get("state_name").and_then(|v| v.as_str()))
        .unwrap_or("Backlog")
        .to_string();
    let labels: Vec<String> = body
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    let updated_at = body
        .get("updated_at")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Ok(FetchSnapshot {
        description: description.to_string(),
        identifier,
        state_name,
        labels,
        updated_at,
    })
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let item_ref = str_field(inputs, "item_ref").ok_or("item_ref requerido")?;
    let bound = project_binding::bind(repo, inputs)?
        .ok_or("project_slug requerido para refine-item")?;
    let execution_id = str_field(inputs, "execution_id").unwrap_or_else(|| Uuid::new_v4().to_string());
    let ws = workspace_dir(repo, &execution_id);
    fs::create_dir_all(&ws).map_err(|e| e.to_string())?;

    let tracker_cfg = project_binding::tracker_config_for_inputs(repo, inputs)?;
    let target = tracker_lifecycle_port::resolve_target(repo, inputs)?;
    let remote = matches!(
        target,
        tracker_lifecycle_port::LifecycleTarget::Remote { .. }
    );

    let item_path = ws.join("item.md");
    let brief_path = ws.join("brief.json");
    let reentry = item_path.is_file();

    let (canonical, snap, initial_updated_at) = if reentry {
        let raw = fs::read_to_string(&item_path).map_err(|e| e.to_string())?;
        let brief: Value = serde_json::from_str(
            &fs::read_to_string(&brief_path).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let updated = brief
            .get("updated_at")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let snap = FetchSnapshot {
            description: raw.clone(),
            identifier: brief
                .get("tracker_ref")
                .and_then(|v| v.as_str())
                .unwrap_or(&item_ref)
                .to_string(),
            state_name: String::new(),
            labels: brief
                .get("labels")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            updated_at: updated.clone(),
        };
        (raw, snap, updated)
    } else {
        let fetched = fetch_issue(repo, inputs, &item_ref, &bound.slug)?;
        let snap = snapshot_from_fetch(&fetched)?;
        let canonical = ensure_canonical_markdown(
            &snap.description,
            &snap,
            &bound.slug,
            tracker_cfg.as_ref(),
        )?;
        fs::write(&item_path, &canonical).map_err(|e| e.to_string())?;
        let (fm, _) = parse_properties(&canonical)?;
        let item_type = item_type_from_fm(&fm, &snap.labels);
        let brief = json!({
            "item_type": item_type,
            "tracker_ref": snap.identifier,
            "updated_at": snap.updated_at,
            "labels": snap.labels,
            "item_ref": item_ref,
        });
        fs::write(&brief_path, serde_json::to_string_pretty(&brief).unwrap())
            .map_err(|e| e.to_string())?;
        let at = snap.updated_at.clone();
        (canonical, snap, at)
    };

    let (fm, body) = parse_properties(&canonical)?;
    let item_type = item_type_from_fm(&fm, &snap.labels);
    let mut tracker_ref = fm_str(&fm, "tracker_ref");
    let mut issue_target = tracker_ref
        .clone()
        .or_else(|| Some(item_ref.clone()))
        .unwrap_or_else(|| snap.identifier.clone());

    let verdict_path = ws.join("verdict.json");
    let skip_mayeuta = verdict_path.is_file();

    if !skip_mayeuta {
        let state = json!({
            "execution_id": execution_id,
            "workspace_path": ws.to_string_lossy(),
            "project_root": bound.project_root.to_string_lossy(),
        });
        let agent_entry = invoke_agent_phase(
            repo,
            PROCESS,
            PHASE_MAYEUTA,
            &[json!("agent:mayeuta")],
            inputs,
            &state,
            None,
        );
        let status = agent_entry
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("failed");
        if status == "simulated" || status == "awaiting_agents" {
            return Ok(envelope_status(
                &item_ref,
                &item_type,
                tracker_ref.as_deref(),
                &execution_id,
                &ws,
                "awaiting_agents",
                &[],
                None,
            ));
        }
        if status == "blocked" || status == "failed" {
            return Err(format!(
                "refine-item: fase Mayeuta {status}: {}",
                agent_entry
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("sin detalle")
            ));
        }
        if !verdict_path.is_file() {
            return Ok(envelope_status(
                &item_ref,
                &item_type,
                tracker_ref.as_deref(),
                &execution_id,
                &ws,
                "awaiting_agents",
                &[],
                None,
            ));
        }
    }

    if remote && !reentry {
        let fresh = fetch_issue(repo, inputs, &item_ref, &bound.slug)?;
        let fresh_snap = snapshot_from_fetch(&fresh)?;
        if !initial_updated_at.is_empty()
            && !fresh_snap.updated_at.is_empty()
            && fresh_snap.updated_at != initial_updated_at
        {
            return Ok(envelope_status(
                &item_ref,
                &item_type,
                tracker_ref.as_deref(),
                &execution_id,
                &ws,
                "conflict",
                &[],
                None,
            ));
        }
    }

    let verdict = read_verdict(&ws)?;
    let verdict_kind = verdict
        .get("verdict")
        .and_then(|v| v.as_str())
        .unwrap_or("needs_clarification");
    let open_questions: Vec<String> = verdict
        .get("open_questions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default();

    let refined_md_path = ws.join("item.refined.md");
    let refined_body = if refined_md_path.is_file() {
        let raw = fs::read_to_string(&refined_md_path).map_err(|e| e.to_string())?;
        if let Ok((_, b)) = split_frontmatter(&raw) {
            b
        } else {
            raw
        }
    } else {
        body.clone()
    };

    let dor_ok = dor::dor_ok(&item_type, &refined_body);
    let wants_refined = verdict_kind == "refined" && dor_ok && open_questions.is_empty();

    if !wants_refined {
        if remote {
            let gaps = dor::dor_gaps(&item_type, &refined_body);
            let comment = format!(
                "refine-item: clarificación pendiente (DoR / verdict).\n\nPreguntas: {}\n\nHuecos DoR: {}",
                if open_questions.is_empty() {
                    "(ver verdict.json)".to_string()
                } else {
                    open_questions.join("; ")
                },
                if gaps.is_empty() {
                    "ninguno".to_string()
                } else {
                    gaps.join(", ")
                }
            );
            let patch = serde_yaml::from_str(&format!("{FM_REFINEMENT}: {REFINEMENT_PENDING}"))
                .expect("patch");
            let sealed =
                item_lifecycle_frontmatter::seal_markdown(&canonical, &patch)?;
            tracker_lifecycle_port::invoke(
                repo,
                inputs,
                json!({
                    "operation": "update_issue_description",
                    "project_slug": bound.slug,
                    "issue_ref": &issue_target,
                    "description": sealed,
                }),
                PROCESS,
            )?;
            tracker_lifecycle_port::invoke(
                repo,
                inputs,
                json!({
                    "operation": "create_comment",
                    "project_slug": bound.slug,
                    "issue_ref": &issue_target,
                    "body": comment,
                }),
                PROCESS,
            )?;
        }
        return Ok(envelope_status(
            &item_ref,
            &item_type,
            tracker_ref.as_deref(),
            &execution_id,
            &ws,
            "needs_clarification",
            &open_questions,
            None,
        ));
    }

    if item_type == "hu" && remote && tracker_ref.is_none() {
        if let Some(tc) = tracker_cfg.as_ref() {
            let title = title_from_item(&fm, &refined_body);
            let description = refined_body.chars().take(8000).collect::<String>();
            if let Ok(issue_ref) = try_create_hu_issue(repo, tc, &title, &description) {
                tracker_ref = Some(issue_ref.clone());
                issue_target = issue_ref;
            }
        }
    }

    let refined_date = Utc::now().format("%Y-%m-%d").to_string();
    let mut patch = yaml_patch_refinement_refined(&refined_date);
    if let Some(map) = patch.as_mapping_mut() {
        map.insert(
            YamlValue::String("refined_by".into()),
            YamlValue::String("mayeuta".into()),
        );
        map.insert(
            YamlValue::String("refinement_execution_id".into()),
            YamlValue::String(execution_id.clone()),
        );
        if let Some(tr) = &tracker_ref {
            map.insert(
                YamlValue::String("tracker_ref".into()),
                YamlValue::String(tr.clone()),
            );
        }
    }
    let body_clean = refined_body
        .replace("<!-- refined:mayeuta -->", "")
        .trim()
        .to_string();
    let sealed_fm =
        item_lifecycle_frontmatter::seal_markdown(&canonical, &patch)?;
    let (fm_text, _) = split_frontmatter(&sealed_fm)?;
    let sealed = format!("---\n{fm_text}---\n\n{body_clean}\n");

    if remote {
        tracker_lifecycle_port::invoke(
            repo,
            inputs,
            json!({
                "operation": "update_issue_description",
                "project_slug": bound.slug,
                "issue_ref": &issue_target,
                "description": sealed,
            }),
            PROCESS,
        )?;
    }

    let event_type = if item_type == "hu" {
        "HU_Refined"
    } else {
        "PBI_Refined"
    };
    let event_path = emit_refined(
        repo,
        event_type,
        &item_ref,
        &bound.slug,
        tracker_ref.as_deref(),
    )?;

    Ok(envelope_status(
        &item_ref,
        &item_type,
        tracker_ref.as_deref(),
        &execution_id,
        &ws,
        "refined",
        &[],
        Some(event_path),
    ))
}

fn envelope_status(
    item_ref: &str,
    item_type: &str,
    tracker_ref: Option<&str>,
    execution_id: &str,
    ws: &Path,
    status: &str,
    open_questions: &[String],
    event_path: Option<String>,
) -> OrchestratorEnvelope {
    OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "item_ref": item_ref,
            "item_type": item_type,
            "tracker_ref": tracker_ref,
            "execution_id": execution_id,
            "workspace_path": ws.to_string_lossy(),
            "status": status,
            "open_questions": open_questions,
            "event_path": event_path,
        })),
        error: None,
        execution_report: Some(json!({
            "phases": [
                {"phase_name": "Preparación", "status": "executed", "handler": PROCESS},
                {"phase_name": PHASE_MAYEUTA, "status": "executed", "handler": PROCESS},
                {"phase_name": "Validación DoR", "status": "executed", "handler": PROCESS},
                {"phase_name": "Sellado Argos", "status": "executed", "handler": PROCESS},
            ]
        })),
        exit_code: 0,
    }
}
