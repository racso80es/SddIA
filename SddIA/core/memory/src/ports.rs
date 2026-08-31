use crate::error::MemoryStoreError;
use crate::models::evolution_node::EvolutionEvent;
use crate::models::thought_node::ThoughtNode;

pub trait ThoughtGraphRepository {
    type Error;

    fn store_thought(&self, thought: ThoughtNode) -> Result<(), Self::Error>;

    fn get_thought_by_id(&self, node_id: &str) -> Result<Option<ThoughtNode>, Self::Error>;

    fn get_children(&self, parent_id: &str) -> Result<Vec<ThoughtNode>, Self::Error>;

    fn search_similar_thoughts(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<ThoughtNode>, Self::Error>;
}

pub trait EvolutionStore {
    type Error;

    fn store_event(&self, event: EvolutionEvent) -> Result<(), Self::Error>;

    fn get_event_by_id(&self, id: &str) -> Result<Option<EvolutionEvent>, Self::Error>;

    fn search_similar_events(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<EvolutionEvent>, Self::Error>;
}

pub type DefaultStoreError = MemoryStoreError;
