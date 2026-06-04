use crate::models::thought_node::ThoughtNode;

pub trait ThoughtGraphRepository {
    type Error;

    /// Stores a thought node in the spatial graph memory
    fn store_thought(&self, thought: ThoughtNode) -> Result<(), Self::Error>;

    /// Retrieves a thought node by its exact deterministic ID
    fn get_thought_by_id(&self, node_id: &str) -> Result<Option<ThoughtNode>, Self::Error>;

    /// Retrieves all immediate children of a given thought node
    fn get_children(&self, parent_id: &str) -> Result<Vec<ThoughtNode>, Self::Error>;

    /// Performs a semantic search to find spatially close thoughts (K-Nearest Neighbors)
    fn search_similar_thoughts(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<ThoughtNode>, Self::Error>;
}

use crate::models::evolution_node::EvolutionEvent;

pub trait EvolutionStore {
    type Error;
    fn store_event(&self, event: EvolutionEvent) -> Result<(), Self::Error>;
}
