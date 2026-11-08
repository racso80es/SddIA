//! Inspección del bus vía `EventManagerPort` (^1.1.0).

use std::path::Path;

use crate::engine::eda_port;
use sddia_core::ports::event_manager::{ListDlqRequest, ListPendingRequest, ReplayDlqRequest};
use serde_json::{json, Value};

pub const EVENT_STORE_DISABLED: &str = "EVENT_STORE_DISABLED";

pub fn publish_cli(repo: &Path, inputs: &Value) -> (i32, Value) {
    let family = inputs
        .get("family")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("pending");
    let mut event = inputs.get("event").cloned().unwrap_or_else(|| inputs.clone());
    if event
        .get("event_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .is_empty()
    {
        return fail("event_id requerido");
    }
    if matches!(family, "pending" | "domain") {
        if let Err(err) = sddia_eda_catalog::stamp_published_event(repo, &mut event) {
            return fail(err);
        }
    }
    match eda_port::publish_ecst(repo, family, &event) {
        Ok(seal) => ok(json!({
            "event_id": seal.event_id,
            "logical_rel": seal.logical_rel,
            "target_path": seal.target_path.to_string_lossy(),
        })),
        Err(err) => fail(err.to_string()),
    }
}

pub fn run(repo: &Path, flag: &str, inputs: &Value) -> (i32, Value) {
    match flag {
        "--query-events" => query_events(repo, inputs),
        "--inspect-dead-letter" => inspect_dead_letter(repo),
        "--replay-dead-letter" => replay_dead_letter(repo, inputs),
        "--reset-subscriber-delivery" => reset_subscriber_delivery(repo, inputs),
        other => fail(format!("flag desconocida: {other}")),
    }
}

fn fail(error: impl Into<String>) -> (i32, Value) {
    let error = error.into();
    (
        1,
        json!({
            "success": false,
            "status_code": 1,
            "exitCode": 1,
            "data": null,
            "error": error,
        }),
    )
}

fn ok(data: Value) -> (i32, Value) {
    (
        0,
        json!({
            "success": true,
            "status_code": 0,
            "exitCode": 0,
            "data": data,
            "error": null,
        }),
    )
}

fn query_events(repo: &Path, inputs: &Value) -> (i32, Value) {
    let event_type = inputs
        .get("event_type")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let limit = inputs
        .get("limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(50)
        .min(5000) as u32;
    let _family = inputs.get("family").and_then(|v| v.as_str());
    let listed = match eda_port::with_event_port(repo, "query_events", |port| {
        port.list_pending(ListPendingRequest {
            limit: Some(limit),
        })
        .map_err(|e| e.to_string())
    }) {
        Ok(r) => r,
        Err(e) => return fail(e),
    };
    let mut rows: Vec<Value> = listed
        .events
        .into_iter()
        .filter_map(|e| {
            if let Some(want) = event_type {
                if e.ecst.get("event_type").and_then(|v| v.as_str()) != Some(want) {
                    return None;
                }
            }
            Some(json!({
                "event_id": e.event_id,
                "event_type": e.ecst.get("event_type"),
                "ecst": e.ecst,
            }))
        })
        .collect();
    rows.truncate(limit as usize);
    ok(json!({ "events": rows }))
}

fn inspect_dead_letter(repo: &Path) -> (i32, Value) {
    let listed = match eda_port::with_event_port(repo, "list_dlq", |port| {
        port.list_dlq(ListDlqRequest { limit: Some(500) })
            .map_err(|e| e.to_string())
    }) {
        Ok(r) => r,
        Err(e) => return fail(e),
    };
    let rows: Vec<Value> = listed
        .events
        .into_iter()
        .map(|e| {
            json!({
                "event_id": e.event_id,
                "cause": e.reason,
            })
        })
        .collect();
    ok(json!({ "dead_letters": rows }))
}

fn replay_dead_letter(repo: &Path, inputs: &Value) -> (i32, Value) {
    let Some(event_id) = inputs
        .get("event_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return fail("event_id requerido");
    };
    match eda_port::with_event_port(repo, "replay_dlq", |port| {
        port.replay_dlq(ReplayDlqRequest {
            event_id: Some(event_id.to_string()),
            all: None,
        })
        .map_err(|e| e.to_string())
    }) {
        Ok(()) => ok(json!({ "event_id": event_id, "replayed": true })),
        Err(e) => fail(e),
    }
}

fn reset_subscriber_delivery(_repo: &Path, _inputs: &Value) -> (i32, Value) {
    fail("reset-subscriber-delivery no soportado en EventManager 1.1 (sin matriz subscriber_delivery)")
}
