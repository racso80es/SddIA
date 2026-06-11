---
feature_name: memoria-vectorial
created: "2026-06-11"
process: feature
branch_name: feat/memoria-vectorial
persist_ref: docs/features/memoria-vectorial
---

# Plan de Ejecución — memoria-vectorial

## Fases

1. **Core domain** — Definir `KnowledgeChunk`, `EmbeddingGenerator`, `VectorStore` en `SddIA/core/memory/src/lib.rs`.
2. **Inferencia nativa** — Binding local All-MiniLM-L6-v2 vía `inference_binding.rs` (WASI).
3. **Adaptadores LanceDB** — Implementar persistencia física en `SddIA/infrastructure/adapters/`.
4. **EDA** — Emitir `Vector_Memory_Indexed` tras cada `store_chunk` / `store_event` exitoso.
5. **Validación** — Tests unitarios SHA-256, búsqueda KNN mock, compilación `wasm32-wasip1`.

## Estado en rama coalescente

| Fase | Estado |
|------|--------|
| 1 | ✅ `lib.rs`, modelos, puertos |
| 2 | ✅ `inference_binding.rs` (estructura) |
| 3 | ✅ adaptadores mock LanceDB |
| 4 | ✅ evento + suscripciones |
| 5 | ⏳ integración LanceDB real (deuda Kaizen) |
