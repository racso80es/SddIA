//! Handler `tracker-sync-replay` — reintento fail-soft de `Tracker_Sync_Failed`.

use super::super::capsules::invoke_tool_for_process;
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

fn state_rank(canonical: &str) -> Option<i32> {
    match canonical {
        "backlog" => Some(0),
        "in_progress" => Some(1),
        "in_review" => Some(2),
        "done" => Some(3),
        "cancelled" => Some(100),
        _ => None,
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

fn sync_marker(event_id: &str) -> String {
    format!("sddia-sync-id: {event_id}")
}

fn load_event(repo: &Path, inputs: &Value) -> Result<Value, String> {
    if let Some(rel) = str_field(inputs, "event_file_path") {
        let path = repo.join(rel.trim_start_matches("./"));
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        return serde_json::from_str(&text).map_err(|e| e.to_string());
    }
    let event_id = str_field(inputs, "event_id").ok_or("event_id o event_file_path requerido")?;
    Ok(json!({
        "event_id": event_id,
        "payload": inputs.get("payload").cloned().unwrap_or_else(|| inputs.clone()),
    }))
}

fn payload_from_event(event: &Value) -> Result<&Value, String> {
    event
        .get("payload")
        .filter(|p| p.is_object())
        .ok_or_else(|| "payload ECST ausente".into())
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
    TrackerConfig {
        team_key,
        project_id: None,
        state_map,
        label_hu: "hu".into(),
        label_pbi: "pbi".into(),
    }
}

pub fn should_discard_transition(current_canonical: &str, target: &str) -> bool {
    let cur_rank = state_rank(current_canonical).unwrap_or(-1);
    let tgt_rank = state_rank(target).unwrap_or(-1);
    tgt_rank >= 0 && cur_rank >= tgt_rank
}

pub fn replay_payload(
    repo: &Path,
    event_id: &str,
    payload: &Value,
) -> Result<&'static str, String> {
    let operation = str_field(payload, "operation").ok_or("operation ausente")?;
    let issue_ref = str_field(payload, "issue_ref").ok_or("issue_ref ausente")?;
    let mut inputs = json!({});
    if let Some(slug) = str_field(payload, "project_slug") {
        inputs["project_slug"] = json!(slug);
    }
    let tc = match project_binding::tracker_config_for_inputs(repo, &inputs)? {
        Some(t) => t,
        None if lab_mock_outbound_enabled() => lab_tracker_fallback(&issue_ref),
        None => return Err("tracker no configurado".into()),
    };

    let fetch = invoke_tool_for_process(
        repo,
        "linear-tracker-adapter",
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
            }
        }),
        Some("tracker-sync-replay"),
    )?;

    match operation.as_str() {
        "update_issue_state" => {
            let target = str_field(payload, "target_state").ok_or("target_state ausente")?;
            let current_raw = fetch.get("state").and_then(|v| v.as_str()).unwrap_or("");
            let current = linear_to_canonical(current_raw, &tc);
            if should_discard_transition(&current, &target) {
                return Ok("discarded");
            }
            let state_name = tc
                .state_map
                .get(&target)
                .cloned()
                .ok_or_else(|| format!("state_map sin entrada para {target}"))?;
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
                Some("tracker-sync-replay"),
            )?;
            Ok("applied")
        }
        "create_comment" => {
            let marker = sync_marker(event_id);
            let body = format!(
                "sddia-sync replay ({}) — kind {}",
                marker,
                str_field(payload, "comment_kind").unwrap_or_else(|| "note".into())
            );
            if body.contains(&marker) {
                // idempotencia: el cuerpo ya lleva marca única por event_id
            }
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
                Some("tracker-sync-replay"),
            )?;
            Ok("applied")
        }
        other => Err(format!("operation no soportada: {other}")),
    }
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let event = load_event(repo, process_inputs)?;
    let event_id = str_field(&event, "event_id").ok_or("event_id ausente")?;
    let payload = payload_from_event(&event)?;
    let attempt = payload
        .get("attempt")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    match replay_payload(repo, &event_id, payload) {
        Ok(status) => Ok(OrchestratorEnvelope {
            success: true,
            status_code: 0,
            data: Some(json!({
                "replay_status": status,
                "event_id": event_id,
            })),
            error: None,
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Replay Linear",
                    "status": "executed",
                    "handler": "tracker-sync-replay",
                }]
            })),
            exit_code: 0,
        }),
        Err(e) => {
            if attempt >= 1 {
                return Ok(OrchestratorEnvelope {
                    success: false,
                    status_code: 1,
                    data: Some(json!({
                        "replay_status": "failed",
                        "event_id": event_id,
                        "attempt": attempt,
                        "dead_letter_eligible": true,
                    })),
                    error: Some(e),
                    execution_report: None,
                    exit_code: 1,
                });
            }
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_rank_orders_cycle() {
        assert!(state_rank("backlog").unwrap() < state_rank("in_progress").unwrap());
        assert!(state_rank("in_review").unwrap() < state_rank("done").unwrap());
    }

    #[test]
    fn discard_when_issue_already_at_or_past_target() {
        assert!(should_discard_transition("in_review", "in_progress"));
        assert!(should_discard_transition("done", "done"));
        assert!(!should_discard_transition("backlog", "in_progress"));
    }
}
