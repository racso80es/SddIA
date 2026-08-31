---
feature_name: lancedb-real-vector-memory
created: "2026-08-31"
process: feature
purpose: Estabilización Mayeuta — PBI-CORE-LANCEDB-REAL-001 v1.1.0
branch_name: feat/lancedb-real-vector-memory
persist_ref: docs/features/lancedb-real-vector-memory
pbi_ref: docs/todos/pending/[ARQUITECTURA] LanceDB — integración física real y memoria vectorial efectiva.md
document_id: PBI-CORE-LANCEDB-REAL-001
uuid: "3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46"
execution_id: "c4e7971d-7c67-4745-b1b4-eb8b3d84d652"
mayeuta_verdict: ok
laudo: host-nativo-lancedb-hashing-embed-ssot-vectorstore
---

# Clarificación — lancedb-real-vector-memory

Transcript Mayeuta. Semilla: PBI `PBI-CORE-LANCEDB-REAL-001` v1.1.0. Init lab `execution_id` `c4e7971d-7c67-4745-b1b4-eb8b3d84d652`. Filtro A contra genoma vigente.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `lancedb-real-vector-memory` |
| Rama | `feat/lancedb-real-vector-memory` |
| `persist_ref` | `docs/features/lancedb-real-vector-memory` |
| `document_id` | `PBI-CORE-LANCEDB-REAL-001` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery |
| `execution_id` | `c4e7971d-7c67-4745-b1b4-eb8b3d84d652` |
| Stop planning | esta parada: clarify + objectives + spec + plan + commit; ejecución T0–Tn después |

**Toll:** un `persist_ref`, un PR.

---

## D1 — Estado actual (Filtro A, no narrativa)

| Hecho | Evidencia |
|-------|-----------|
| Ningún `Cargo.toml` declara crate `lancedb` | grep workspace |
| `lancedb_thought_repo` cuatro métodos placeholder | `Ok(())` / `Ok(None)` / `Ok(vec![])` |
| `lancedb_evolution_repo::store_event` escribe `{id}.json` | `.SddIA/vector_store/evolution/` |
| `memory_evolution_ingest_core.rs` **no** usa el puerto | `STORE_REL` hardcodeado; JSON inline |
| `LocalSemanticInference` = `vec![0.0; 384]` | `inference_binding.rs` |
| `core/memory` y ambos adaptadores fuera de `workspace.members` | `SddIA/Cargo.toml` |
| Cúmulo solo declara `paths.userPreferencesStore` | `cumulo.paths.json` v1.8.0 |
| Fichas adaptador `status: placeholder` | `adapters/index.md` |

Deuda `docs/features/memoria-vectorial/plan.md` fase 5: «integración LanceDB real». Este PBI la cierra. No reabre preferencias de usuario.

---

## D2 — Crate y matriz de targets

| Decisión | Valor |
|----------|-------|
| Crate | `lancedb` (crates.io) |
| Pin | `0.38` (crates.io 2026-08) |
| API | async (`connect().execute().await`, `merge_insert`, `query().nearest_to`) |
| Native host | **mínimo obligatorio** |
| Toolchain | **`protoc` obligatorio.** PoC `/tmp/lancedb-target-poc` (`lancedb = "0.38"`): `cargo check` nativo **falló** en `lance-encoding v11.0.0` — `Could not find protoc`. Host y CI deben instalar `protobuf-compiler` (Debian/Ubuntu) o exportar `PROTOC`. |
| `wasm32-wasip1` | **fuera**. PoC WASI no se ejecutó (el nativo abortó antes). Sysdeps (`lzma-sys`, Arrow, FS, `protoc`) descartan WASI ficticio. |

Evidencia PoC native: exit 101, `lance-encoding` build-script. Reintento T0 con `protoc` en PATH.

Cápsulas WASI, si participan, JSON stdin/stdout (`capsule-json-io.md`); no abren LanceDB.

---

## D3 — Frontera async

`execute-process` es síncrono. Puertos de `core/memory` **permanecen sync** (hexágono agnóstico; no Tokio en dominio).

| Capa | Contrato |
|------|----------|
| Core | traits sync; cero tipos LanceDB/Arrow/Tokio |
| Adaptador | un `tokio::runtime::Runtime` creado **una vez** en `open()`; `block_on` por método del adaptador |
| Composición host | `open(path)` en el handler; prohibido `Runtime::new()` por operación |

No esconder un runtime por llamada. No `block_on` dentro de `core/memory`.

---

## D4 — SSOT de ruta

Clave nueva `paths.vectorStore` = `.SddIA/vector_store/`.

URI LanceDB **inyectada**, no inferida: `{vectorStore}/lancedb/` (subdirectorio aislado). Motivo: coexistir con JSON legado `evolution/*.json` y con `user_preferences/` sin contaminar el catálogo Lance.

| Prohibido | Sustituto |
|-----------|-----------|
| `STORE_REL` en `memory_evolution_ingest_core.rs` | `load_paths_config` → `paths.vectorStore` |
| `connection_string: ".SddIA/vector_store/evolution/"` en adaptador | `LanceDbEvolutionAdapter::open(&path)` |

Tests: `tempdir`, nunca el store del operador.

---

## D5 — Embeddings

Sustituir ceros constantes. CA13 exige CI **sin red** → no `fastembed`/ONNX con descarga de modelo.

| Proveedor | `sddia-local-hashing-v1` |
|-----------|--------------------------|
| Algoritmo | hashing de n-gramas (n=3) con signo, dim 384, L2 |
| Determinismo | misma entrada → mismo vector bit a bit |
| No constante | textos distintos → vectores distintos (norma > 0) |
| Fallo | input vacío o dim ≠ 384 → error tipado; **prohibido** degradar a ceros |

Metadata persistida: `embedding_model`, `embedding_dim`, `embedding_norm=l2`. Interfaz `EmbeddingGenerator` / `SemanticInference` permite swap futuro a MiniLM **sin** cambiar puertos de store.

KNN de aceptación usa fixtures con vectores conocidos (no depende de semántica neural).

---

## D6 — Puertos y esquemas

`EvolutionStore` hoy solo `store_event`. Ampliar (mínimo verificable):

- `get_event_by_id`
- `search_similar_events` (KNN)

Tablas:

| Tabla | Clave | Vector |
|-------|-------|--------|
| `thought_graph_collection` | `node_id` | `embedding` FixedSizeList\<f32, 384\> |
| `evolution` | `id` | igual |

Dimensión incompatible → error; no truncado/padding/ceros. Schema drift al abrir → error. Upsert via `merge_insert` sobre la clave (idempotencia de tabla, no de archivo).

---

## D7 — Runtime ingest y JSON legado

Una sola ruta de persistencia: handler → `EvolutionStore` → LanceDB.

JSON `{id}.json` deja de ser SSOT. No fallback silencioso. Migración: importador **explícito** e idempotente (`import_legacy_evolution_json`); no se dispara en cada ingest. Retirada de JSON = decisión posterior (fuera de alcance automático).

---

## D8 — Workspace / CI

Incluir en `SddIA/Cargo.toml` `members`:

- `core/memory`
- `infrastructure/adapters/lancedb_evolution_repo`
- `infrastructure/adapters/lancedb_thought_repo`

`execute-process` depende del adaptador evolution + core memory. WASI capsules script **no** construye estos crates (solo skills/tools/interfaces). CI nativo: `cargo build --workspace` ya cubre compile; añadir step de test de los tres crates + filtro ingest **sin red**.

Fichas adaptador: `status: placeholder` → `active` al cerrar T-adapters. Mutación de `{name}.md` + `index.md` **directa** (familia `infrastructure-adapter`; no `entity-manager` v1).

`cumulo.paths.json` no es genoma DA-2; bump `1.8.0` → `1.9.0`.

---

## D9 — Fuera

Preferencias de usuario. LanceDB remoto. UI. Reentrenamiento. Borrado automático de JSON legado. Evento nuevo `Vector_Memory_Indexed` (ya existe en deuda memoria-vectorial; no es AC de este PBI).

---

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / `feature` v1.3.2
- `CONSTITUTION_CORE` — Triaje C/A/B; Verdad Objetiva
- DA-2/DA-3: genoma vía `entity-manager`; este ciclo **no** muta tools/skills/process/events/norms
- DA-4: topología activa `docs/features/lancedb-real-vector-memory/`
- DA-5 fire-and-forget post-acuse CLI
- Rutas vía `cumulo.paths.json`
- Git vía `skill:git-manager`
