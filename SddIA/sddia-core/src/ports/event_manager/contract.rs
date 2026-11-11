//! Tipos tipados alineados a `eda.event_manager` **2.0.0** (MAN-43).

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Versión de contrato que implementan los adapters built-in / mock.
pub const CONTRACT_VERSION: &str = "2.0.0";

/// Rango SemVer canónico exigido por el host (`^2.0.0`).
pub const REQUIRED_CONTRACT_RANGE: &str = "^2.0.0";

/// Identificador de contrato Genoma.
pub const CONTRACT_ID: &str = "eda.event_manager";

/// Familias URI canónicas (L-1). Nunca estados de cola.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EventFamily {
    Domain,
    Telemetry,
    Orchestration,
}

impl EventFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Domain => "domain",
            Self::Telemetry => "telemetry",
            Self::Orchestration => "orchestration",
        }
    }

    /// Parseo estricto de familia URI (rechaza estados de cola).
    pub fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim() {
            "domain" => Ok(Self::Domain),
            "telemetry" => Ok(Self::Telemetry),
            "orchestration" => Ok(Self::Orchestration),
            "pending" | "claimed" | "processed" | "dead_letter" | "dead-letter" | "progress" => {
                Err(format!(
                    "'{raw}' es estado de cola, no familia URI (L-1)"
                ))
            }
            other if other.is_empty() => Err("event_family vacío".into()),
            other => Err(format!("event_family inválida: {other}")),
        }
    }

    /// Mapea token histórico de publicación FS (`.events/{token}/…`) a familia URI.
    /// `pending`/`claimed`/… → `Domain` (estado queda detrás del puerto).
    pub fn from_publish_token(token: &str) -> Result<Self, String> {
        match token.trim().trim_start_matches("./") {
            "domain" | "pending" | "claimed" | "processed" | "dead_letter" | "dead-letter" => {
                Ok(Self::Domain)
            }
            "telemetry" => Ok(Self::Telemetry),
            "orchestration" | "progress" => Ok(Self::Orchestration),
            other if other.is_empty() => Err("publish family token vacío".into()),
            other => Err(format!("publish family token desconocido: {other}")),
        }
    }
}

impl fmt::Display for EventFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EventFamily {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Estados internos de cola (nunca familias URI).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum EventQueueState {
    Pending,
    Claimed,
    Processed,
    DeadLetter,
}

impl EventQueueState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Claimed => "claimed",
            Self::Processed => "processed",
            Self::DeadLetter => "dead_letter",
        }
    }
}

/// Identidad canónica `eda://{family}/{event_id}` (L-1 / L-2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EventUri {
    pub family: EventFamily,
    pub event_id: String,
}

impl EventUri {
    pub fn new(family: EventFamily, event_id: impl Into<String>) -> Result<Self, String> {
        let event_id = event_id.into();
        let trimmed = event_id.trim();
        if trimmed.is_empty() {
            return Err("event_id vacío".into());
        }
        if trimmed.contains('/') || trimmed.contains('\\') {
            return Err("event_id no puede contener separadores de ruta".into());
        }
        Ok(Self {
            family,
            event_id: trimmed.to_string(),
        })
    }

    pub fn parse(raw: &str) -> Result<Self, String> {
        let raw = raw.trim();
        let rest = raw
            .strip_prefix("eda://")
            .ok_or_else(|| format!("EventUri debe empezar por eda:// (got '{raw}')"))?;
        let (fam, id) = rest
            .split_once('/')
            .ok_or_else(|| format!("EventUri sin event_id: '{raw}'"))?;
        let family = EventFamily::parse(fam)?;
        Self::new(family, id)
    }

    pub fn validate(&self) -> Result<(), String> {
        Self::new(self.family, self.event_id.as_str()).map(|_| ())
    }

    pub fn as_canonical(&self) -> String {
        format!("eda://{}/{}", self.family.as_str(), self.event_id)
    }
}

impl fmt::Display for EventUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_canonical())
    }
}

impl FromStr for EventUri {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetEventRequest {
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetEventResult {
    pub event_id: String,
    pub ecst: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListPendingRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListedPendingEvent {
    pub event_id: String,
    pub ecst: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListPendingResult {
    pub events: Vec<ListedPendingEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListDlqRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListedDlqEvent {
    pub event_id: String,
    pub reason: String,
    pub ecst: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListDlqResult {
    pub events: Vec<ListedDlqEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListTracesRequest {
    pub trace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListTracesResult {
    pub frames: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PurgeEventRequest {
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListEventsRequest {
    /// Familia URI como string (`domain`|`telemetry`|`orchestration`) o vacío.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListedEvent {
    pub event_id: String,
    pub ecst: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListEventsResult {
    pub events: Vec<ListedEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListDeliveriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeliveryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListedDelivery {
    pub delivery_id: String,
    pub event_id: String,
    pub status: DeliveryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListDeliveriesResult {
    pub deliveries: Vec<ListedDelivery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Barrido de terminales (sustituye sweeper FS — L-6).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListTerminalEventsRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

pub type ListTerminalEventsResult = ListEventsResult;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SealDltRequest {
    pub event_id: String,
    pub tx_hash: String,
    pub digest: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetDltSealRequest {
    pub event_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetDltSealResult {
    pub event_id: String,
    pub tx_hash: String,
    pub digest: String,
    pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListProgressRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListProgressResult {
    pub frames: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VerifyInlineEcstRequest {
    pub event_id: String,
    pub ecst: Value,
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
    pub fn builtin_v2(capsule_version: &str) -> Self {
        Self {
            capsule_version: capsule_version.to_string(),
            implements_contract: ImplementsContract {
                contract: CONTRACT_ID.to_string(),
                contract_version: CONTRACT_VERSION.to_string(),
                schema_uri: Some(
                    "https://sddia.local/capability-contracts/eda.event_manager.v2.schema.json"
                        .to_string(),
                ),
            },
        }
    }

    /// Alias histórico; apunta al manifiesto v2 (sin dualidad productiva).
    pub fn builtin_v1(capsule_version: &str) -> Self {
        Self::builtin_v2(capsule_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_uri_roundtrip() {
        let uri = EventUri::new(EventFamily::Domain, "abc-1").unwrap();
        assert_eq!(uri.as_canonical(), "eda://domain/abc-1");
        assert_eq!(EventUri::parse("eda://domain/abc-1").unwrap(), uri);
    }

    #[test]
    fn rejects_queue_state_as_family() {
        assert!(EventFamily::parse("pending").is_err());
        assert!(EventUri::parse("eda://pending/x").is_err());
    }

    #[test]
    fn publish_token_maps_pending_to_domain() {
        assert_eq!(
            EventFamily::from_publish_token("pending").unwrap(),
            EventFamily::Domain
        );
    }
}
