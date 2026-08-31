pub mod error;
pub mod models;
pub mod ports;
pub mod services;

use serde::{Deserialize, Serialize};

pub const EMBEDDING_DIM: usize = 384;

pub use error::MemoryStoreError;
pub use services::inference_binding::{EMBEDDING_MODEL, EMBEDDING_NORM, LocalHashingEmbedder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: String,
    pub original_source: String,
    pub text_content: String,
    pub metadata: serde_json::Value,
    pub embedding: Option<Vec<f32>>,
}

pub trait EmbeddingGenerator {
    type Error;
    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Self::Error>;
}

pub trait VectorStore {
    type Error;
    fn store_chunk(&self, chunk: KnowledgeChunk) -> Result<(), Self::Error>;
    fn search_similar(
        &self,
        query_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<KnowledgeChunk>, Self::Error>;
}
