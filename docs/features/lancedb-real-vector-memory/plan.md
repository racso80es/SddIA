---
feature_name: lancedb-real-vector-memory
created: "2026-08-31"
process: feature
phase: planning
agents: dedalo
phases:
  - T0-workspace-lancedb-dep
  - T1-core-ports-embedder
  - T2-thought-adapter
  - T3-evolution-adapter-import
  - T4-ssot-ingest-wiring
  - T5-tests-ci
  - T6-adapter-cards-evolution
  - T7-closure
branch_name: feat/lancedb-real-vector-memory
persist_ref: docs/features/lancedb-real-vector-memory
pbi_ref: docs/todos/pending/[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva.md
document_id: PBI-CORE-LANCEDB-REAL-001
uuid: 3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46
runtime_execution_id: "c4e7971d-7c67-4745-b1b4-eb8b3d84d652"
---

# Plan — lancedb-real-vector-memory

Blueprint Tekton. Contratos: `spec.md`. **Stop planning:** este commit sella Diseño. Ejecución T0–T7 **después**.

Init lab: `execution_id` `c4e7971d-7c67-4745-b1b4-eb8b3d84d652` · vehículo `feature` · relevo IDE.

## T0 — Workspace + crate `lancedb`

1. Instalar `protobuf-compiler` en el host de forja (`protoc` en PATH). Sin él, `lance-encoding` no compila (PoC clarify).
2. `SddIA/Cargo.toml` members: `core/memory`, `infrastructure/adapters/lancedb_evolution_repo`, `infrastructure/adapters/lancedb_thought_repo`.
3. Adaptadores: dep `lancedb = "0.38"`, `arrow-array`/`arrow-schema` alineados al crate, `tokio` rt-multi-thread, `futures`.
4. `cargo check -p sddia-infrastructure-lancedb-evolution -p sddia-infrastructure-lancedb-thought` host.
5. Confirmar que `build-wasi-capsules.sh` no los incluye.
6. CI: step `apt-get install protobuf-compiler` **antes** de `cargo build --workspace`.

## T1 — Core

1. `MemoryStoreError` + `EMBEDDING_DIM = 384` + `EMBEDDING_MODEL = "sddia-local-hashing-v1"`.
2. `EvolutionStore`: `get_event_by_id`, `search_similar_events`.
3. Sustituir stub de `LocalSemanticInference`; implementar `LocalHashingEmbedder` (`EmbeddingGenerator` + `SemanticInference`).
4. Test `embedding_is_nonzero_nonconstant_and_repeatable`.
5. `edition` del core: dejar 2024 si el workspace ya lo compila; no mezclar en adapters (2021).

## T2 — Thought adapter

Implementar los 4 métodos sobre tabla `thought_graph_collection`. `open(path)`. Schema + merge_insert + query exacta (`where node_id =`) + `parent_id` + KNN. Tests: roundtrip reopen, children, KNN, dim rechazada.

## T3 — Evolution adapter + importador

Igual para tabla `evolution`. `import_legacy_evolution_json`. Tests: roundtrip, idempotencia (conteo filas = 1), schema mismatch (abrir store con dim 8 y reabrir esperando 384).

## T4 — SSOT + ingest

1. `cumulo.paths.json` v1.9.0: `"vectorStore": ".SddIA/vector_store/"`.
2. Helper en `execute-process` (o adapter `open` recibido PathBuf): resolver clave; URI `lancedb/` bajo esa raíz.
3. Reescribir `ingest_inner`: puerto real; sin JSON. Ajustar test existente + E2E reopen + `json_fallback_is_not_used`.
4. `execute-process/Cargo.toml` path-dep a evolution adapter + core memory.

## T5 — CI

Step `protobuf-compiler` + tests (spec §6). Verificar que `cargo build --workspace` nativo sigue verde (Arrow + prost aumentan tiempo; cache Actions).

## T6 — Fichas + cicatriz

1. `lancedb-*-repo.md` v1.1.0 `status: active` + `index.md`.
2. Evolution `{uuid}.md` 1.1.2 + `sddia-qa evolution-rehash --id`. `gate-evolution --json --range` antes de push.

## T7 — Cierre

`implementation.md` + `execution.md` → Argos `validacion.md` (`global: APTO`, `pbi_archived: true`) → PBI a `docs/todos/done/` → `delivery-close-cycle`. Un PR. Tests verdes locales **antes** de DCC.

## Orden

```text
T0 → T1 → T2
     T1 → T3
T2 + T3 → T4 → T5 → T6 → T7
```

## Verificación por fase

```text
cd SddIA && cargo check -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution
cd SddIA && cargo test -p sddia-core-memory
cd SddIA && cargo test -p sddia-infrastructure-lancedb-thought
cd SddIA && cargo test -p sddia-infrastructure-lancedb-evolution
cd SddIA && cargo test -p execute-process -- memory_evolution_ingest
```

## Prohibido en ejecución

- JSON fallback silencioso.
- Tokio por operación.
- WASI del driver.
- Tocar `.SddIA/vector_store/` del operador.
- `entity-manager` sobre adapters.
- Empujar `validacion.md` con check CI rojo conocido (DA-6).
- `sleep`/poll post-acuse CLI (DA-5).
