---
feature_name: lancedb-real-vector-memory
created: "2026-08-31"
process: feature
items:
  - T0-workspace-lancedb-dep
  - T1-core-ports-embedder
  - T2-thought-adapter
  - T3-evolution-adapter-import
  - T4-ssot-ingest-wiring
  - T5-tests-ci
  - T6-adapter-cards-evolution
branch_name: feat/lancedb-real-vector-memory
persist_ref: docs/features/lancedb-real-vector-memory
document_id: PBI-CORE-LANCEDB-REAL-001
uuid: "3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46"
---

# Implementación — lancedb-real-vector-memory

## Touchpoints

| Artefacto | Ruta |
|-----------|------|
| Workspace members | `SddIA/Cargo.toml` |
| Core memory | `SddIA/core/memory/` (`MemoryStoreError`, `LocalHashingEmbedder`, puertos read) |
| Thought adapter | `SddIA/infrastructure/adapters/lancedb_thought_repo/` |
| Evolution adapter | `SddIA/infrastructure/adapters/lancedb_evolution_repo/` |
| Ingest | `SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs` |
| SSOT | `SddIA/core/cumulo.paths.json` v1.9.0 `paths.vectorStore` |
| CI | `.github/workflows/sddia-index-qa.yml` (`protobuf-compiler` + tests) |
| Fichas | `lancedb-*-repo.md` v1.1.0 `active` |
| Evolution | `SddIA/evolution/4d384bb1-f89d-41ce-835a-9db6d6bed114.md` |

## Contratos

- Driver: `lancedb = "=0.37.1"` (0.38.0: `job.rs` referencia `Error::Http` inexistente).
- URI: `{vectorStore}/lancedb/` — aislado de JSON legado y `user_preferences`.
- Async: `OnceLock<Runtime>` por crate adaptador; puertos sync.
- WASI: no.

## Mutaciones

- Ingest deja de escribir `{id}.json`.
- `LocalSemanticInference` = alias de hashing local (no ceros).
