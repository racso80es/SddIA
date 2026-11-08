//! Composition local: `Arc<dyn EventManagerPort>` productivo por hilo.

use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::Value;
use sddia_core::ports::event_manager::{
    append_progress_monotonic, events_redb_path, load_dispatched_event,
    publish_ecst as publish_ecst_on_port,
    record_delivery_ack as record_delivery_ack_on_port, record_ecst as record_ecst_on_port,
    EventManagerPort, GetDltSealRequest, GetDltSealResult, ListDlqRequest, ListEventsRequest,
    SealDltRequest, VerifyInlineEcstRequest, PublishSeal,
};
use sddia_event_store::{EventStore, Family, StoreMode};
use sddia_event_manager_host::resolve_productive_port;

/// Ruta de directorio (no fichero `.redb`).
pub fn is_filesystem_rel(raw: &str) -> bool {
    let s = raw.trim();
    !s.is_empty()
        && !s.ends_with(".redb")
        && (s.contains('/') || s.contains('\\') || s.starts_with('.'))
}

thread_local! {
    static PORT: RefCell<Option<Arc<dyn EventManagerPort>>> = const { RefCell::new(None) };
    static PORT_REPO: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

pub fn runtime_db_path(repo: &Path) -> PathBuf {
    events_redb_path(repo)
}

pub fn install_productive_port(repo: &Path) -> Result<(), String> {
    let port = resolve_productive_port(repo)?;
    PORT.with(|slot| *slot.borrow_mut() = Some(port));
    PORT_REPO.with(|slot| *slot.borrow_mut() = Some(repo.to_path_buf()));
    Ok(())
}

pub fn install_port(port: Arc<dyn EventManagerPort>) {
    PORT.with(|slot| *slot.borrow_mut() = Some(port));
    PORT_REPO.with(|slot| *slot.borrow_mut() = None);
}

pub fn clear_port() {
    PORT.with(|slot| *slot.borrow_mut() = None);
    PORT_REPO.with(|slot| *slot.borrow_mut() = None);
}

fn ensure_port_for_repo(repo: &Path) -> Result<(), String> {
    let stale = PORT_REPO.with(|slot| {
        slot.borrow()
            .as_deref()
            .map(|bound| bound != repo)
            .unwrap_or(PORT.with(|p| p.borrow().is_some()))
    });
    if stale {
        clear_port();
    }
    if PORT.with(|slot| slot.borrow().is_none()) {
        install_productive_port(repo)?;
    }
    Ok(())
}

fn with_port<R>(
    repo: &Path,
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String> {
    ensure_port_for_repo(repo)?;
    PORT.with(|slot| {
        let port = slot
            .borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado (composition root)".to_string())?;
        f(port.as_ref())
    })
}

fn events_family_dir(repo: &Path, family: &str) -> PathBuf {
    repo.join(format!(".events/{}", family.trim_start_matches("./")))
}

/// Publicación: FS/dual vía gate legacy de tests; productivo Redb vía puerto.
pub fn publish_ecst(repo: &Path, family: &str, event: &Value) -> Result<PublishSeal, String> {
    let mode = sddia_event_store::store_mode();
    if sddia_event_store::writes_filesystem(mode) {
        let dir = events_family_dir(repo, family);
        let seal = sddia_event_store::publish_ecst(repo, family, event, Some(&dir))
            .map_err(|e| e.to_string())?;
        return Ok(PublishSeal {
            event_id: seal.event_id,
            target_path: seal.target_path,
            logical_rel: seal.logical_rel,
        });
    }
    with_port(repo, |port| publish_ecst_on_port(port, repo, family, event))
}

pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), String> {
    let mode = sddia_event_store::store_mode();
    if sddia_event_store::writes_filesystem(mode) && !sddia_event_store::writes_store(mode) {
        let dir = events_family_dir(repo, family);
        sddia_event_store::publish_ecst(repo, family, event, Some(&dir))
            .map(|_| ())
            .map_err(|e| e.to_string())
    } else if sddia_event_store::writes_filesystem(mode) {
        publish_ecst(repo, family, event).map(|_| ())
    } else {
        with_port(repo, |port| record_ecst_on_port(port, repo, family, event))
    }
}

pub fn load_dispatched(
    repo: &Path,
    event_id: &str,
    logical: &Path,
) -> Result<Option<Value>, String> {
    let mode = sddia_event_store::store_mode();
    if mode == StoreMode::Filesystem {
        return sddia_event_store::load_dispatched_event(repo, event_id, logical)
            .map_err(|e| e.to_string());
    }
    with_port(repo, |port| load_dispatched_event(port, event_id, logical))
}

fn append_progress_leaf_fs(
    repo: &Path,
    correlation_id: &str,
    frame: &Value,
) -> Result<u64, String> {
    use crate::engine::fractal::load_progress_dir;
    let rel = load_progress_dir(repo).trim_start_matches("./").to_string();
    let leaf = repo.join(&rel).join(correlation_id);
    fs::create_dir_all(&leaf).map_err(|e| e.to_string())?;
    let trace_id = frame
        .get("trace_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("frame");
    let path = leaf.join(format!("{trace_id}.json"));
    fs::write(
        &path,
        serde_json::to_string(frame).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let n = fs::read_dir(&leaf).map(|d| d.count()).unwrap_or(1);
    Ok(n as u64)
}

pub fn append_progress(repo: &Path, correlation_id: &str, frame: &Value) -> Result<u64, String> {
    let mode = sddia_event_store::store_mode();
    if sddia_event_store::writes_store(mode) {
        with_port(repo, |port| append_progress_monotonic(port, correlation_id, frame))
    } else if sddia_event_store::writes_filesystem(mode) {
        append_progress_leaf_fs(repo, correlation_id, frame)
    } else {
        sddia_event_store::append_progress_monotonic(repo, correlation_id, frame)
            .map_err(|e| e.to_string())
    }
}

pub fn load_domain_ecsts(repo: &Path, limit: u32) -> Result<Vec<Value>, String> {
    let mode = sddia_event_store::store_mode();
    if sddia_event_store::writes_filesystem(mode) {
        let dirs = [
            repo.join(".events/pending"),
            repo.join(".events/processed"),
            repo.join(".events/dead-letter"),
        ];
        return sddia_event_store::load_domain_ecsts(repo, &dirs.to_vec())
            .map_err(|e| e.to_string())
            .map(|rows| rows.into_iter().take(limit as usize).collect());
    }

    let mut out = Vec::new();
    let mut seen = HashSet::new();

    if sddia_event_store::writes_store(mode) {
        if let Ok(store) = EventStore::open(runtime_db_path(repo)) {
            for row in store.list_family(Family::Domain).unwrap_or_default() {
                let id = row
                    .ecst
                    .get("event_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or(row.event_id.as_str())
                    .to_string();
                if seen.insert(id) {
                    out.push(row.ecst);
                }
                if out.len() >= limit as usize {
                    return Ok(out);
                }
            }
        }
    }

    with_port(repo, |port| {
        let listed = port
            .list_events(ListEventsRequest {
                family: None,
                event_type: None,
                limit: Some(limit),
                cursor: None,
            })
            .map_err(|e| e.to_string())?;
        for e in listed.events {
            let id = e
                .ecst
                .get("event_id")
                .and_then(|v| v.as_str())
                .unwrap_or(e.event_id.as_str())
                .to_string();
            if seen.insert(id) {
                out.push(e.ecst);
            }
        }
        Ok(())
    })?;

    Ok(out.into_iter().take(limit as usize).collect())
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
    ensure_port_for_repo(repo)?;
    PORT.with(|slot| {
        slot.borrow()
            .clone()
            .ok_or_else(|| "EventManagerPort no instalado".to_string())
    })
}

/// Laboratorio EDA en FS: evita puerto Redb en repos temporales de test.
#[cfg(test)]
pub mod test_lab {
    use sddia_event_store::{override_store_mode, StoreMode};

    pub struct FilesystemEdaGuard;

    impl Drop for FilesystemEdaGuard {
        fn drop(&mut self) {
            super::clear_port();
            override_store_mode(None);
        }
    }

    pub fn filesystem_eda() -> FilesystemEdaGuard {
        override_store_mode(Some(StoreMode::Filesystem));
        FilesystemEdaGuard
    }

    pub fn fs_lab_tempdir() -> (FilesystemEdaGuard, tempfile::TempDir) {
        (filesystem_eda(), tempfile::tempdir().expect("tempdir"))
    }
}
