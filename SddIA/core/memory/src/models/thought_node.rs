use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtNode {
    pub node_id: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub metadata: serde_json::Value,
    pub friction_trace: Option<String>,
    pub embedding: Option<Vec<f32>>,
}

impl ThoughtNode {
    pub fn new(
        parent_id: Option<String>,
        content: String,
        metadata: serde_json::Value,
        friction_trace: Option<String>,
    ) -> Self {
        // Deterministic hashing based on parent, content, and friction trace (excluding volatile fields like metadata status initially, or including all if rigid).
        let mut hasher = Sha256::new();
        if let Some(pid) = &parent_id {
            hasher.update(pid.as_bytes());
        }
        hasher.update(content.as_bytes());
        if let Some(ft) = &friction_trace {
            hasher.update(ft.as_bytes());
        }
        let result = hasher.finalize();
        let node_id = hex::encode(result);

        Self {
            node_id,
            parent_id,
            content,
            metadata,
            friction_trace,
            embedding: None,
        }
    }
}
