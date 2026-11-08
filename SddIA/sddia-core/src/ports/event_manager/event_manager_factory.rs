//! Factory DI kernel-safe: rol → binding → version_gate → Arc<dyn EventManagerPort>.
//!
//! **L-FACTORY:** no importa crates redb/fs/external; builders inyectados desde host/tests.

use std::collections::HashMap;
use std::sync::Arc;

use crate::ports::event_manager::contract::REQUIRED_CONTRACT_RANGE;
use crate::ports::event_manager::error::EventManagerError;
use crate::ports::event_manager::memory_mock::InMemoryEventManagerMock;
use crate::ports::event_manager::port::EventManagerPort;
use crate::ports::event_manager::version_gate::version_gate;

/// Constructor inyectable de un puerto concreto.
pub type EventManagerBuilder =
    Box<dyn Fn() -> Result<Arc<dyn EventManagerPort>, EventManagerError> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct BindingRole {
    pub provider: String,
    pub provider_version: String,
    /// Versión de contrato que declara el implementador (`implements_contract`).
    pub implements_contract_version: String,
}

#[derive(Default)]
pub struct EventManagerDiFactory {
    /// provider id → builder (p.ej. `adapter:redb-event-adapter`).
    builders: HashMap<String, EventManagerBuilder>,
    /// role → binding.
    roles: HashMap<String, BindingRole>,
}

impl EventManagerDiFactory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra un constructor built-in / puente (inyectado desde fuera del kernel).
    pub fn register_builder(&mut self, provider: impl Into<String>, builder: EventManagerBuilder) {
        self.builders.insert(provider.into(), builder);
    }

    pub fn register_role(&mut self, role: impl Into<String>, binding: BindingRole) {
        self.roles.insert(role.into(), binding);
    }

    /// Semilla mínima: rol `test` → mock in-kernel.
    pub fn with_test_role_defaults() -> Self {
        let mut f = Self::new();
        f.register_role(
            "test",
            BindingRole {
                provider: "mock:in-memory-event-manager".into(),
                provider_version: ">=1.1.0".into(),
                implements_contract_version: "1.1.0".into(),
            },
        );
        f.register_builder(
            "mock:in-memory-event-manager",
            Box::new(|| Ok(Arc::new(InMemoryEventManagerMock::new()) as Arc<dyn EventManagerPort>)),
        );
        f
    }

    /// Resuelve `role` → binding → `version_gate` → puerto.
    pub fn resolve(&self, role: &str) -> Result<Arc<dyn EventManagerPort>, EventManagerError> {
        let binding = self.roles.get(role).ok_or_else(|| {
            EventManagerError::InvalidRequest(format!("unknown EventManager role '{role}'"))
        })?;

        version_gate(
            REQUIRED_CONTRACT_RANGE,
            &binding.implements_contract_version,
        )?;

        let builder = self.builders.get(&binding.provider).ok_or_else(|| {
            EventManagerError::InvalidRequest(format!(
                "no builder registered for provider '{}'",
                binding.provider
            ))
        })?;

        builder()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_resolves_memory_mock() {
        let f = EventManagerDiFactory::with_test_role_defaults();
        let port = f.resolve("test").expect("resolve test");
        let hs = port
            .handshake_version(crate::ports::event_manager::contract::HandshakeVersionRequest {
                requested_range: "^1.1.0".into(),
            })
            .expect("handshake");
        assert!(hs.compatible);
    }

    #[test]
    fn major_mismatch_on_role_is_rejected() {
        let mut f = EventManagerDiFactory::with_test_role_defaults();
        f.register_role(
            "broken",
            BindingRole {
                provider: "mock:in-memory-event-manager".into(),
                provider_version: ">=2.0.0".into(),
                implements_contract_version: "2.0.0".into(),
            },
        );
        match f.resolve("broken") {
            Err(err) => assert!(matches!(
                err,
                EventManagerError::ContractVersionMismatch { .. }
            )),
            Ok(_) => panic!("expected ContractVersionMismatch"),
        }
    }
}
