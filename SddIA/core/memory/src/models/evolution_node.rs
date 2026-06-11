use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpatialPolarity {
    EfficientSymmetry,
    StructuralFracture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    pub id: String, // SHA-256 hash
    pub polarity: SpatialPolarity,
    pub payload: String,
    pub operational_metadata: serde_json::Value,
    pub embedding: Option<Vec<f32>>,
}

impl EvolutionEvent {
    pub fn new(polarity: SpatialPolarity, payload: &str, metadata: serde_json::Value) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let result = hasher.finalize();
        let id = hex::encode(result);

        Self {
            id,
            polarity,
            payload: payload.to_string(),
            operational_metadata: metadata,
            embedding: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_evolution_event_sha256_id() {
        let metadata = json!({"success": true});
        let payload = "System started securely";
        let event = EvolutionEvent::new(SpatialPolarity::EfficientSymmetry, payload, metadata);

        // id debe ser el sha256 hex del payload
        let mut hasher = Sha256::new();
        hasher.update(payload.as_bytes());
        let expected_id = hex::encode(hasher.finalize());

        assert_eq!(event.id, expected_id);
        assert_eq!(event.polarity, SpatialPolarity::EfficientSymmetry);
    }
}
