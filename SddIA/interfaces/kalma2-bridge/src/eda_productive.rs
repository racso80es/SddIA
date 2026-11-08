//! Productivo: EventManagerPort (sin `sddia_event_store` directo).

use std::path::{Path, PathBuf};

use serde_json::Value;
use sddia_core::ports::event_manager::{
    publish_ecst, GetEventRequest, ListDlqRequest, ListEventsRequest, ListPendingRequest,
    ListProgressRequest,
};
use sddia_event_manager_host::{events_redb_path, with_productive_port};

pub fn db_path(repo: &Path) -> PathBuf {
    events_redb_path(repo)
}

pub fn store_active(repo: &Path) -> bool {
    db_path(repo).is_file()
}

pub fn publish_pending(repo: &Path, event: &Value) -> Result<(), String> {
    with_productive_port(repo, "publish_pending", |port| {
        publish_ecst(port, repo, "pending", event).map(|_| ())
    })
}

pub fn domain_event(
    repo: &Path,
    event_id: &str,
) -> Option<(PathBuf, Value, bool)> {
    with_productive_port(repo, "domain_event", |port| {
        let row = port
            .get_event(GetEventRequest {
                event_id: event_id.to_string(),
            })
            .map_err(|e| e.to_string())?;
        let in_dead = port
            .list_dlq(ListDlqRequest { limit: Some(10_000) })
            .map(|r| r.events.iter().any(|e| e.event_id == event_id))
            .unwrap_or(false);
        Ok((db_path(repo), row.ecst, in_dead))
    })
    .ok()
}

pub fn pec_for_correlation(repo: &Path, correlation_id: &str) -> Option<Value> {
    with_productive_port(repo, "pec_for_correlation", |port| {
        let rows = port
            .list_events(ListEventsRequest {
                family: None,
                event_type: Some("Process_Execution_Completed".into()),
                limit: Some(500),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
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
            return Ok(best);
        }
        let listed = port
            .list_pending(ListPendingRequest {
                limit: Some(500),
            })
            .map_err(|e| e.to_string())?;
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
        Ok(best)
    })
    .ok()
    .flatten()
}

pub fn list_telemetry_ecsts(repo: &Path, limit: u32) -> Vec<Value> {
    with_productive_port(repo, "list_telemetry", |port| {
        port.list_events(ListEventsRequest {
            family: None,
            event_type: None,
            limit: Some(limit),
            cursor: None,
        })
        .map(|r| r.events.into_iter().map(|e| e.ecst).collect())
        .map_err(|e| e.to_string())
    })
    .unwrap_or_default()
}

pub fn list_progress_frames(repo: &Path, correlation_id: &str) -> Vec<Value> {
    with_productive_port(repo, "list_progress", |port| {
        port.list_progress(ListProgressRequest {
            trace_id: None,
            correlation_id: Some(correlation_id.to_string()),
        })
        .map(|r| r.frames)
        .map_err(|e| e.to_string())
    })
    .unwrap_or_default()
}
