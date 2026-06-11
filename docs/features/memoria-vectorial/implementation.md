---
feature_name: memoria-vectorial
created: "2026-06-11"
process: feature
branch_name: feat/memoria-vectorial
persist_ref: docs/features/memoria-vectorial
---

# Implementación — memoria-vectorial

## Touchpoints físicos

| Artefacto | Ruta |
|-----------|------|
| Crate core | `SddIA/core/memory/` |
| Entidad chunk | `src/lib.rs` → `KnowledgeChunk` |
| Generador embeddings | `src/services/inference_binding.rs` |
| Puertos grafo/evolución | `src/ports.rs` |
| Adaptador evolución | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |
| Adaptador grafo | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Ignore vector store | `.gitignore` → `.SddIA/vector_store/` |

## Contratos implementados

```rust
pub trait EmbeddingGenerator {
    fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Self::Error>;
}

pub trait VectorStore {
    fn store_chunk(&self, chunk: KnowledgeChunk) -> Result<(), Self::Error>;
    fn search_similar(&self, query_embedding: &[f32], limit: usize) -> Result<Vec<KnowledgeChunk>, Self::Error>;
}
```

## Mutaciones de estado

- `EvolutionProxyService::capture_event` → persistencia en subíndice evolution.
- `ThoughtTriageService::evaluate_and_spawn` → embedding + persistencia nodo ACTIVE.

## Deuda conocida

Adaptadores LanceDB operan en modo mock (WASI-ready); bindings físicos pendientes de Ola posterior.
