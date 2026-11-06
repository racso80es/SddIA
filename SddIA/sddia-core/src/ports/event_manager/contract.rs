//! Tipos tipados alineados a `eda.event_manager.v1.schema.json`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Versión de contrato que implementan los adapters built-in / mock.
pub const CONTRACT_VERSION: &str = "1.0.0";

/// Rango SemVer canónico exigido por el host (`^1.0.0`).
pub const REQUIRED_CONTRACT_RANGE: &str = "^1.0.0";

/// Identificador de contrato Genoma.
pub const CONTRACT_ID: &str = "eda.event_manager";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublishEcstRequest {
    pub event_id: String,
    pub ecst: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublishEcstResult {
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimPendingRequest {
    pub lease_owner: String,
    pub limit: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_ttl_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimedEvent {
    pub event_id: String,
    pub ecst: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub lease_owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimPendingResult {
    pub events: Vec<ClaimedEvent>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Ack,
    Nack,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecordDeliveryRequest {
    pub event_id: String,
    pub delivery_id: String,
    pub status: DeliveryStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MoveToDlqRequest {
    pub event_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayDlqRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppendTraceRequest {
    pub trace_id: String,
    pub frame: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandshakeVersionRequest {
    pub requested_range: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandshakeVersionResult {
    pub contract: String,
    pub contract_version: String,
    pub compatible: bool,
}

/// Manifiesto dual-version de un implementador.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DualVersionManifest {
    pub capsule_version: String,
    pub implements_contract: ImplementsContract,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImplementsContract {
    pub contract: String,
    pub contract_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_uri: Option<String>,
}

impl DualVersionManifest {
    pub fn builtin_v1(capsule_version: &str) -> Self {
        Self {
            capsule_version: capsule_version.to_string(),
            implements_contract: ImplementsContract {
                contract: CONTRACT_ID.to_string(),
                contract_version: CONTRACT_VERSION.to_string(),
                schema_uri: Some(
                    "https://sddia.local/capability-contracts/eda.event_manager.v1.schema.json"
                        .to_string(),
                ),
            },
        }
    }
}
