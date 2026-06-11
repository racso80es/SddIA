pub mod models;
pub mod ports;
pub mod services;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: String,                  // Hash SHA-256 del contenido, NUNCA un Uuid aleatorio.
    pub original_source: String,     // Ruta de origen del activo.
    pub text_content: String,        // El fragmento de texto puro.
    pub metadata: serde_json::Value, // Etiquetas agnósticas (Capacidad, Estatus).
    pub embedding: Option<Vec<f32>>, // Vector espacial denso.
}

pub trait EmbeddingGenerator {
    type Error;
    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Self::Error>;
}

pub trait VectorStore {
    type Error;
    fn store_chunk(&self, chunk: KnowledgeChunk) -> Result<(), Self::Error>;
    fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<KnowledgeChunk>, Self::Error>;
}
