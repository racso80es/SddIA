use crate::error::MemoryStoreError;
use crate::models::evolution_node::EvolutionEvent;
use crate::{EMBEDDING_DIM, EmbeddingGenerator};

pub const EMBEDDING_MODEL: &str = "sddia-local-hashing-v1";
pub const EMBEDDING_NORM: &str = "l2";

const FNV_OFFSET: u32 = 2_166_136_261;
const FNV_PRIME: u32 = 16_777_619;

pub trait SemanticInference {
    type Error;
    fn embed_event(&self, event: &mut EvolutionEvent) -> Result<(), Self::Error>;
}

/// Embeddings locales deterministas (hashing de n-gramas). Sin red. Sin vectores cero.
pub struct LocalHashingEmbedder;

/// Alias de compatibilidad: el stub MiniLM queda sustituido por hashing local.
pub type LocalSemanticInference = LocalHashingEmbedder;

fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut h = FNV_OFFSET;
    for b in bytes {
        h ^= u32::from(*b);
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

fn hashing_embed(text: &str) -> Result<Vec<f32>, MemoryStoreError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(MemoryStoreError::EmbeddingFailed {
            reason: "empty input".into(),
        });
    }
    let lowered = trimmed.to_lowercase();
    let padded = format!("^{lowered}$");
    let bytes = padded.as_bytes();
    let mut v = vec![0.0f32; EMBEDDING_DIM];
    if bytes.len() < 3 {
        let h = fnv1a32(bytes);
        let idx = (h as usize) % EMBEDDING_DIM;
        let sign = if (h >> 31) & 1 == 0 { 1.0 } else { -1.0 };
        v[idx] = sign;
    } else {
        for window in bytes.windows(3) {
            let h = fnv1a32(window);
            let idx = (h as usize) % EMBEDDING_DIM;
            let sign = if (h >> 31) & 1 == 0 { 1.0 } else { -1.0 };
            v[idx] += sign;
        }
    }
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm == 0.0 || !norm.is_finite() {
        return Err(MemoryStoreError::EmbeddingFailed {
            reason: "zero-norm vector".into(),
        });
    }
    for x in &mut v {
        *x /= norm;
    }
    Ok(v)
}

impl EmbeddingGenerator for LocalHashingEmbedder {
    type Error = MemoryStoreError;

    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Self::Error> {
        hashing_embed(text)
    }
}

impl SemanticInference for LocalHashingEmbedder {
    type Error = MemoryStoreError;

    fn embed_event(&self, event: &mut EvolutionEvent) -> Result<(), Self::Error> {
        event.embedding = Some(self.generate_embedding(&event.payload)?);
        Ok(())
    }
}

pub fn validate_embedding_dim(embedding: &[f32]) -> Result<(), MemoryStoreError> {
    if embedding.len() != EMBEDDING_DIM {
        return Err(MemoryStoreError::DimensionMismatch {
            expected: EMBEDDING_DIM,
            actual: embedding.len(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedding_is_nonzero_nonconstant_and_repeatable() {
        let embedder = LocalHashingEmbedder;
        let a = embedder.generate_embedding("alpha-memory-one").unwrap();
        let a2 = embedder.generate_embedding("alpha-memory-one").unwrap();
        let b = embedder.generate_embedding("beta-memory-two").unwrap();
        assert_eq!(a.len(), EMBEDDING_DIM);
        assert_eq!(a, a2);
        assert_ne!(a, b);
        assert!(a.iter().any(|x| *x != 0.0));
        assert!(b.iter().any(|x| *x != 0.0));
        let norm: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn empty_input_is_rejected() {
        let err = LocalHashingEmbedder
            .generate_embedding("   ")
            .unwrap_err();
        assert!(matches!(err, MemoryStoreError::EmbeddingFailed { .. }));
    }
}
