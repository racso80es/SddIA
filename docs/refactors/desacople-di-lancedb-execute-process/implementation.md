---
feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
items:
  - "SddIA/tools/lancedb-memory-adapter/Cargo.toml"
  - "SddIA/tools/lancedb-memory-adapter/src/main.rs"
  - "SddIA/tools/lancedb-memory-adapter.md"
  - "SddIA/engine/execute-process/Cargo.toml"
  - "SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs"
  - "SddIA/engine/execute-process/src/engine/handlers/user_preference.rs"
  - "SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs"
  - "SddIA/engine/execute-process/src/engine/eda_port.rs"
  - "SddIA/engine/execute-process/tests/memory_evolution_ingest.rs"
  - "SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml"
  - "SddIA/infrastructure/adapters/lancedb_thought_repo/src/lib.rs"
  - "SddIA/infrastructure/adapters/lancedb_preferences_repo/src/lib.rs"
  - "SddIA/core/cumulo.paths.json"
  - "SddIA/core/capability-bindings.md"
  - "SddIA/core/eda-coverage.json"
  - "SddIA/library/norms/capability-taxonomy.md"
  - "SddIA/tools/index.md"
  - "SddIA/Cargo.toml"
  - ".github/workflows/sddia-index-qa.yml"
  - "SddIA/evolution/bab6e051-04bd-4d83-b73e-267189430e37.md"
---
# Implementación — desacople-di-lancedb-execute-process (`AUT-119`)

1. **Satélite Nativo Unificado (`tool:lancedb-memory-adapter`):**
   - Creado `SddIA/tools/lancedb-memory-adapter/` (`Cargo.toml`, `src/main.rs`) sobre `sddia-io` (`read_stdin_json` fail-fast TTY, `emit_v2`), unificando `evolution.*`, `preferences.*` y `thought.*`, encapsulando `LocalHashingEmbedder` y añadiendo `with_lancedb_retry` ante conflictos transitorios de commit/I/O.
2. **Desacople de `execute-process`:**
   - Eliminadas `sddia-infrastructure-lancedb-evolution` y `sddia-infrastructure-lancedb-preferences` de `SddIA/engine/execute-process/Cargo.toml`.
   - `memory_evolution_ingest_core.rs` refactorizado al puerto `EvolutionIngestPort` (`CapsuleEvolutionIngestPort` → `lancedb-memory-adapter`).
   - `handlers/user_preference.rs` refactorizado a `invoke_tool_capsule_json("lancedb-memory-adapter")` con fast-check `lancedb_uri(repo).join("user_preferences.lance").is_dir()`.
   - `handlers/aiua_stimulus.rs` migrado de `thought-graph-access` a `lancedb-memory-adapter` (`thought.search` / `thought.store`) y emisión directa de `Thought_Persisted` vía `eda_port::with_productive_port`.
   - `tests/memory_evolution_ingest.rs` migrado a `MockEvolutionIngestPort` sin dependencia de LanceDB.
3. **Purga Redb en `lancedb_thought_repo`:**
   - Eliminados `sddia-event-manager-host`, `sddia-core`, `sddia-eda-catalog`, `uuid`, `chrono` de `lancedb_thought_repo/Cargo.toml` y `emit_thought_persisted` de `src/lib.rs`.
   - Podado `SddIA/tools/thought-graph-access/` de `allowlists/eda-unit-backend.toml` y `gate_eda_unit_no_real_backend.rs`.
4. **Retirada de `thought-graph-access` y SSOT:**
   - Eliminados `SddIA/tools/thought-graph-access/` y `SddIA/tools/thought-graph-access.md`.
   - Añadido `paths.lancedbStore` en `cumulo.paths.json`, capacidades `memory:evolution-store`, `memory:pref-vector`, `memory:thought-graph` en `capability-taxonomy.md` y `capability-bindings.md`, y actualizado `eda-coverage.json` + `tools/index.md`.
5. **Workspace y CI:**
   - Añadido `default-members` a `SddIA/Cargo.toml` excluyendo crates LanceDB.
   - Retirado `protoc` de `wasi-build` y `EventManagerPort` en `.github/workflows/sddia-index-qa.yml`, e incluidos `lancedb_preferences` y `lancedb-memory-adapter` en `lance_memory` tests y `classify-sddia-index-qa-changes.sh`.
6. **Evolution (DA-2):**
   - Registrado `SddIA/evolution/bab6e051-04bd-4d83-b73e-267189430e37.md` en `Evolution_log.md`.
