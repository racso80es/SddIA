---
feature_name: desacople-di-lancedb-execute-process
created: "2026-11-23"
process: refactorization
branch_name: refactor/desacople-di-lancedb-execute-process
persist_ref: docs/refactors/desacople-di-lancedb-execute-process
tracker_ref: AUT-119
base: main
scope: "execute-process, lancedb-memory-adapter, lancedb_*, capability-bindings, cumulo.paths, CI"
---
# Especificación Técnica — desacople-di-lancedb-execute-process (`AUT-119`)

## 1. Arquitectura de Satélite Unificado (`tool:lancedb-memory-adapter`)
- **Ubicación:** `SddIA/tools/lancedb-memory-adapter/` (`Cargo.toml`, `src/main.rs`) + descriptor `SddIA/tools/lancedb-memory-adapter.md`.
- **Dependencias:** `sddia-io`, `sddia-core-memory`, `sddia-infrastructure-lancedb-evolution`, `sddia-infrastructure-lancedb-preferences`, `sddia-infrastructure-lancedb-thought`, `serde_json`, `serde`.
- **Contrato I/O (`capsule-json-io` v2 via `sddia-io`):**
  - Entrada: `sddia_io::read_stdin_json()` (soporta `--request-file`, `SDDIA_CAPSULE_REQUEST`, `SDDIA_SKIP_STDIN`, fail-fast TTY).
  - Desenvuelve `request` si existe (`doc.get("request").unwrap_or(doc)`).
  - Campos comunes: `operation` (string), `repository_path` (string obligatorio).
  - Salida: `emit_v2` (`meta: {schemaVersion: "2.0", entityKind: "tool", entityId: "lancedb-memory-adapter"}`, `success`, `exitCode`, `message`, `result?`, `error?`, `feedback?`).
- **Operaciones soportadas:**
  - `evolution.get_by_id` (`record_id`) → `{found: bool, event?: ...}`
  - `evolution.store` (`event`: `{id, polarity, payload, operational_metadata}`) → `{stored: true, record_id, table: "evolution"}` (calcula embedding internamente con `LocalHashingEmbedder` si falta)
  - `evolution.ingest_telemetry` (`record_id`, `polarity`, `payload`, `operational_metadata`) → `{skipped?: "already_indexed", record_id, store_backend: "lancedb", table: "evolution"}` (operación atómica idempotente para evitar doble spawn en `memory_evolution_ingest_core`)
  - `preferences.put_revision` (`revision`: `UserPreference`) → `{stored: true, preference_id, revision_id}`
  - `preferences.purge` (`preference_id`) → `{purged: true}`
  - `preferences.query_context` (`spec`: `QuerySpec`) → `{context_block: Value}`
  - `thought.search` (`query_text`, `limit?`) → `{memories: [...]}`
  - `thought.store` (`content`, `parent_id?`, `metadata?`) → `{node_id, persisted: true}`
- **Concurrencia:** Helper de retry acotado (backoff corto ante errores transitorios de commit/I/O de LanceDB) en operaciones de escritura.

## 2. Ruta SSOT en `cumulo.paths.json`
- Añadir `paths.lancedbStore: ".SddIA/vector_store/lancedb/"` en `SddIA/core/cumulo.paths.json`.
- Helper compartido en `sddia-core-memory` (o resolución consistente desde `paths.lancedbStore` con fallback a `paths.vectorStore + "lancedb"` para compatibilidad con fixtures existentes).

## 3. Desacople en `execute-process`
- Eliminar `sddia-infrastructure-lancedb-evolution` y `sddia-infrastructure-lancedb-preferences` de `SddIA/engine/execute-process/Cargo.toml`.
- Crear puerto/adaptador ligero en `execute-process` (`VectorMemoryPort` / invocación DI a `tool:lancedb-memory-adapter` vía `invoke_tool_capsule_json` con soporte para stub/mock en tests).
- `memory_evolution_ingest_core.rs`: usa `VectorMemoryPort` (producción: `lancedb-memory-adapter` con `evolution.ingest_telemetry`; tests: mock in-memory).
- `handlers/user_preference.rs`: comprueba existencia rápida del directorio `lancedb_uri(repo).join("user_preferences.lance").is_dir()` antes de invocar al satélite; mantiene fallback a `skill:user-preference-store` / `user-preference-core`.
- `handlers/aiua_stimulus.rs`: migra de `thought-graph-access` a `lancedb-memory-adapter` (`thought.search` / `thought.store`) y emite `Thought_Persisted` tras confirmación `persisted: true` del satélite.

## 4. Oráculos mecánicos ejecutables (Ola 0)
| ID | Criterio | Oráculo mecánico |
| :--- | :--- | :--- |
| AC-1 | Cero refs LanceDB/Tokio en `execute-process` | `cargo tree --manifest-path SddIA/Cargo.toml -p execute-process -e normal \| grep -E "lancedb\|lance\|arrow\|datafusion\|tokio"` (debe devolver vacío) |
| AC-2 | Build/test `execute-process` sin LanceDB | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process` |
| AC-3 | Satélite unificado operativo | `cargo test --manifest-path SddIA/Cargo.toml -p lancedb-memory-adapter` |
| AC-4 | Retirada de `thought-graph-access` | `test ! -e SddIA/tools/thought-graph-access && sddia-qa verify-tools-index` |
| AC-5 | WASI `user-preference-store` intacto | `cargo check --manifest-path SddIA/Cargo.toml -p user-preference-store --target wasm32-wasip1` |
| AC-6 | Purga Redb en `lancedb_thought_repo` | `cargo test --manifest-path SddIA/Cargo.toml -p sddia-infrastructure-lancedb-thought` + `sddia-qa gate-eda-unit-no-real-backend` |
| AC-7 | `default-members` y CI | `grep -q "default-members" SddIA/Cargo.toml` + `sddia-qa verify-process-integrity` |
| AC-8 | Evolution y cierre | `sddia-qa gate-evolution --json --range` + `sddia-qa audit-eda-coverage --scan --json` |
