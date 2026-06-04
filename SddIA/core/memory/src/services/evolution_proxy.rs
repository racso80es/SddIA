use crate::models::evolution_node::{EvolutionEvent, SpatialPolarity};
use crate::ports::EvolutionStore;

pub struct EvolutionProxyService<S: EvolutionStore> {
    store: S,
}

impl<S: EvolutionStore> EvolutionProxyService<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }

    pub fn capture_event(&self, payload: &str, metadata: serde_json::Value) -> Result<String, S::Error> {
        // Triage the polarity based on metadata attributes
        // The default assumption is structural fracture unless success flag is present
        let mut polarity = SpatialPolarity::StructuralFracture;

        if let Some(success) = metadata.get("success").and_then(|v| v.as_bool()) {
            if success {
                polarity = SpatialPolarity::EfficientSymmetry;
            }
        }

        let event = EvolutionEvent::new(polarity, payload, metadata);
        let id = event.id.clone();

        self.store.store_event(event)?;

        Ok(id)
    }
}
