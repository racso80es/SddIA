use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: Uuid,
    pub content: String,
    pub source_id: String,
    pub embedding: Option<Vec<f32>>,
}

pub trait EmbeddingGenerator {
    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub trait VectorStore {
    fn store_chunk(&self, chunk: KnowledgeChunk) -> Result<(), String>;
    fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<KnowledgeChunk>, String>;
}
