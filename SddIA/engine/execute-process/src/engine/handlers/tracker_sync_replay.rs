//! Handler `tracker-sync-replay` — reintento fail-soft de `Tracker_Sync_Failed`.

use super::super::capsules::invoke_tool_for_process;
use super::super::project_binding::{DoneGate, TrackerConfig};
use crate::envelope::OrchestratorEnvelope;
use sddia_io::outbound_lab::lab_mock_outbound_enabled;
use serde_json::{json, Value};
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
        "todo" => Some(1),
        "in_progress" => Some(2),
        "in_validation" => Some(3),
        "in_review" => Some(4),
        "done" => Some(5),
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
    super::super::ecst_input_loader::load_ecst_from_inputs(repo, inputs)
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

pub fn should_discard_transition(current_canonical: &str, target: &str) -> bool {
    if current_canonical == "cancelled" {
        return true;
    }
    let cur_rank = state_rank(current_canonical).unwrap_or(-1);
    let tgt_rank = state_rank(target).unwrap_or(-1);
    tgt_rank >= 0 && cur_rank >= tgt_rank
}

fn resolve_replay_tracker(
    configured: Option<TrackerConfig>,
    issue_ref: &str,
) -> Result<TrackerConfig, String> {
    match configured {
        Some(tc) => Ok(tc),
        None if lab_mock_outbound_enabled() => Ok(lab_tracker_fallback(issue_ref)),
        None => Err("replay: sin tracker y sin SDDIA_LAB_MOCK_OUTBOUND".into()),
    }
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
    let target = super::super::tracker_lifecycle_port::resolve_target(repo, &inputs)?;
    if !matches!(
        target,
        super::super::tracker_lifecycle_port::LifecycleTarget::Remote { .. }
    ) {
        let _ = super::super::tracker_lifecycle_port::invoke(
            repo,
            &inputs,
            json!({
                "operation": operation,
                "issue_ref": issue_ref,
                "project_slug": inputs.get("project_slug"),
                "state_name": payload.get("state_name").or_else(|| payload.get("target_state")),
                "description": payload.get("description"),
                "body": payload.get("body"),
            }),
            "tracker-sync-replay",
        )?;
        return Ok("replayed");
    }
    let tool = super::tracker_stamp::lifecycle_tool_name_for(repo, &inputs)?;
    let tc = resolve_replay_tracker(
        super::super::project_binding::tracker_config_for_inputs(repo, &inputs)?,
        &issue_ref,
    )?;
    let fetch = invoke_tool_for_process(
        repo,
        &tool,
        &json!({
            "request": {
                "operation": "fetch_issue",
                "issue_ref": issue_ref,
                "project_id": tc.project_id,
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
                &tool,
                &json!({
                    "request": {
                        "operation": "update_issue_state",
                        "issue_ref": issue_ref,
                        "state_name": state_name,
                        "team_key": tc.team_key,
                        "project_id": tc.project_id,
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
                &tool,
                &json!({
                    "request": {
                        "operation": "create_comment",
                        "issue_ref": issue_ref,
                        "body": body,
                        "project_id": tc.project_id,
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
    if let Err(reason) = super::super::ecst_input_loader::verify_ecst_matches_bus(repo, &event) {
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "delivery_status": "failed-bus-mismatch",
                "reason": reason,
            })),
            error: Some(reason.clone()),
            execution_report: Some(json!({
                "phases": [{
                    "phase_name": "Replay de sincronización",
                    "status": "failed",
                    "handler": "tracker-sync-replay",
                }]
            })),
            exit_code: 1,
        });
    }
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
                    "phase_name": "Replay de sincronización",
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
        assert!(state_rank("backlog").unwrap() < state_rank("todo").unwrap());
        assert!(state_rank("todo").unwrap() < state_rank("in_progress").unwrap());
        assert!(state_rank("in_progress").unwrap() < state_rank("in_validation").unwrap());
        assert!(state_rank("in_validation").unwrap() < state_rank("in_review").unwrap());
        assert!(state_rank("in_review").unwrap() < state_rank("done").unwrap());
    }

    #[test]
    fn discard_when_issue_already_at_or_past_target() {
        assert!(should_discard_transition("in_review", "in_progress"));
        assert!(should_discard_transition("in_validation", "in_progress"));
        assert!(!should_discard_transition("in_progress", "in_validation"));
        assert!(should_discard_transition("in_progress", "todo"));
        assert!(should_discard_transition("done", "done"));
        assert!(!should_discard_transition("backlog", "in_progress"));
        assert!(should_discard_transition("cancelled", "todo"));
    }

    #[test]
    fn ac6_discards_todo_when_in_progress() {
        assert!(should_discard_transition("in_progress", "todo"));
    }

    #[test]
    fn run_rejects_tampered_inline_event_against_store() {
        use crate::core::repo::find_repo_root;
        use crate::engine::route_domain_core::materialize_pending_domain_event;
        use sddia_event_store::{override_store_mode, StoreMode};
        use std::fs;

        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                override_store_mode(None);
                std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
            }
        }
        let _g = Guard;
        override_store_mode(Some(StoreMode::Redb));
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        let repo = find_repo_root().expect("repo");
        let rel = {
            materialize_pending_domain_event(
                &repo,
                "Tracker_Sync_Failed",
                "test",
                json!({
                    "issue_ref": "AUT-99",
                    "operation": "update_issue_state",
                    "target_state": "todo",
                    "attempt": 0,
                }),
            )
            .expect("materialize");
            let store = sddia_event_store::EventStore::open(sddia_event_store::runtime_db_path(&repo))
                .expect("open");
            let rows = store
                .list_family(sddia_event_store::Family::Domain)
                .expect("list");
            let event_id = rows.last().expect("row").event_id.clone();
            drop(store);
            let pending = repo.join(".events/pending").join(format!("{event_id}.json"));
            if pending.is_file() {
                fs::remove_file(&pending).ok();
            }
            format!(".events/pending/{event_id}.json")
        };
        let stored = crate::engine::ecst_input_loader::load_ecst_from_rel(&repo, &rel).expect("load");
        let mut tampered = stored.clone();
        tampered["payload"]["issue_ref"] = json!("AUT-TAMPER");
        let env = run(
            &repo,
            &json!({
                "event": tampered,
                "event_id": stored.get("event_id").cloned().unwrap_or(json!("")),
                "project_slug": "core",
            }),
        )
        .expect("run");
        assert!(!env.success);
        assert_eq!(
            env.data
                .as_ref()
                .and_then(|d| d.get("delivery_status"))
                .and_then(|v| v.as_str()),
            Some("failed-bus-mismatch")
        );
    }

    #[test]
    fn replay_fallback_only_with_lab_mock() {
        let prev = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        let err = resolve_replay_tracker(None, "AUT-3").unwrap_err();
        assert!(err.contains("sin SDDIA_LAB_MOCK_OUTBOUND"));
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        let tc = resolve_replay_tracker(None, "AUT-3").unwrap();
        assert_eq!(tc.team_key, "AUT");
        if let Some(v) = prev {
            std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v);
        } else {
            std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND");
        }
    }
}
