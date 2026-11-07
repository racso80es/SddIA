//! Composition local: `Arc<dyn EventManagerPort>` productivo por hilo.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use sddia_core::ports::event_manager::{
    append_progress_monotonic, events_redb_path, load_dispatched_event,
    load_domain_ecsts as load_domain_ecsts_on_port, publish_ecst as publish_ecst_on_port,
    record_delivery_ack as record_delivery_ack_on_port, record_ecst as record_ecst_on_port,
    EventManagerPort, PublishSeal,
};

/// Ruta de directorio (no fichero `.redb`).
pub fn is_filesystem_rel(raw: &str) -> bool {
    let s = raw.trim();
    !s.is_empty()
        && !s.ends_with(".redb")
        && (s.contains('/') || s.contains('\\') || s.starts_with('.'))
}
use sddia_event_manager_host::resolve_productive_port;

thread_local! {
    static PORT: RefCell<Option<Arc<dyn EventManagerPort>>> = const { RefCell::new(None) };
}

pub fn runtime_db_path(repo: &Path) -> PathBuf {
    events_redb_path(repo)
}

pub fn install_productive_port(repo: &Path) -> Result<(), String> {
    let port = resolve_productive_port(repo)?;
    PORT.with(|slot| *slot.borrow_mut() = Some(port));
    Ok(())
}

pub fn install_port(port: Arc<dyn EventManagerPort>) {
    PORT.with(|slot| *slot.borrow_mut() = Some(port));
}

pub fn clear_port() {
    PORT.with(|slot| *slot.borrow_mut() = None);
}

fn with_port<R>(f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>) -> Result<R, String> {
    PORT.with(|slot| {
        let port = slot
            .borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado (composition root)".to_string())?;
        f(port.as_ref())
    })
}

pub fn publish_ecst(repo: &Path, family: &str, event: &Value) -> Result<PublishSeal, String> {
    with_port(|port| publish_ecst_on_port(port, repo, family, event))
}

pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), String> {
    with_port(|port| record_ecst_on_port(port, repo, family, event))
}

pub fn load_dispatched(
    repo: &Path,
    event_id: &str,
    logical: &Path,
) -> Result<Option<Value>, String> {
    let _ = repo;
    with_port(|port| load_dispatched_event(port, event_id, logical))
}

pub fn append_progress(repo: &Path, correlation_id: &str, frame: &Value) -> Result<u64, String> {
    let _ = repo;
    with_port(|port| append_progress_monotonic(port, correlation_id, frame))
}

pub fn load_domain_ecsts(repo: &Path, limit: u32) -> Result<Vec<Value>, String> {
    let _ = repo;
    with_port(|port| load_domain_ecsts_on_port(port, limit))
}

pub fn record_delivery_ack(repo: &Path, event_id: &str, delivery_id: &str) -> Result<(), String> {
    let _ = repo;
    with_port(|port| record_delivery_ack_on_port(port, event_id, delivery_id))
}

pub fn move_to_dlq(repo: &Path, event_id: &str, reason: &str) -> Result<(), String> {
    let _ = repo;
    with_port(|port| {
        port.move_to_dlq(sddia_core::ports::event_manager::MoveToDlqRequest {
            event_id: event_id.to_string(),
            reason: reason.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn purge_event(repo: &Path, event_id: &str) -> Result<(), String> {
    let _ = repo;
    with_port(|port| {
        port.purge_event(sddia_core::ports::event_manager::PurgeEventRequest {
            event_id: event_id.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn port_arc() -> Result<Arc<dyn EventManagerPort>, String> {
    PORT.with(|slot| {
        slot.borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado".to_string())
    })
}
