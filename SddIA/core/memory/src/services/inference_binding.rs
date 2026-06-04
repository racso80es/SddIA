use crate::models::evolution_node::EvolutionEvent;

pub trait SemanticInference {
    type Error;
    fn embed_event(&self, event: &mut EvolutionEvent) -> Result<(), Self::Error>;
}

// Local mock representing the All-MiniLM-L6-v2 model inference binding.
pub struct LocalSemanticInference;

impl SemanticInference for LocalSemanticInference {
    type Error = String;

    fn embed_event(&self, event: &mut EvolutionEvent) -> Result<(), Self::Error> {
        // This is a stub for local inference integration running under wasm32-wasip1.
        // E.g., via ONNX Runtime or candle framework using `all-MiniLM-L6-v2`.
        event.embedding = Some(vec![0.0; 384]);
        Ok(())
    }
}
