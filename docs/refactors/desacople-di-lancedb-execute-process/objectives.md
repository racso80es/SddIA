---
feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
---
# Objetivos y Criterios de Aceptación — desacople-di-lancedb-execute-process (`AUT-119`)

## Objetivo
Desacoplar por completo `execute-process` de LanceDB y Tokio mediante Inyección de Dependencias (`capability-bindings.md` + puertos puros) y consolidar los tres adaptadores vectoriales (`evolution`, `preferences`, `thought`) en un único binario nativo satélite (`tool:lancedb-memory-adapter`) bajo `sddia-io` (`capsule-json-io` v2), preservando paridad funcional estricta y la portabilidad WASI de `skill:user-preference-store`.

## Criterios de Aceptación
- [ ] **AC-1 (Árbol de dependencias limpio):** `cargo tree -p execute-process` confirma cero referencias (directas y transitivas) a `lancedb`, `lance`, `arrow`, `datafusion` y `tokio`.
- [ ] **AC-2 (Compilación y tests Core sin LanceDB/protoc):** `cargo build -p execute-process --release` y `cargo test -p execute-process` completan sin compilar ningún crate de LanceDB y sin requerir `protoc`.
- [ ] **AC-3 (Satélite Nativo Unificado `lancedb-memory-adapter`):** Existe un único binario nativo satélite (`tool:lancedb-memory-adapter`) que sirve `evolution.*`, `preferences.*` y `thought.*` bajo `sddia-io` (`capsule-json-io` v2, fail-fast TTY, anti-panic, embeddings encapsulados y retry ante concurrencia) y pasa sus tests de integración contra LanceDB.
- [ ] **AC-4 (Retirada de `thought-graph-access`):** `thought-graph-access` queda absorbido y retirado sin dejar referencias rotas en `SddIA/tools/index.md`, `build-wasi-capsules.sh` ni `build-release-bundle.sh`.
- [ ] **AC-5 (Preservación WASI y fast-check):** `skill:user-preference-store` sigue compilando a `wasm32-wasip1` sin LanceDB y `handlers/user_preference.rs` evita invocar el satélite cuando `user_preferences.lance` no existe en disco.
- [ ] **AC-6 (Purga Redb en `lancedb_thought_repo`):** `lancedb_thought_repo` no depende de `sddia-event-manager-host`; `Thought_Persisted` se emite desde `aiua_stimulus.rs`, y se poda su excepción en `allowlists/eda-unit-backend.toml`.
- [ ] **AC-7 (SSOT rutas, `default-members` y CI):** `cumulo.paths.json` define la ruta canónica de LanceDB; `SddIA/Cargo.toml` define `default-members` excluyendo crates LanceDB; `wasi-build` en `.github/workflows/sddia-index-qa.yml` no instala `protoc` y `sddia-index-qa.yml` / `classify-sddia-index-qa-changes.sh` cubren `lancedb_preferences` y `lancedb-memory-adapter`.
- [ ] **AC-8 (Paridad EDA, Evolution y Cierre):** Paridad intacta de eventos (`Vector_Memory_Indexed`, `User_Preference_Changed`, `Thought_Persisted`), registro en `SddIA/evolution/` (DA-2) y paso en verde de aduanas con `global: APTO`.
