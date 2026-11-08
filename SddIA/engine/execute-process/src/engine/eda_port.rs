//! Composition local: mock por hilo en lab; productivo scoped vía host (MAN-38).

use std::cell::RefCell;
use std::collections::HashSet;
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
use sddia_event_manager_host::{resolve_productive_port, with_productive_port};

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

/// Mock / DI de test: persiste en thread-local sin abrir Redb.
pub fn install_port(port: Arc<dyn EventManagerPort>) {
    PORT.with(|slot| *slot.borrow_mut() = Some(port));
    PORT_REPO.with(|slot| *slot.borrow_mut() = None);
}

pub fn clear_port() {
    PORT.with(|slot| *slot.borrow_mut() = None);
    PORT_REPO.with(|slot| *slot.borrow_mut() = None);
}

fn test_mock_installed() -> bool {
    PORT.with(|slot| slot.borrow().is_some())
        && PORT_REPO.with(|slot| slot.borrow().is_none())
}

/// No instala puerto productivo (MAN-38). Compat: no-op.
pub fn install_productive_port(_repo: &Path) -> Result<(), String> {
    Ok(())
}

/// Deprecado: composición lazy por operación EDA (MAN-38).
pub fn bootstrap_composition_root(_repo: &Path) -> Result<(), String> {
    Ok(())
}

/// Puente apertura acotada (p. ej. sonda de topología) sin cache productivo.
pub fn probe_productive_handshake(repo: &Path) -> Result<(), String> {
    with_productive_port(repo, "probe_handshake", |port| {
        let hs = port
            .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .map_err(|e| e.to_string())?;
        if hs.compatible {
            Ok(())
        } else {
            Err("EventManagerPort incompatible".into())
        }
    })
}

/// Ejecuta `f` contra mock de test o apertura productiva scoped.
pub fn with_event_port<R>(
    repo: &Path,
    op_label: &'static str,
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String> {
    with_port(repo, op_label, f)
}

fn with_port<R>(
    repo: &Path,
    op_label: &'static str,
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String> {
    if PORT_REPO.with(|slot| slot.borrow().is_some()) {
        clear_port();
    }
    if test_mock_installed() {
        return PORT.with(|slot| {
            let port = slot
                .borrow()
                .clone()
                .ok_or_else(|| "EventManagerPort mock no instalado".to_string())?;
            f(port.as_ref())
        });
    }
    with_productive_port(repo, op_label, f)
}

/// Publicación productiva vía `EventManagerPort`.
pub fn publish_ecst(repo: &Path, family: &str, event: &Value) -> Result<PublishSeal, String> {
    #[cfg(test)]
    {
        use sddia_event_store::writes_filesystem;
        if writes_filesystem(sddia_event_store::store_mode()) {
            return crate::engine::eda_port_legacy::publish_ecst(repo, family, event);
        }
    }
    with_port(repo, "publish_ecst", |port| publish_ecst_on_port(port, repo, family, event))
}

pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), String> {
    #[cfg(test)]
    {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) {
            return crate::engine::eda_port_legacy::record_ecst(repo, family, event);
        }
    }
    with_port(repo, "record_ecst", |port| record_ecst_on_port(port, repo, family, event))
}

pub fn load_dispatched(
    repo: &Path,
    event_id: &str,
    logical: &Path,
) -> Result<Option<Value>, String> {
    #[cfg(test)]
    {
        if sddia_event_store::store_mode() == sddia_event_store::StoreMode::Filesystem {
            return crate::engine::eda_port_legacy::load_dispatched(repo, event_id, logical);
        }
    }
    with_port(repo, "load_dispatched", |port| {
        load_dispatched_event(port, event_id, logical)
    })
}

pub fn append_progress(repo: &Path, correlation_id: &str, frame: &Value) -> Result<u64, String> {
    #[cfg(test)]
    {
        let mode = sddia_event_store::store_mode();
        if !sddia_event_store::writes_store(mode) && sddia_event_store::writes_filesystem(mode) {
            return crate::engine::eda_port_legacy::append_progress(repo, correlation_id, frame);
        }
    }
    with_port(repo, "append_progress", |port| {
        append_progress_monotonic(port, correlation_id, frame)
    })
}

pub fn load_domain_ecsts(repo: &Path, limit: u32) -> Result<Vec<Value>, String> {
    #[cfg(test)]
    {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) {
            return crate::engine::eda_port_legacy::load_domain_ecsts(repo, limit);
        }
        if sddia_event_store::writes_store(mode) {
            let legacy = crate::engine::eda_port_legacy::load_domain_ecsts(repo, limit)?;
            if !legacy.is_empty() {
                return Ok(legacy);
            }
        }
    }

    let out = with_port(repo, "load_domain_ecsts", |port| {
        let mut out = Vec::new();
        let mut seen = HashSet::new();
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
        Ok(out)
    })?;
    Ok(out.into_iter().take(limit as usize).collect())
}

pub fn record_delivery_ack(repo: &Path, event_id: &str, delivery_id: &str) -> Result<(), String> {
    with_port(repo, "record_delivery_ack", |port| {
        record_delivery_ack_on_port(port, event_id, delivery_id)
    })
}

pub fn move_to_dlq(repo: &Path, event_id: &str, reason: &str) -> Result<(), String> {
    with_port(repo, "move_to_dlq", |port| {
        port.move_to_dlq(sddia_core::ports::event_manager::MoveToDlqRequest {
            event_id: event_id.to_string(),
            reason: reason.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn purge_event(repo: &Path, event_id: &str) -> Result<(), String> {
    with_port(repo, "purge_event", |port| {
        port.purge_event(sddia_core::ports::event_manager::PurgeEventRequest {
            event_id: event_id.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn verify_inline_ecst(repo: &Path, event_id: &str, ecst: &Value) -> Result<(), String> {
    with_port(repo, "verify_inline_ecst", |port| {
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
    with_port(repo, "seal_dlt", |port| {
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
    with_port(repo, "get_dlt_seal", |port| {
        port.get_dlt_seal(GetDltSealRequest {
            event_id: event_id.to_string(),
        })
        .map_err(|e| e.to_string())
    })
}

pub fn list_dlq_event_ids(repo: &Path, limit: u32) -> Result<Vec<String>, String> {
    with_port(repo, "list_dlq", |port| {
        port.list_dlq(ListDlqRequest {
            limit: Some(limit),
        })
        .map(|r| r.events.into_iter().map(|e| e.event_id).collect())
        .map_err(|e| e.to_string())
    })
}

/// Solo tests / ventanas breves; preferir operaciones scoped en productivo.
pub fn port_arc(repo: &Path) -> Result<Arc<dyn EventManagerPort>, String> {
    if test_mock_installed() {
        return PORT.with(|slot| {
            slot.borrow()
                .clone()
                .ok_or_else(|| "EventManagerPort mock no instalado".to_string())
        });
    }
    resolve_productive_port(repo)
}

#[cfg(test)]
mod scoped_tests {
    use super::*;
    use sddia_core::ports::event_manager::InMemoryEventManagerMock;

    #[test]
    fn productive_not_left_in_thread_local_after_op() {
        let dir = tempfile::tempdir().expect("tempdir");
        let repo = dir.path();
        with_port(repo, "unit_ca6", |port| {
            let _ = port.handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            });
            Ok(())
        })
        .expect("op");
        assert!(!test_mock_installed());
        PORT.with(|slot| assert!(slot.borrow().is_none()));
    }

    #[test]
    fn mock_stays_installed_across_ops() {
        let mock = Arc::new(InMemoryEventManagerMock::default());
        install_port(mock);
        assert!(test_mock_installed());
        with_port(Path::new("."), "mock_op", |port| {
            let hs = port
                .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                    requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
                })
                .map_err(|e| e.to_string())?;
            assert!(hs.compatible);
            Ok(())
        })
        .expect("mock op");
        clear_port();
    }
}

/// Laboratorio EDA: mutex global + modo store para tests paralelos estables.
#[cfg(test)]
pub mod test_lab {
    use std::sync::{Mutex, MutexGuard, OnceLock};
    use sddia_event_store::{override_store_mode, StoreMode};

    static PARALLEL_LAB: OnceLock<Mutex<()>> = OnceLock::new();

    /// Serializa tests que mutan env lab, `EventManagerPort` o `override_store_mode`.
    pub fn parallel_lab_mutex() -> MutexGuard<'static, ()> {
        PARALLEL_LAB
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    /// RAII: mutex + `clear_port` + restablecer modo store al soltar.
    pub struct ParallelLabGuard {
        _lock: MutexGuard<'static, ()>,
    }

    impl ParallelLabGuard {
        pub fn acquire() -> Self {
            let lock = parallel_lab_mutex();
            super::clear_port();
            Self { _lock: lock }
        }
    }

    impl Drop for ParallelLabGuard {
        fn drop(&mut self) {
            super::clear_port();
            override_store_mode(None);
        }
    }

    pub fn with_parallel_isolated<F: FnOnce()>(f: F) {
        let _g = ParallelLabGuard::acquire();
        f();
    }

    pub struct FilesystemEdaGuard {
        _scope: ParallelLabGuard,
    }

    impl Drop for FilesystemEdaGuard {
        fn drop(&mut self) {
            override_store_mode(None);
        }
    }

    /// Modo FS bajo mutex global (repos temporales / aserciones `.events/`).
    pub fn filesystem_eda() -> FilesystemEdaGuard {
        let scope = ParallelLabGuard::acquire();
        override_store_mode(Some(StoreMode::Filesystem));
        FilesystemEdaGuard { _scope: scope }
    }

    pub struct RedbEdaGuard {
        _scope: ParallelLabGuard,
    }

    impl Drop for RedbEdaGuard {
        fn drop(&mut self) {
            override_store_mode(None);
        }
    }

    /// Modo Redb bajo mutex global (p. ej. REDB-CA*).
    pub fn redb_eda() -> RedbEdaGuard {
        let scope = ParallelLabGuard::acquire();
        override_store_mode(Some(StoreMode::Redb));
        RedbEdaGuard { _scope: scope }
    }

    pub fn fs_lab_tempdir() -> (FilesystemEdaGuard, tempfile::TempDir) {
        (filesystem_eda(), tempfile::tempdir().expect("tempdir"))
    }
}
