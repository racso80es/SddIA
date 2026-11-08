//! Productivo: EventManagerPort (sin `sddia_event_store` directo).

use std::path::{Path, PathBuf};

use serde_json::Value;
use sddia_core::ports::event_manager::{
    publish_ecst, GetEventRequest, ListDlqRequest, ListEventsRequest, ListPendingRequest,
    ListProgressRequest,
};
use sddia_event_manager_host::{events_redb_path, resolve_productive_port};

pub fn db_path(repo: &Path) -> PathBuf {
    events_redb_path(repo)
}

pub fn store_active(repo: &Path) -> bool {
    db_path(repo).is_file()
}

pub fn publish_pending(repo: &Path, event: &Value) -> Result<(), String> {
    let port = resolve_productive_port(repo)?;
    publish_ecst(port.as_ref(), repo, "pending", event).map(|_| ())
}

pub fn domain_event(
    repo: &Path,
    event_id: &str,
) -> Option<(PathBuf, Value, bool)> {
    let port = resolve_productive_port(repo).ok()?;
    let row = port
        .get_event(GetEventRequest {
            event_id: event_id.to_string(),
        })
        .ok()?;
    let in_dead = port
        .list_dlq(ListDlqRequest { limit: Some(10_000) })
        .ok()
        .map(|r| r.events.iter().any(|e| e.event_id == event_id))
        .unwrap_or(false);
    Some((db_path(repo), row.ecst, in_dead))
}

pub fn pec_for_correlation(repo: &Path, correlation_id: &str) -> Option<Value> {
    let port = resolve_productive_port(repo).ok()?;
    let rows = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: Some("Process_Execution_Completed".into()),
            limit: Some(500),
            cursor: None,
        })
        .ok()?;
    let mut best: Option<Value> = None;
    let mut best_ts = String::new();
    for row in rows.events {
        let cid = row
            .ecst
            .get("payload")
            .and_then(|p| p.get("correlation_id"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if cid != correlation_id {
            continue;
        }
        let ts = row
            .ecst
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if best.is_none() || ts >= best_ts {
            best_ts = ts;
            best = Some(row.ecst);
        }
    }
    if best.is_some() {
        return best;
    }
    let listed = port
        .list_pending(ListPendingRequest {
            limit: Some(500),
        })
        .ok()?;
    for row in listed.events {
        if row.ecst.get("event_type").and_then(|v| v.as_str())
            != Some("Process_Execution_Completed")
        {
            continue;
        }
        let cid = row
            .ecst
            .get("payload")
            .and_then(|p| p.get("correlation_id"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if cid != correlation_id {
            continue;
        }
        let ts = row
            .ecst
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if best.is_none() || ts >= best_ts {
            best_ts = ts;
            best = Some(row.ecst);
        }
    }
    best
}

pub fn list_telemetry_ecsts(repo: &Path, limit: u32) -> Vec<Value> {
    let Ok(port) = resolve_productive_port(repo) else {
        return Vec::new();
    };
    port.list_events(ListEventsRequest {
        family: None,
        event_type: None,
        limit: Some(limit),
        cursor: None,
    })
    .map(|r| r.events.into_iter().map(|e| e.ecst).collect())
    .unwrap_or_default()
}

pub fn list_progress_frames(repo: &Path, correlation_id: &str) -> Vec<Value> {
    let Ok(port) = resolve_productive_port(repo) else {
        return Vec::new();
    };
    port.list_progress(ListProgressRequest {
        trace_id: None,
        correlation_id: Some(correlation_id.to_string()),
    })
    .map(|r| r.frames)
    .unwrap_or_default()
}
