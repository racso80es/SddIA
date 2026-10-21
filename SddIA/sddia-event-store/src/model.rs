use serde::{Deserialize, Serialize};

/// Familia ECST. El store no lee suscripciones ni elige destino.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Family {
    Telemetry,
    Orchestration,
    Domain,
}

impl Family {
    pub fn as_str(self) -> &'static str {
        match self {
            Family::Telemetry => "telemetry",
            Family::Orchestration => "orchestration",
            Family::Domain => "domain",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeliveryStatus {
    Pending,
    Success,
    Failed,
    Skipped,
}

/// Fila de `events`. `created_at` es unix millis y ordena `family_index`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredEvent {
    pub event_id: String,
    pub family: Family,
    pub event_type: String,
    pub created_at: i64,
    pub ecst: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryWitness {
    pub status: DeliveryStatus,
    pub witness: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeadLetter {
    pub cause: String,
    pub failed_subscriber_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DltSeal {
    pub tx_hash: String,
    pub digest: String,
    pub scope: String,
}
