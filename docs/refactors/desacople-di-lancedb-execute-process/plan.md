---
feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
phases:
  - "T1: Purgar sddia-event-manager-host de lancedb_thought_repo y podar allowlists/eda-unit-backend.toml"
  - "T2: Crear satélite nativo unificado SddIA/tools/lancedb-memory-adapter con sddia-io, retry y tests"
  - "T3: Desacoplar execute-process (Cargo.toml, memory_evolution_ingest_core, user_preference, aiua_stimulus, tests)"
  - "T4: Retirar thought-graph-access y actualizar SSOT (cumulo.paths.json, capability-taxonomy, capability-bindings, tools/index.md)"
  - "T5: Configurar default-members en SddIA/Cargo.toml, actualizar scripts y CI (.github/workflows/sddia-index-qa.yml)"
  - "T6: Registrar en SddIA/evolution/, ejecutar suites de test, gates sddia-qa y sellar validacion.md APTO"
---
# Plan — desacople-di-lancedb-execute-process (`AUT-119`)

1. **T1 (Purga Redb en `lancedb_thought_repo`):** Eliminar `sddia-event-manager-host`, `sddia-core`, `sddia-eda-catalog`, `uuid`, `chrono` de `lancedb_thought_repo/Cargo.toml` y `emit_thought_persisted` de `lancedb_thought_repo/src/lib.rs`. Actualizar `gate_eda_unit_no_real_backend.rs` y `allowlists/eda-unit-backend.toml`.
2. **T2 (Satélite nativo `lancedb-memory-adapter`):** Crear `SddIA/tools/lancedb-memory-adapter/` (`Cargo.toml`, `src/main.rs`) enlazando los 3 adaptadores LanceDB bajo `sddia-io`, con retry acotado ante concurrencia y tests unitarios/integración completos.
3. **T3 (Desacople de `execute-process`):** Eliminar dependencias LanceDB de `execute-process/Cargo.toml`; introducir puerto/cliente DI en `memory_evolution_ingest_core.rs`, `handlers/user_preference.rs` (con fast-check de directorio `user_preferences.lance`) y `handlers/aiua_stimulus.rs` (incluyendo emisión de `Thought_Persisted`); migrar `tests/memory_evolution_ingest.rs` a mock/stub.
4. **T4 (Retirada de `thought-graph-access` y SSOT):** Eliminar `SddIA/tools/thought-graph-access/` y `.md`; añadir `paths.lancedbStore` en `cumulo.paths.json`; registrar capacidades `memory:evolution-store`, `memory:pref-vector`, `memory:thought-graph` en `capability-taxonomy.md` y `capability-bindings.md`; actualizar `SddIA/tools/index.md` y `SddIA/process/memory-evolution-ingest.md` / `aiua-stimulus-processing.md`.
5. **T5 (`default-members`, scripts y CI):** Añadir `default-members` a `SddIA/Cargo.toml`; actualizar `build-wasi-capsules.sh`, `build-release-bundle.sh`, `classify-sddia-index-qa-changes.sh` y `.github/workflows/sddia-index-qa.yml` (retirando `protoc` de `wasi-build` y cubriendo `lancedb_preferences` + `lancedb-memory-adapter`).
6. **T6 (Validación y Evolution):** Registrar cambios en `SddIA/evolution/` (DA-2), ejecutar `cargo tree`, `cargo test`, `sddia-qa` gates y generar `validacion.md` (`global: APTO`).
