//! Composition local: `Arc<dyn EventManagerPort>` productivo por hilo.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use sddia_core::ports::event_manager::{
    append_progress_monotonic, events_redb_path, load_dispatched_event,
    load_domain_ecsts as load_domain_ecsts_on_port, publish_ecst as publish_ecst_on_port,
    record_delivery_ack as record_delivery_ack_on_port, record_ecst as record_ecst_on_port,
    EventManagerPort, GetDltSealRequest, GetDltSealResult, ListDlqRequest, SealDltRequest,
    VerifyInlineEcstRequest, PublishSeal,
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

fn with_port<R>(
    repo: &Path,
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String> {
    if PORT.with(|slot| slot.borrow().is_none()) {
        install_productive_port(repo)?;
    }
    PORT.with(|slot| {
        let port = slot
            .borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado (composition root)".to_string())?;
        f(port.as_ref())
    })
}

pub fn publish_ecst(repo: &Path, family: &str, event: &Value) -> Result<PublishSeal, String> {
    with_port(repo, |port| publish_ecst_on_port(port, repo, family, event))
}

pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), String> {
    with_port(repo, |port| record_ecst_on_port(port, repo, family, event))
}

pub fn load_dispatched(
    repo: &Path,
    event_id: &str,
    logical: &Path,
) -> Result<Option<Value>, String> {
    with_port(repo, |port| load_dispatched_event(port, event_id, logical))
}

pub fn append_progress(repo: &Path, correlation_id: &str, frame: &Value) -> Result<u64, String> {
    with_port(repo, |port| append_progress_monotonic(port, correlation_id, frame))
}

pub fn load_domain_ecsts(repo: &Path, limit: u32) -> Result<Vec<Value>, String> {
    with_port(repo, |port| load_domain_ecsts_on_port(port, limit))
}

pub fn record_delivery_ack(repo: &Path, event_id: &str, delivery_id: &str) -> Result<(), String> {
    with_port(repo, |port| record_delivery_ack_on_port(port, event_id, delivery_id))
}

pub fn move_to_dlq(repo: &Path, event_id: &str, reason: &str) -> Result<(), String> {
    with_port(repo, |port| {
        port.move_to_dlq(sddia_core::ports::event_manager::MoveToDlqRequest {
            event_id: event_id.to_string(),
            reason: reason.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn purge_event(repo: &Path, event_id: &str) -> Result<(), String> {
    with_port(repo, |port| {
        port.purge_event(sddia_core::ports::event_manager::PurgeEventRequest {
            event_id: event_id.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn verify_inline_ecst(repo: &Path, event_id: &str, ecst: &Value) -> Result<(), String> {
    with_port(repo, |port| {
        port.verify_inline_ecst(VerifyInlineEcstRequest {
            event_id: event_id.to_string(),
            ecst: ecst.clone(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn seal_dlt(
    repo: &Path,
    event_id: &str,
    tx_hash: &str,
    digest: &str,
    scope: &str,
) -> Result<(), String> {
    with_port(repo, |port| {
        port.seal_dlt(SealDltRequest {
            event_id: event_id.to_string(),
            tx_hash: tx_hash.to_string(),
            digest: digest.to_string(),
            scope: scope.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn get_dlt_seal(repo: &Path, event_id: &str) -> Result<GetDltSealResult, String> {
    with_port(repo, |port| {
        port.get_dlt_seal(GetDltSealRequest {
            event_id: event_id.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn list_dlq_event_ids(repo: &Path, limit: u32) -> Result<Vec<String>, String> {
    with_port(repo, |port| {
        port.list_dlq(ListDlqRequest {
            limit: Some(limit),
        })
        .map(|r| r.events.into_iter().map(|e| e.event_id).collect())
        .map_err(|e| e.to_string())
    })
}

pub fn port_arc(repo: &Path) -> Result<Arc<dyn EventManagerPort>, String> {
    if PORT.with(|slot| slot.borrow().is_none()) {
        install_productive_port(repo)?;
    }
    PORT.with(|slot| {
        slot.borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado".to_string())
    })
}
