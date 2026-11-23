---
id: "audit-desacople-di-lancedb-execute-process-20261123"
title: "Auditoría Técnica: Desacople DI de Memoria Vectorial (LanceDB) respecto a execute-process"
type: "audit"
version: "1.0.0"
created: "2026-11-23"
mode: "read-only"
scope:
  - "SddIA/engine/execute-process"
  - "SddIA/infrastructure/adapters/lancedb_*"
  - "SddIA/tools/thought-graph-access"
  - "SddIA/skills/user-preference-store"
  - "SddIA/core/capability-bindings.md"
  - ".github/workflows/sddia-index-qa.yml"
---

# Auditoría Técnica (Solo Lectura): Desacople DI de Memoria Vectorial (LanceDB) respecto a `execute-process`

> **Fecha:** 2026-11-23
> **Alcance:** Inspección directa de código fuente, manifiestos SSOT, configuración de workspace/CI y metadatos de Cargo (`cargo metadata --no-deps`), sin compilación ni modificación de crates.

---

## 1. Patrón de referencia (Desacople DI del Event Store y adaptadores)

### 1.1. Patrones vigentes en el repositorio

El repositorio utiliza **dos planos complementarios** de inyección de dependencias:

1. **Plano interno Rust (Event Store — `eda:event-manager`, ADR-EDA-002):**
   - **Documentación:** `Documentacion/informe_de_dise_o_arquitect_nico_desacople_di_del_event_store.md` define la arquitectura de doble plano: trait nativo en proceso (`EventManagerPort` en `sddia-core`) + cápsula externa opcional.
   - **Implementación real:** `SddIA/sddia-event-manager-host/src/lib.rs` actúa como *composition root* mediante `EventManagerDiFactory` (`productive_factory`, `resolve_productive_port`, `with_productive_port`, `resolve_test_port`, `resolve_role_port`):
     - Rol `default` → `adapter:redb-event-adapter` (`RedbEventAdapter`, enlazado estáticamente en Rust, con reintentos ante lock en `open_redb_adapter` y cierre scoped al salir de `with_productive_port`).
     - Rol `test` → `mock:in-memory-event-manager` (`InMemoryEventManagerMock`).
     - Rol `migration` / `test-fs` → `adapter:fs-event-adapter` (`FsEventAdapter`, prohibido como fallback productivo).
     - Rol `capsule:*` → `adapter:external-capsule-event-adapter` (`SddIA/infrastructure/adapters/external_capsule_event_adapter/src/lib.rs`): puente stateful por `stdin`/`stdout` línea JSON (`{"operation", "payload"}`).
   - **Nota:** `capability-bindings.md` usa prefijos `adapter:` y `mock:` para `eda:event-manager`, los cuales **no** son resueltos por `capability_di_resolver.rs` (que solo resuelve `skill:`, `action:`, `tool:`), sino por `EventManagerDiFactory`.

2. **Plano de cápsulas externas (Skills/Tools — `linear-tracker-adapter`, `thought-graph-access`, etc.):**
   - **Resolución DI:** `SddIA/engine/execute-process/src/engine/capability_di_resolver.rs` (`resolve_phase_bindings`, `load_binding_rows`, `phase_with_effective_delegates`) lee `capability_di.bindings` desde `cumulo.paths.json`, valida `provides` en `SddIA/{skills,actions,tools}/{name}.md`, y resuelve el schema en `SddIA/library/norms/capability-contracts/`.
   - **Aduana pre-ignición:** `SddIA/engine/execute-process/src/engine/capability_di_gate.rs` (`validate_phase_capability_di`) verifica que `capability_id` exista en `SddIA/library/norms/capability-taxonomy.md`, que el proveedor en `delegates_to` declare `provides` con contrato y SemVer compatible (`version_compatible`), y que sus `outputs` cubran los `required` del JSON Schema.
   - **Resolución física de binario:** `SddIA/engine/execute-process/src/engine/capsule_paths.rs` lee `compiled_capsules` en `cumulo.paths.json` (`native_root: "SddIA/target"`, `wasm_root: "SddIA/target/wasm32-wasip1"`, `profiles: ["release", "debug"]`). Si `SDDIA_CAPSULE_ANCHOR=1`, exige testigo `.sha256` + `source_sha256` en el `.md` (`resolve_capsule_native_anchored`). En modo lab (`SDDIA_LAB_MOCK_OUTBOUND=1`), prioriza `debug` sobre `release`.
   - **Invocación y fallback:** `SddIA/engine/execute-process/src/engine/capsules.rs` (`resolve_capsule`, `invoke_capsule_json`, `invoke_tool_capsule_json`, `invoke_tool_for_process`) valida el tipo contra `SddIA/{skills,tools}/index.md` (`validate_capsule_kind_against_ssot`), intenta WASM si `prefer_wasm=true` y hay `wasmtime`, y hace fallback a binario nativo ante errores en `WASM_NATIVE_FALLBACK_MARKERS`. Para `file-tracker-adapter`, hace dispatch inline (`file_tracker_adapter::dispatch`).

### 1.2. Entradas existentes para memoria vectorial / preferencias / evolución

| Archivo SSOT | `thought-graph-access` | `user-preference-store` | Memoria evolutiva / vectorial |
| :--- | :--- | :--- | :--- |
| `SddIA/core/capability-bindings.md` | **No existe** | **Sí** (L49-56): `memory:pref-write` y `memory:pref-query` → `skill:user-preference-store` (`>=1.0.0`) | **No existe** binding para `evolution` ni `thought` |
| `SddIA/library/norms/capability-taxonomy.md` | **No existe** | **Sí** (L55-62): `memory:pref-write`, `memory:pref-query` | **No existe** capacidad para `evolution` ni `thought` |
| `SddIA/core/cumulo.paths.json` | Sin clave propia (usa `paths.vectorStore`) | **Sí** (L124): `paths.userPreferencesStore = ".SddIA/vector_store/user_preferences/"` | **Sí** (L125): `paths.vectorStore = ".SddIA/vector_store/"` |
| `SddIA/skills/index.md` | No aplica | **Sí** (L28): `user-preference-store.md` (`memory:pref-write`, `memory:pref-query`) | No |
| `SddIA/tools/index.md` | **Sí** (L22): `thought-graph-access.md` (`thought_graph_access`, sin `provides` DI) | No aplica | No |

---

## 2. Anatomía de las cápsulas satélite y adaptadores LanceDB

### 2.1. Inventario de crates, targets y ubicación de binarios

| Crate | Ruta física | Tipo de target | Binario / artefacto | Peso medido en disco (`SddIA/target/`) |
| :--- | :--- | :--- | :--- | :--- |
| `thought-graph-access` | `SddIA/tools/thought-graph-access/Cargo.toml` | **Binario nativo** (excluido de WASI en `build-wasi-capsules.sh` L23) | `SddIA/target/{release,debug}/thought-graph-access` | **263.9 MB** (release) / **378.8 MB** (debug) |
| `user-preference-store` | `SddIA/skills/user-preference-store/Cargo.toml` | **Dual: nativo + `wasm32-wasip1`** (compila en `build-wasi-capsules.sh`) | `SddIA/target/{release,debug}/user-preference-store` y `SddIA/target/wasm32-wasip1/debug/user-preference-store.wasm` | **0.96 MB** (release) / **10.6 MB** (debug) / **3.3 MB** (wasm) |
| `sddia-infrastructure-lancedb-evolution` | `SddIA/infrastructure/adapters/lancedb_evolution_repo/Cargo.toml` | **Librería nativa** (`lib`, `lancedb = "=0.37.1"`, `tokio rt-multi-thread`) | Enlazada hoy en `execute-process` | — |
| `sddia-infrastructure-lancedb-preferences` | `SddIA/infrastructure/adapters/lancedb_preferences_repo/Cargo.toml` | **Librería nativa** (`lib`, `lancedb = "=0.37.1"`, `tokio rt-multi-thread`) | Enlazada hoy en `execute-process` | — |
| `sddia-infrastructure-lancedb-thought` | `SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml` | **Librería nativa** (`lib`, `lancedb = "=0.37.1"`, `tokio rt-multi-thread`) | Enlazada hoy en `thought-graph-access` | — |

### 2.2. Motor interno de `user-preference-store` y patrón híbrido en `handlers/user_preference.rs`

- **Motor interno de `user-preference-store`:** Depende **únicamente** de `user-preference-core` y `serde_json` (`SddIA/skills/user-preference-store/Cargo.toml`). `user-preference-core` (`SddIA/user-preference-core/src/lib.rs`) es un store **100% JSON en filesystem** (`head_index.json` + `revisions/{revision_id}.json` bajo `paths.userPreferencesStore`, sin LanceDB, compatible con WASI).
- **Por qué `handlers/user_preference.rs` combina ambos:** En `SddIA/engine/execute-process/src/engine/handlers/user_preference.rs`:
  - **Escritura (`put_revision_durable`, `purge_preference_durable`, `run_capsule`):** Escribe siempre primero en JSON vía `user_preference_core`, y luego hace **réplica condicional** a LanceDB (`replica_put` / `replica_purge`) solo si `LanceDbPreferenceAdapter::table_exists(lancedb_uri(repo))` devuelve `true`.
  - **Lectura (`query_context_block_with_capsule_fallback`, L69-102):** Si la tabla `user_preferences` existe en LanceDB, lee directo desde `LanceDbPreferenceAdapter::open(...)`. Si **no** existe la tabla LanceDB, invoca la cápsula `user-preference-store` (`invoke_capsule_json(..., prefer_wasm=true)`) con `{"op": "QUERY_CONTEXT", "spec": spec}` y, si falla, hace fallback inline a `user_preference_core::query_context_block`.

### 2.3. Contrato I/O de las cápsulas satélite

| Aspecto | `thought-graph-access` (`SddIA/tools/thought-graph-access/src/main.rs`) | `user-preference-store` (`SddIA/skills/user-preference-store/src/main.rs`) |
| :--- | :--- | :--- |
| **Librería I/O** | Usa `sddia_io::read_stdin_json()` (`SddIA/sddia-io/src/lib.rs`): soporta `--request-file`, `SDDIA_CAPSULE_REQUEST`, `SDDIA_SKIP_STDIN` y **guarda Fail-Fast TTY** (`io::stdin().is_terminal()`). | **No usa `sddia-io`**. Lee `io::stdin().read_to_string()` directamente (sin fail-fast TTY, sin `--request-file`, sin `SDDIA_CAPSULE_REQUEST`). |
| **Envelope de salida** | `emit_v2`: `meta` (`schemaVersion: "2.0"`, `entityKind: "tool"`, `entityId`), `success`, `exitCode`, `message`, `result`, `feedback`/`error`. | Plano sin `meta`: `{"success", "exitCode", "message", "result"}`. |
| **Estructura de request** | Acepta `{ "request": { ... } }` o plano (`request_inner`). Campos obligatorios: `operation` (`"search"` \| `"store"`) y `repository_path`. Para `search`: `query_text`, `limit` (default 5). Para `store`: `content`, `parent_id?`, `metadata?`. | Plano (no desenvuelve `request`). Acepta `op` o `operation` (case-insensitive): `"PUT"`, `"ACTIVATE"`, `"PROPOSE"`, `"REVOKE"`, `"SUPERSEDE"`, `"QUERY"`, `"PURGE"`, `"EXPORT"`, `"QUERY_CONTEXT"`. Campos: `revision?`, `spec?`, `preference_id?`. |
| **Resolución de repo** | Explícita vía `request.repository_path`. | Implícita caminando hacia arriba desde `std::env::current_dir()` buscando `SddIA/core/cumulo.paths.json`. |

### 2.4. Tablas y rutas físicas de LanceDB en `.SddIA/`

Estado verificado en disco bajo `.SddIA/vector_store/`:
- **Directorio LanceDB único:** `.SddIA/vector_store/lancedb/`
  - Tabla `evolution.lance` (`TABLE_EVOLUTION = "evolution"` en `SddIA/infrastructure/adapters/lancedb_evolution_repo/src/lib.rs` L20).
  - Tabla `thought_graph_collection.lance` (`TABLE_THOUGHT = "thought_graph_collection"` en `SddIA/infrastructure/adapters/lancedb_thought_repo/src/lib.rs` L18).
  - Tabla `user_preferences` (`TABLE_PREFERENCES = "user_preferences"` en `SddIA/infrastructure/adapters/lancedb_preferences_repo/src/lib.rs` L24): **no materializada aún en disco** en este workspace.
- **Almacenes JSON coexistentes en `.SddIA/vector_store/`:**
  - `.SddIA/vector_store/evolution/stim-*.json` (archivos legacy previos a LanceDB).
  - `.SddIA/vector_store/user_preferences/{head_index.json, revisions/*.json}` (almacén JSON activo actual de preferencias).
- **Resolución de ruta:** Los tres consumidores (`memory_evolution_ingest_core::lancedb_uri`, `lancedb_preferences_repo::default_lancedb_uri`, `thought-graph-access::resolve_lancedb_path`) leen `SddIA/core/cumulo.paths.json` → `/paths/vectorStore` (default `.SddIA/vector_store/`) y concatenan el subdirectorio literal `"lancedb"`.

---

## 3. Superficie exacta de uso en `execute-process`

### 3.1. Operaciones invocadas sobre los puertos de memoria

| Dominio / Puerto | Archivo y función en `execute-process` | Métodos invocados |
| :--- | :--- | :--- |
| **`EvolutionStore`** (`LanceDbEvolutionAdapter`) | `SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs` (`ingest_ecst_body`, L176-216) | 1. `LanceDbEvolutionAdapter::open(&uri)`<br>2. `adapter.get_event_by_id(&record_id)` (deduplicación)<br>3. `adapter.store_event(event)` (escritura/upsert)<br>*(No invoca `search_similar_events` ni `row_count`)* |
| **`UserPreferenceStore`** (`LanceDbPreferenceAdapter`) | `SddIA/engine/execute-process/src/engine/handlers/user_preference.rs` (`prefs_table_exists`, `replica_put`, `replica_purge`, `query_context_block_with_capsule_fallback`, L12-77) | 1. `LanceDbPreferenceAdapter::table_exists(uri)`<br>2. `LanceDbPreferenceAdapter::open(uri)`<br>3. `UserPreferenceStore::put_revision(&adapter, pref)`<br>4. `UserPreferenceStore::purge_preference(&adapter, preference_id)`<br>5. `UserPreferenceStore::query_context_block(&adapter, spec)`<br>*(No invoca `get_active` ni `query` directamente)* |
| **`ThoughtGraphRepository`** (ya desacoplado vía cápsula) | `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` (`invoke_thought_graph`, L112-140 y L218) | Invoca `invoke_tool_capsule_json(repo, "thought-graph-access", &payload, false)` con `operation: "search"` y `operation: "store"`. |

Consumidores internos en `execute-process` de estas funciones:
- `memory_evolution_ingest_core`: llamado desde `src/engine/mod.rs` (L111, `ingest_from_process_inputs`) y `src/engine/route_fractal_core.rs` (L12, `ingest_domain_event_file`).
- `handlers::user_preference`: llamado desde `src/engine/mod.rs` (L222, `run_ingest`), `src/engine/handlers/email_triage.rs` (L734, `query_context_block_with_capsule_fallback`), `src/engine/handlers/telegram_fallback.rs` (`build_pref_context_hint`) y `src/engine/handlers/email_digest_preference_reply.rs` (L442, `run_ingest`).

### 3.2. Otros usos de `tokio` o adaptadores LanceDB en `execute-process`

- **`tokio`:** `execute-process` **no** declara `tokio` en `SddIA/engine/execute-process/Cargo.toml` ni existe ningún `use tokio` en `src/` o `tests/`. `tokio` entra **únicamente de forma transitiva** a través de `sddia-infrastructure-lancedb-evolution` y `sddia-infrastructure-lancedb-preferences`.
- **Adaptadores LanceDB:** Solo los dos módulos anteriores (`memory_evolution_ingest_core.rs` y `handlers/user_preference.rs`) y el test de integración `SddIA/engine/execute-process/tests/memory_evolution_ingest.rs` (que solo llama a `ingest_domain_event_file`, `lancedb_uri` y `vector_store_root`, sin importar el crate LanceDB).

### 3.3. Generación de embeddings

- Se realiza **localmente en Rust puro sin red ni runtime async**, mediante `LocalHashingEmbedder` en `SddIA/core/memory/src/services/inference_binding.rs` (`EMBEDDING_DIM = 384`, `EMBEDDING_MODEL = "sddia-local-hashing-v1"`, `EMBEDDING_NORM = "l2"`, hashing FNV-1a de trigramas con normalización L2).
- `sddia-core-memory` (`SddIA/core/memory/Cargo.toml`) solo depende de `hex`, `serde`, `serde_json` y `sha2` (ultraligero).
- **Dónde se invoca hoy:**
  - En `execute-process` (`memory_evolution_ingest_core.rs` L213): pre-calcula el embedding antes de `store_event`, pero **`LanceDbEvolutionAdapter::upsert` ya lo calcula automáticamente si `embedding` es `None` o ceros** (`lancedb_evolution_repo/src/lib.rs` L255-259).
  - En `LanceDbPreferenceAdapter`: lo calcula internamente en `resolve_embedding` (en `upsert`) y en `query` cuando recibe `query_text` (`lancedb_preferences_repo/src/lib.rs` L315-326 y L574-576).
  - En `LanceDbThoughtRepo` y `thought-graph-access`: `upsert` lo calcula si `embedding` es `None`; `thought-graph-access::search` lo calcula con `LocalHashingEmbedder` antes de `search_similar_thoughts`.

---

## 4. Workspace, empaquetado y CI

### 4.1. Configuración de `SddIA/Cargo.toml` y grafo de dependencias LanceDB

- `SddIA/Cargo.toml` declara **54 miembros** en `members` y **no define `default-members`** (verificado con `cargo metadata --no-deps`: `workspace_default_members: 54 == workspace_members: 54`). Por tanto, `cargo build --workspace` compila todos los crates nativos, incluidos los 3 adaptadores LanceDB y `thought-graph-access`.
- **Únicos crates del workspace que dependen directa o indirectamente de `lancedb = "=0.37.1"`:**
  1. `sddia-infrastructure-lancedb-evolution` → consumido solo por `execute-process`.
  2. `sddia-infrastructure-lancedb-preferences` → consumido solo por `execute-process`.
  3. `sddia-infrastructure-lancedb-thought` → consumido solo por `thought-graph-access`.

### 4.2. Compilación y verificación en CI, `delivery-close-cycle` y `qa_attestation`

- **CI (`.github/workflows/sddia-index-qa.yml`):**
  - `native-build`: instala `protobuf-compiler`/`libprotobuf-dev` (`protoc`, exigido por `lance-encoding`), ejecuta `cargo build --workspace`, empaqueta todos los binarios `src/main.rs` vía `SddIA/scripts/qa/stage-native-debug-bins.sh` y los sube como artefacto `native-debug-bins`.
  - `wasi-build`: **también instala `protoc` hoy** solo porque compila `execute-process` (`cargo build -p event-watcher -p execute-process -p sddia-qa`, L133). Excluye `thought-graph-access` en `SddIA/scripts/qa/build-wasi-capsules.sh` y **no** empaqueta binarios satélite LanceDB en el artefacto `wasi-runtime` (solo `sddia-qa`, `event-watcher`, `execute-process` y `linear-tracker-adapter`).
  - `sddia-index-integrity`: corre `sddia-qa verify-compiled-capsules` (descubre todo crate con `src/main.rs` bajo `engine|skills|tools|daemons|interfaces` y exige su binario en `SddIA/target/{release,debug}/`, `SddIA/tools/sddia-qa/src/verify_compiled_capsules.rs`). Si `changes.outputs.lance_memory == 'true'` (`SddIA/scripts/qa/classify-sddia-index-qa-changes.sh`), corre `cargo test` para `sddia-core-memory`, `lancedb_thought`, `lancedb_evolution` y `execute-process --test memory_evolution_ingest` *(nota: `lancedb_preferences` no está incluido en L222)*.
- **Empaquetado y sincronización local:**
  - `SddIA/scripts/sync-live-organs.sh` recompila en `--release` y sella testigos `.sha256` **solo para los 9 `LIVE_ORGANS`** definidos en `SddIA/sddia-runtime-parity/src/lib.rs` (L12-22: `execute-process`, `sddia-qa` y 7 daemons). `verify-compiled-capsules` solo valida frescura (`check_binary_freshness`) sobre `LIVE_ORGANS`, no sobre cápsulas satélite como `thought-graph-access`.
  - `SddIA/scripts/build-release-bundle.sh` incluye `thought-graph-access` explícitamente en `CONSUMER_BINS` (L65 y verificación L351).
- **`delivery-close-cycle` y `qa_attestation.rs`:**
  - `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` y `SddIA/engine/execute-process/src/engine/qa_attestation.rs` **no compilan ni inspeccionan binarios satélite**: validan índices (`verify-process-integrity`, `verify-tools-index`), `gate-evolution`, `audit-eda-coverage` y la atestación `qa-attestation/1.0` (`tree_sha` + `issuer_process = pull-request-review`).

---

## 5. Tabla comparativa con base empírica: Opción A vs Opción B

| Criterio (evidencia verificada) | Opción A: Único binario nativo LanceDB (dispatch por dominio/operación) | Opción B: Binarios nativos separados (`thought-graph-access`, `memory-evolution-store`, `user-preference-store`) |
| :--- | :--- | :--- |
| **Tamaño en disco (release / debug)** | **~264 MB** release / **~379 MB** debug (1 único enlazado de `lancedb 0.37.1` + DataFusion + Arrow + Tokio). | **~792 MB** release / **~1.14 GB** debug (3× ~264 MB / ~379 MB; medido sobre `thought-graph-access`). |
| **Tiempo de linkeo y CI (`native-build` + artefacto `native-debug-bins`)** | 1 linkeo pesado; artefacto CI crece ~0 MB respecto al estado actual (reemplaza el peso de `execute-process` + `thought-graph-access` por 1 binario). | 3 linkeos pesados en `cargo build --workspace`; triplica el peso de `native-debug-bins` y `build-release-bundle.sh`. |
| **Desacople de `execute-process` (liberar `protoc` y `tokio`)** | **Total**: `execute-process` elimina las 2 deps LanceDB y `tokio` transitivo; `wasi-build` deja de necesitar `protoc`. | **Total**: mismo beneficio para `execute-process`. |
| **Compatibilidad con `user-preference-store` WASI** | **Mantiene `skill:user-preference-store` intacto como WASI ultraligero (0.96 MB)** para JSON fallback, mientras la tabla LanceDB vive en el proveedor vectorial unificado. | **Rompe la portabilidad WASI de `user-preference-store`** si se le enlaza `lancedb` directamente (habría que añadirlo a `EXCLUDE` en `build-wasi-capsules.sh` y pasaría de 0.96 MB a ~264 MB), o bien requeriría crear un 4º crate `lancedb-preference-adapter`. |
| **Alineación con taxonomía DI (`capability-bindings.md` y `capability_di_resolver.rs`)** | **Alta**: un único `tool` (p. ej. `tool:lancedb-memory-adapter`) puede declarar múltiples entradas en `provides:` (`memory:thought-graph`, `memory:evolution-store`, `memory:pref-vector`), igual que `filesystem-manager` provee `doc:closure` y `fs:persist`. | **Alta**, pero exige mantener 3 manifiestos `.md`, 3 entradas en `tools/index.md`, 3 entradas en `CONSUMER_BINS`, 3 entradas en `EXCLUDE` de `build-wasi-capsules.sh` y 3 allowlists en `sddia-qa`. |
| **Runtime Tokio / recursos por invocación** | 1 solo `OnceLock<tokio::runtime::Runtime>` (multi-thread) por proceso si opera por invocación o modo daemon/RPC; comparte el mismo directorio `.SddIA/vector_store/lancedb/`. | 3 binarios distintos abriendo el mismo directorio `.SddIA/vector_store/lancedb/` con runtimes Tokio independientes. |
| **Aislamiento de fallos / granularidad NFT** | Menor aislamiento por dominio (un fallo de compilación en una tabla bloquea el binario único); un solo activo NFT de sustrato tecnológico (`lancedb`). | Máximo aislamiento por dominio (cada tabla es un binario independiente), pero multiplica por 3 la deuda de build/distribución. |

---

## 6. Riesgos y bloqueos ocultos detectados

1. **Pérdida de portabilidad WASI de `user-preference-store` (si se elige Opción B sobre el crate existente):**
   - Hoy `SddIA/skills/user-preference-store` compila a `wasm32-wasip1` (3.3 MB) porque solo depende de `user-preference-core`. Añadirle `sddia-infrastructure-lancedb-preferences` rompería `build-wasi-capsules.sh` (fallo de compilación WASI por `lancedb`/`tokio`/`lance-encoding`).
2. **Acoplamiento oculto de `lancedb_thought_repo` al Event Store productivo (Redb):**
   - `SddIA/infrastructure/adapters/lancedb_thought_repo/Cargo.toml` (L9-11) y `src/lib.rs` (L204-240) importan `sddia-event-manager-host` y emiten `Thought_Persisted` directamente sobre Redb dentro de `store_thought`. Si un binario unificado o separado es invocado desde `execute-process` mientras otro proceso mantiene abierto `events.redb`, entra en contención de lock de Redb (y requiere estar en la allowlist de `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs` L73 / `allowlists/eda-unit-backend.toml`).
3. **Asimetría de contrato I/O en `user-preference-store`:**
   - `user-preference-store` **no** usa `sddia-io` (sin guarda fail-fast TTY ni `--request-file`), no acepta `{ "request": ... }`, y resuelve `repo_root()` desde `current_dir()` en lugar de `repository_path`. Cualquier unificación o reemplazo debe normalizar esto hacia `sddia-io` (`capsule-json-io` v2).
4. **Rutas hardcoded y dispersas a `lancedb`:**
   - El subdirectorio literal `"lancedb"` está hardcoded en 3 sitios distintos (`memory_evolution_ingest_core.rs` L16, `lancedb_preferences_repo/src/lib.rs` L26, `thought-graph-access/src/main.rs` L62). `cumulo.paths.json` solo declara `paths.vectorStore = ".SddIA/vector_store/"`, no el subdir `lancedb`.
5. **Concurrencia y latencia de arranque por subproceso (LanceDB + Tokio):**
   - Cada adaptador crea un `tokio::runtime::Builder::new_multi_thread()` en un `OnceLock` y ejecuta `ensure_table()` (`table_names()` + `open_table()` + `schema()`) en cada `open()`. En invocación *one-shot* por subproceso (`invoke_tool_capsule_json`), cada llamada paga spawn de proceso + inicialización de pool Tokio + apertura de tablas LanceDB.
   - Además, `LanceDbPreferenceAdapter::table_exists()` (`lancedb_preferences_repo/src/lib.rs` L391-406) abre una conexión LanceDB completa solo para comprobar si la tabla existe; si se invoca como subproceso en cada `query_context_block` / `replica_put`, duplicaría llamadas a menos que el propio binario encapsule la semántica condicional o use un puente stateful como `ExternalCapsuleEventAdapter`.
   - En escrituras concurrentes (`merge_insert` sobre `evolution.lance` o `user_preferences.lance` desde procesos paralelos como `event-watcher` y `execute-process`), LanceDB usa commits optimistas en `_transactions/` sin lock cooperativo en el adaptador (a diferencia de `open_redb_adapter` en `sddia-event-manager-host`, que tiene retry budget).
6. **Huecos en CI y `wasi-runtime`:**
   - En `.github/workflows/sddia-index-qa.yml` (L130-155), el job `wasi-build` **no** compila ni empaqueta `thought-graph-access` (ni futuros binarios LanceDB) dentro del artefacto `wasi-runtime`. Si un test de `wasi-runtime-smoke` o `eda-bus-e2e-smoke` dispara `memory-evolution-ingest` o `user-preference-ingest` delegando a un binario nativo satélite, fallará con `cápsula no encontrada bajo SddIA/target` salvo que se incluya en `wasi-build` / `wasi-runtime`.
   - `SddIA/scripts/qa/classify-sddia-index-qa-changes.sh` (L19) **no** observa `SddIA/tools/thought-graph-access/*` ni `SddIA/skills/user-preference-store/*`, y la línea de `cargo test` en `sddia-index-qa.yml` (L222) omite `-p sddia-infrastructure-lancedb-preferences`.
   - `verify-compiled-capsules` solo verifica frescura (`source_drift` / `mtime`) para `LIVE_ORGANS` (los 9 órganos); un binario satélite LanceDB modificado podría quedar *stale* en `SddIA/target/release/` en local sin que `sync-live-organs.sh` lo recompile.
7. **Semántica de embeddings al desacoplar:**
   - `LocalHashingEmbedder` es determinista y vive en `sddia-core-memory` (sin `lancedb`), pero si en el futuro se sustituye por un modelo neuronal/remoto, conviene que la generación de embeddings quede encapsulada dentro del proveedor vectorial (como ya hacen `upsert` y `query` en los 3 adaptadores) para que `execute-process` nunca necesite conocer la dimensión (`384`) ni enviar vectores crudos por JSON `stdin`.
