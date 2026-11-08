//! Helpers provider-neutral sobre `EventManagerPort` (sustituyen gate del store legacy).

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::ports::event_manager::contract::{
    AppendTraceRequest, DeliveryStatus, GetEventRequest, ListEventsRequest, ListPendingRequest,
    PublishEcstRequest, RecordDeliveryRequest,
};
use crate::ports::event_manager::error::EventManagerError;
use crate::ports::event_manager::port::EventManagerPort;

/// Ruta canónica Cúmulo del store de eventos.
pub fn events_redb_path(repo: &Path) -> PathBuf {
    repo.join(".SddIA/runtime/events.redb")
}

#[derive(Clone, Debug)]
pub struct PublishSeal {
    pub event_id: String,
    pub target_path: PathBuf,
    pub logical_rel: String,
}

fn event_token(event: &Value, key: &str) -> Result<String, String> {
    event
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{key} vacío en ECST"))
}

fn logical_event_rel(family: &str, event_id: &str) -> String {
    let fam = family.trim().trim_start_matches("./");
    format!(".events/{fam}/{event_id}.json")
}

fn trace_id_from_event(event: &Value) -> Option<String> {
    event
        .get("correlation_id")
        .or_else(|| event.get("trace_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Publica ECST al puerto; idempotente ante duplicado.
pub fn publish_ecst(
    port: &dyn EventManagerPort,
    repo: &Path,
    family: &str,
    event: &Value,
) -> Result<PublishSeal, String> {
    let event_id = event_token(event, "event_id")?;
    let req = PublishEcstRequest {
        event_id: event_id.clone(),
        ecst: event.clone(),
        trace_id: trace_id_from_event(event),
    };
    match port.publish_ecst(req) {
        Ok(_) => {}
        Err(EventManagerError::InvalidRequest(msg)) if msg.contains("duplicate") => {}
        Err(err) => return Err(err.to_string()),
    }
    let logical_rel = logical_event_rel(family, &event_id);
    Ok(PublishSeal {
        event_id,
        target_path: events_redb_path(repo),
        logical_rel,
    })
}

/// Registro de dominio: alias de `publish_ecst` con familia fija.
pub fn record_ecst(
    port: &dyn EventManagerPort,
    repo: &Path,
    family: &str,
    event: &Value,
) -> Result<(), String> {
    publish_ecst(port, repo, family, event).map(|_| ())
}

/// Carga ECST pendiente por `event_id` (ignora ruta lógica legacy).
pub fn load_dispatched_event(
    port: &dyn EventManagerPort,
    event_id: &str,
    _logical_path: &Path,
) -> Result<Option<Value>, String> {
    match port.get_event(GetEventRequest {
        event_id: event_id.to_string(),
    }) {
        Ok(row) => Ok(Some(row.ecst)),
        Err(EventManagerError::NotFound(_)) => Ok(None),
        Err(err) => Err(err.to_string()),
    }
}

/// Append de frame de progreso; devuelve índice sintético (conteo previo + 1).
pub fn append_progress_monotonic(
    port: &dyn EventManagerPort,
    correlation_id: &str,
    frame: &Value,
) -> Result<u64, String> {
    let frames = port
        .list_traces(crate::ports::event_manager::contract::ListTracesRequest {
            trace_id: correlation_id.to_string(),
        })
        .map(|r| r.frames.len() as u64)
        .unwrap_or(0);
    port.append_trace(AppendTraceRequest {
        trace_id: correlation_id.to_string(),
        frame: frame.clone(),
    })
    .map_err(|e| e.to_string())?;
    Ok(frames + 1)
}

pub fn record_delivery_ack(
    port: &dyn EventManagerPort,
    event_id: &str,
    delivery_id: &str,
) -> Result<(), String> {
    port.record_delivery(RecordDeliveryRequest {
        event_id: event_id.to_string(),
        delivery_id: delivery_id.to_string(),
        status: DeliveryStatus::Ack,
        attempt: None,
    })
    .map_err(|e| e.to_string())
}

pub fn record_delivery_nack(
    port: &dyn EventManagerPort,
    event_id: &str,
    delivery_id: &str,
) -> Result<(), String> {
    port.record_delivery(RecordDeliveryRequest {
        event_id: event_id.to_string(),
        delivery_id: delivery_id.to_string(),
        status: DeliveryStatus::Nack,
        attempt: None,
    })
    .map_err(|e| e.to_string())
}

/// Lista ECST de dominio / histórico reciente (incluye eventos con lease).
pub fn load_domain_ecsts(port: &dyn EventManagerPort, limit: u32) -> Result<Vec<Value>, String> {
    let listed = port
        .list_events(ListEventsRequest {
            family: None,
            event_type: None,
            limit: Some(limit),
            cursor: None,
        })
        .map_err(|e| e.to_string())?;
    Ok(listed.events.into_iter().map(|e| e.ecst).collect())
}

pub fn list_pending_ecsts(
    port: &dyn EventManagerPort,
    limit: u32,
) -> Result<Vec<Value>, String> {
    let listed = port
        .list_pending(ListPendingRequest {
            limit: Some(limit),
        })
        .map_err(|e| e.to_string())?;
    Ok(listed.events.into_iter().map(|e| e.ecst).collect())
}
