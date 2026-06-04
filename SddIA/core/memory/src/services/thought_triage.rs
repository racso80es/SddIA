use crate::models::thought_node::ThoughtNode;
use crate::ports::ThoughtGraphRepository;
use crate::EmbeddingGenerator;

pub struct ThoughtTriageService<R: ThoughtGraphRepository, E: EmbeddingGenerator> {
    pub repository: R,
    pub embedding_generator: E,
    pub similarity_threshold: f32,
}

impl<R: ThoughtGraphRepository, E: EmbeddingGenerator> ThoughtTriageService<R, E> {
    pub fn new(repository: R, embedding_generator: E, similarity_threshold: f32) -> Self {
        Self {
            repository,
            embedding_generator,
            similarity_threshold,
        }
    }

    /// Triage predicts if a newly spawned thought is structurally sound or collides with a known error
    pub fn evaluate_and_spawn(
        &self,
        parent_id: Option<String>,
        content: String,
    ) -> Result<ThoughtNode, String> {
        // Generate embedding
        let embedding = self.embedding_generator
            .generate_embedding(&content)
            .map_err(|_| "Failed to generate embedding".to_string())?;

        // Predictive Vector Triage (Paso 0)
        let similar_thoughts = self.repository
            .search_similar_thoughts(&embedding, 5)
            .map_err(|_| "Vector search failed".to_string())?;

        // If collision with previous PRUNED thoughts, reject early.
        for prior in similar_thoughts {
            if prior.metadata.get("status").and_then(|s| s.as_str()) == Some("PRUNED") {
                // Here we would perform a cosine similarity check locally
                // If it's over `self.similarity_threshold`, it's an immediate fail
                return Err("Thought collides with known failure (Secuestro Semantico). Pruned.".into());
            }
        }

        // Emit domain events internally silently, decoupled from biological vertex
        let mut node = ThoughtNode::new(parent_id, content, serde_json::json!({"status": "ACTIVE"}), None);
        node.embedding = Some(embedding);

        self.repository.store_thought(node.clone()).map_err(|_| "Storage failed".to_string())?;

        Ok(node)
    }

    /// Autopoiesis logic: If a node fails Filter A/B during deeper execution, it is pruned, saving its trace as an antibody
    pub fn prune_thought(
        &self,
        mut node: ThoughtNode,
        friction_trace: String,
    ) -> Result<Option<String>, String> {
        node.metadata = serde_json::json!({"status": "PRUNED"});
        node.friction_trace = Some(friction_trace);

        self.repository.store_thought(node.clone()).map_err(|_| "Failed to store pruned antibody".to_string())?;

        // Return parent_id to autonomously retrocede
        Ok(node.parent_id)
    }
}
