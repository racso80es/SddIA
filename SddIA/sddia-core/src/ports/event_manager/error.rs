//! Errores tipados del puerto EventManager.

use thiserror::Error;

/// Semilla de fallo DI por MAJOR mismatch (Genoma / Cerbero).
pub const F_DI_CONTRACT_VERSION_MISMATCH: &str = "F-DI-CONTRACT-VERSION-MISMATCH";

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EventManagerError {
    #[error("contract version mismatch: {message} ({seed})")]
    ContractVersionMismatch {
        message: String,
        seed: String,
    },

    #[error("event not found: {0}")]
    NotFound(String),

    #[error("lease conflict for event {event_id}: {detail}")]
    LeaseConflict { event_id: String, detail: String },

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("external capsule error: {0}")]
    ExternalCapsule(String),

    #[error("injected fault: {0}")]
    InjectedFault(String),
}

impl EventManagerError {
    pub fn version_mismatch(message: impl Into<String>) -> Self {
        Self::ContractVersionMismatch {
            message: message.into(),
            seed: F_DI_CONTRACT_VERSION_MISMATCH.to_string(),
        }
    }
}
