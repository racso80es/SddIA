//! Composition root productivo: rol `default` → `adapter:redb-event-adapter`.
//! Configuración física desde bóveda de instancia (MAN-36).

mod config;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sddia_core::ports::event_manager::{
    BindingRole, EventManagerDiFactory, EventManagerPort,
};
use sddia_infrastructure_redb_event::RedbEventAdapter;

pub use config::{
    resolve_backend_config, resolve_fs_backend_config, resolve_productive_backend_config,
    resolve_var, validate_configured_path, EventManagerBackendConfig, EventManagerConfigError,
    EventManagerProvider, DEFAULT_PROVIDER, DEFAULT_REDB_REL, VAR_FS_ROOT, VAR_PROVIDER,
    VAR_REDB_PATH,
};

const DEFAULT_ROLE: &str = "default";
const REDB_PROVIDER: &str = "adapter:redb-event-adapter";
const FS_PROVIDER: &str = "adapter:fs-event-adapter";

fn open_budget_ms() -> u64 {
    std::env::var("SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(2000)
}

/// Ruta Redb ya resuelta/validada desde bóveda (solo composition root).
pub fn resolved_redb_path(repo_root: &Path) -> Result<PathBuf, String> {
    resolve_productive_backend_config(repo_root)
        .map(|c| c.redb_path)
        .map_err(|e| e.to_string())
}

/// ¿Existe el fichero Redb en la ruta resuelta? (sin exportar el path al caller).
pub fn productive_store_present(repo_root: &Path) -> bool {
    resolved_redb_path(repo_root)
        .map(|p| p.is_file())
        .unwrap_or(false)
}

/// Compara un candidato con la ruta Redb resuelta (detección sin cablear default).
pub fn is_resolved_redb_path(repo_root: &Path, candidate: &Path) -> bool {
    match resolved_redb_path(repo_root) {
        Ok(resolved) => paths_equal_lex(candidate, &resolved),
        Err(_) => false,
    }
}

fn paths_equal_lex(a: &Path, b: &Path) -> bool {
    let na = a.to_string_lossy().replace('\\', "/");
    let nb = b.to_string_lossy().replace('\\', "/");
    na == nb
}

/// Abre `RedbEventAdapter` con reintentos acotados ante lock de base de datos.
pub fn open_redb_adapter(path: &Path) -> Result<Arc<dyn EventManagerPort>, String> {
    let budget = open_budget_ms();
    let start = std::time::Instant::now();
    loop {
        match RedbEventAdapter::open(path) {
            Ok(adapter) => return Ok(Arc::new(adapter)),
            Err(e) => {
                let msg = e.to_string();
                let locked = msg.contains("Database already open")
                    || msg.contains("Cannot acquire lock");
                if locked && start.elapsed().as_millis() < budget as u128 {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                }
                return Err(msg);
            }
        }
    }
}

/// Factory con rol productivo `default` → Redb desde config de bóveda.
pub fn productive_factory(repo_root: &Path) -> Result<EventManagerDiFactory, String> {
    let cfg = resolve_productive_backend_config(repo_root).map_err(|e| e.to_string())?;
    let path = cfg.redb_path;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let path_clone = path.clone();
    let mut factory = EventManagerDiFactory::new();
    factory.register_role(
        DEFAULT_ROLE,
        BindingRole {
            provider: REDB_PROVIDER.into(),
            provider_version: ">=2.0.0".into(),
            implements_contract_version: "2.0.0".into(),
        },
    );
    factory.register_builder(
        REDB_PROVIDER,
        Box::new(move || {
            open_redb_adapter(&path_clone).map_err(|e| {
                sddia_core::ports::event_manager::EventManagerError::Storage(e)
            })
        }),
    );
    Ok(factory)
}

/// Resuelve el puerto productivo (`default`) con `version_gate` ^1.2.0.
pub fn resolve_productive_port(repo_root: &Path) -> Result<Arc<dyn EventManagerPort>, String> {
    let factory = productive_factory(repo_root)?;
    factory
        .resolve(DEFAULT_ROLE)
        .map_err(|e| e.to_string())
}

/// Abre adaptador FS solo para test/migración (raíz desde bóveda).
pub fn resolve_fs_port(repo_root: &Path) -> Result<Arc<dyn EventManagerPort>, String> {
    let root = resolve_fs_backend_config(repo_root).map_err(|e| e.to_string())?;
    if let Some(parent) = root.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let adapter = sddia_infrastructure_fs_event::FsEventAdapter::open(&root)
        .map_err(|e| e.to_string())?;
    let _ = FS_PROVIDER;
    Ok(Arc::new(adapter))
}

fn process_name_for_telemetry() -> String {
    std::env::var("SDDIA_PROCESS_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| std::env::args().nth(0))
        .unwrap_or_else(|| "unknown".into())
}

fn log_open_contention(op_label: &str, wait_ms: u128, budget_ms: u64, outcome: &str, err: &str) {
    eprintln!(
        "[event-manager-host] open_contention process={} operation={} wait_ms={} budget_ms={} outcome={} err={err}",
        process_name_for_telemetry(),
        op_label,
        wait_ms,
        budget_ms,
        outcome,
    );
}

/// Abre puerto productivo, ejecuta `f` y libera el adapter al salir (MAN-38).
pub fn with_productive_port<R>(
    repo_root: &Path,
    op_label: &str,
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String> {
    let start = std::time::Instant::now();
    let budget_ms = open_budget_ms();
    match resolve_productive_port(repo_root) {
        Ok(port) => {
            let wait_ms = start.elapsed().as_millis();
            if wait_ms >= 100 {
                log_open_contention(op_label, wait_ms, budget_ms, "ok", "");
            }
            let out = f(port.as_ref());
            drop(port);
            out
        }
        Err(e) => {
            let wait_ms = start.elapsed().as_millis();
            let locked = e.contains("Database already open") || e.contains("Cannot acquire lock");
            let outcome = if locked { "locked" } else { "error" };
            log_open_contention(op_label, wait_ms, budget_ms, outcome, &e);
            Err(e)
        }
    }
}

/// Rol `test` → `InMemoryEventManagerMock` (laboratorio / unit).
pub fn resolve_test_port() -> Result<Arc<dyn EventManagerPort>, String> {
    EventManagerDiFactory::with_test_role_defaults()
        .resolve("test")
        .map_err(|e| e.to_string())
}

/// Resuelve un rol de binding (`default`, `test`, …).
pub fn resolve_role_port(
    repo_root: &Path,
    role: &str,
) -> Result<Arc<dyn EventManagerPort>, String> {
    if role == "test" {
        return resolve_test_port();
    }
    if role == "migration" || role == "test-fs" {
        return resolve_fs_port(repo_root);
    }
    if role == DEFAULT_ROLE {
        return resolve_productive_port(repo_root);
    }
    let factory = productive_factory(repo_root)?;
    factory.resolve(role).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sddia_core::ports::event_manager::EventManagerError;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_em_env() {
        for k in [
            super::config::VAR_PROVIDER,
            super::config::VAR_REDB_PATH,
            super::config::VAR_FS_ROOT,
        ] {
            std::env::remove_var(k);
        }
    }

    #[test]
    fn default_resolves_redb_on_tempdir() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().expect("tempdir");
        let port = resolve_productive_port(dir.path()).expect("resolve");
        let hs = port
            .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .expect("handshake");
        assert!(hs.compatible);
        assert!(productive_store_present(dir.path()));
    }

    #[test]
    fn vault_relative_redb_path_used() {
        let _g = ENV_LOCK.lock().unwrap();
        clear_em_env();
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".SddIA/.dev")).unwrap();
        std::fs::write(
            root.join(".SddIA/.dev/.env"),
            "SDDIA_EVENT_MANAGER_REDB_PATH=custom/vault.redb\n",
        )
        .unwrap();
        let port = resolve_productive_port(root).expect("resolve");
        let _ = port;
        assert!(root.join("custom/vault.redb").is_file());
    }

    #[test]
    fn test_role_resolves_memory_mock() {
        let port = resolve_test_port().expect("test port");
        let hs = port
            .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .expect("handshake");
        assert!(hs.compatible);
    }

    #[test]
    fn with_productive_port_drops_adapter_after_op() {
        let dir = tempdir().expect("tempdir");
        let opened = with_productive_port(dir.path(), "unit-scoped", |port| {
            let hs = port
                .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                    requested_range:
                        sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
                })
                .map_err(|e| e.to_string())?;
            assert!(hs.compatible);
            Ok(())
        });
        assert!(opened.is_ok());
        let again = with_productive_port(dir.path(), "unit-scoped-2", |port| {
            port.handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .map(|_| ())
            .map_err(|e| e.to_string())
        });
        assert!(again.is_ok());
    }

    #[test]
    fn rejects_contract_below_required_range() {
        let dir = tempdir().expect("tempdir");
        let mut factory = productive_factory(dir.path()).expect("factory");
        factory.register_role(
            "legacy",
            BindingRole {
                provider: REDB_PROVIDER.into(),
                provider_version: ">=1.0.0".into(),
                implements_contract_version: "1.0.0".into(),
            },
        );
        match factory.resolve("legacy") {
            Err(EventManagerError::ContractVersionMismatch { .. }) => {}
            Err(_) => panic!("expected ContractVersionMismatch"),
            Ok(_) => panic!("expected version gate rejection"),
        }
    }

    #[test]
    fn fs_role_opens_configured_root() {
        let dir = tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".SddIA/.dev")).unwrap();
        std::fs::write(
            root.join(".SddIA/.dev/.env"),
            "SDDIA_EVENT_MANAGER_FS_ROOT=fs-lab\n",
        )
        .unwrap();
        let port = resolve_fs_port(root).expect("fs port");
        let hs = port
            .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .expect("handshake");
        assert!(hs.compatible);
        assert!(root.join("fs-lab").is_dir());
    }
}
