//! Composition root productivo: rol `default` → `adapter:redb-event-adapter`.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sddia_core::ports::event_manager::{
    BindingRole, EventManagerDiFactory, EventManagerPort,
};
use sddia_infrastructure_redb_event::RedbEventAdapter;

const DEFAULT_ROLE: &str = "default";
const REDB_PROVIDER: &str = "adapter:redb-event-adapter";

/// Ruta canónica del store de eventos bajo la raíz del repositorio.
pub fn events_redb_path(repo_root: &Path) -> PathBuf {
    repo_root.join(".SddIA/runtime/events.redb")
}

fn open_budget_ms() -> u64 {
    std::env::var("SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(2000)
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

/// Factory con rol productivo `default` → Redb y builder registrado.
pub fn productive_factory(repo_root: &Path) -> Result<EventManagerDiFactory, String> {
    let path = events_redb_path(repo_root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let path_clone = path.clone();
    let mut factory = EventManagerDiFactory::new();
    factory.register_role(
        DEFAULT_ROLE,
        BindingRole {
            provider: REDB_PROVIDER.into(),
            provider_version: ">=1.2.0".into(),
            implements_contract_version: "1.2.0".into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn default_resolves_redb_on_tempdir() {
        let dir = tempdir().expect("tempdir");
        let port = resolve_productive_port(dir.path()).expect("resolve");
        let hs = port
            .handshake_version(sddia_core::ports::event_manager::HandshakeVersionRequest {
                requested_range: sddia_core::ports::event_manager::REQUIRED_CONTRACT_RANGE.into(),
            })
            .expect("handshake");
        assert!(hs.compatible);
    }
}
