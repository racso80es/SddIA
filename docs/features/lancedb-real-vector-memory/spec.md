---
feature_name: lancedb-real-vector-memory
created: "2026-08-31"
process: feature
base: main
scope: lancedb-real-vector-memory
version_spec: "1.0.0"
document_id: PBI-CORE-LANCEDB-REAL-001
uuid: "3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46"
---

# Especificación — lancedb-real-vector-memory

## 1. Contratos vigentes (no reinventar)

| Pieza | Hecho |
|-------|--------|
| `sddia-core-memory` | Modelos `ThoughtNode` / `EvolutionEvent`; traits `ThoughtGraphRepository`, `EvolutionStore` (solo `store_event`), `EmbeddingGenerator` |
| Adaptadores | Crates path-dep; fichas `status: placeholder`; **sin** crate `lancedb` |
| Ingest | `memory_evolution_ingest_core.rs` escribe JSON; tests asumen `store_path` fichero |
| Cúmulo | `paths.userPreferencesStore`; no raíz vectorial común |
| Adapters-contract v1.0.0 | `{name}.md` + `index.md`; familia **no** pasa por `entity-manager` |
| `capsule-json-io.md` | WASI no abre FS LanceDB |

## 2. Hexágono

```text
core/memory          ← modelos, puertos, embedder hashing, errores tipados
infrastructure/adapters/lancedb_*  ← connect, schema, Arrow, merge_insert, KNN
execute-process      ← composición: Cúmulo → PathBuf → Adapter::open → ingest
```

Prohibido importar `lancedb` / `arrow` / `tokio` en `SddIA/core/memory/`.

### 2.1 Errores de dominio (core)

Enum serializable a `String` en el puerto (`type Error = MemoryStoreError` o `String` mapeado):

| Variante | Disparo |
|----------|---------|
| `DimensionMismatch { expected, actual }` | embedding.len() ≠ 384 |
| `SchemaIncompatible { reason }` | tabla existente con dim/tipo distinto |
| `StoreCorrupt { reason }` | open falla sobre path no vacío ilegible |
| `EmbeddingFailed { reason }` | texto vacío / norma cero |
| `Io { reason }` | FS |

### 2.2 Puertos ampliados

`ThoughtGraphRepository` — ya declara los 4 métodos; implementarlos de verdad.

`EvolutionStore`:

```text
store_event(event) -> Result<()>
get_event_by_id(id) -> Result<Option<EvolutionEvent>>
search_similar_events(query_embedding, limit) -> Result<Vec<EvolutionEvent>>
```

Idempotencia: `store_event` con el mismo `id` = upsert; no duplicar filas.

## 3. Persistencia LanceDB

### 3.1 Ubicación

| Clave Cúmulo | Valor default | URI driver |
|--------------|---------------|------------|
| `paths.vectorStore` | `.SddIA/vector_store/` | `{repo}/{vectorStore}/lancedb/` |

Bump manifiesto Cúmulo `1.8.0` → `1.9.0`. `.SddIA/vector_store/` permanece gitignored.

### 3.2 Tablas

**`thought_graph_collection`**

| Columna | Tipo Arrow | Notas |
|---------|------------|-------|
| `node_id` | Utf8 | clave merge |
| `parent_id` | Utf8 nullable | hijos exactos |
| `content` | Utf8 | |
| `metadata` | Utf8 | JSON |
| `friction_trace` | Utf8 nullable | |
| `embedding` | FixedSizeList\<Float32, 384\> | requerido al persistir |
| `embedding_model` | Utf8 | `sddia-local-hashing-v1` |
| `embedding_dim` | UInt16 | 384 |
| `embedding_norm` | Utf8 | `l2` |

**`evolution`** — `id`, `polarity`, `payload`, `operational_metadata` (JSON Utf8), mismo bloque embedding.

Apertura: `create_table` si ausente; si existe, validar schema (dim del FixedSizeList). Incompatible → `SchemaIncompatible`.

Upsert: `merge_insert` on `node_id` / `id`, matched update all, not-matched insert all.

KNN: `query().nearest_to(vec).limit(k)` sobre columna `embedding`. Sin índice ANN en v1 (flat search; volumen local pequeño). Orden = distancia creciente; test con tres vectores ortogonales/conocidos.

### 3.3 Async

`LanceDb*Adapter` posee `Arc<tokio::runtime::Runtime>` (multi-thread, `enable_all`) creado en `open()`. Cada método público del trait hace `self.rt.block_on(...)`. Drop del adapter cierra el runtime. Tests: open → write → drop adapter → `open` de nuevo → read.

## 4. Embeddings

`LocalHashingEmbedder` en `core/memory` (`inference_binding.rs`):

1. Rechazar `text.trim().is_empty()`.
2. N-gramas de bytes n=3 sobre texto lowercase con centinelas `^` `$`.
3. Hash estable (p. ej. FNV-1a 32) → índice `h % 384`, signo según bit alto.
4. L2; si norma == 0 → `EmbeddingFailed`.
5. `embed_event` escribe `event.embedding`; no ceros.

Tests: dos textos distintos → vectores ≠; misma entrada dos veces → iguales; ningún componente-all-zero del stub previo como único resultado.

Si el caller aporta embedding, validar dim 384 **antes** de Arrow.

## 5. Wiring ingest

`ingest_inner` en `memory_evolution_ingest_core.rs`:

1. Resolver raíz: `load_paths_config(repo).paths.vectorStore` (default `.SddIA/vector_store/` si clave ausente **solo** en tests con cfg inyectado; en producto la clave existe).
2. `LanceDbEvolutionAdapter::open(repo.join(vectorStore).join("lancedb"))`.
3. Construir `EvolutionEvent` (id actual `stim-` / sha256; polaridad; payload; metadata).
4. `LocalHashingEmbedder.embed_event`.
5. `store_event`.
6. **No** `write_json_atomic` al store evolution.
7. Respuesta JSON: `record_id`, `store_backend: "lancedb"`, `table: "evolution"`. Dejar de devolver path de `.json` como SSOT.

Idempotencia: reingesta mismo id → upsert LanceDB + `skipped: already_indexed` **si** el get_by_id ya existía **antes** del upsert (comportamiento observable equivalente al actual, sobre tabla).

Tests E2E: tempdir con `cumulo.paths.json` mínimo; ingest; drop; reopen; `get_event_by_id`. Afirmar ausencia de `{id}.json` nuevo bajo `evolution/`.

### 5.1 Importador legado

`import_legacy_evolution_json(src_dir, adapter)`: lee `*.json` (no `.tmp`), mapea a `EvolutionEvent`, embed si `embedding` null/ceros, `store_event`. Idempotente por `id`. No se invoca desde ingest. CLI/test only en este PR (`#[cfg(test)]` + fn pública del crate adaptador).

## 6. Workspace y CI

`SddIA/Cargo.toml` members += tres crates. `execute-process/Cargo.toml`:

```toml
sddia-core-memory = { path = "../../core/memory" }
sddia-infrastructure-lancedb-evolution = { path = "../../infrastructure/adapters/lancedb_evolution_repo" }
tokio = { version = "1", features = ["rt-multi-thread"] }  # solo si el binario abre el adapter; preferible que el runtime viva entero en el adaptador
```

Preferible: **cero Tokio en execute-process**; el adaptador encapsula el runtime.

CI (`.github/workflows/sddia-index-qa.yml` job `sddia-index-integrity`):

```text
sudo apt-get update && sudo apt-get install -y protobuf-compiler
cargo test -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution
cargo test -p execute-process -- memory_evolution_ingest
```

Mismo `protobuf-compiler` en cualquier job que ejecute `cargo build --workspace` (el member LanceDB entra al grafo nativo).

Sin red. `CARGO_NET_OFFLINE` no se exige si el cache de Actions ya tiene registry (deps se resuelven en el job de build previo). Tests no tocan `.SddIA/vector_store/` del checkout.

WASI job: sin cambio (script no incluye adapters).

## 7. Fichas adaptador

Bump `1.0.0` → `1.1.0`, `status: active`. Sincronizar `index.md`. Cuerpo: frontera host, URI `{vectorStore}/lancedb`, tablas, no-WASI.

## 8. Tests mínimos (nombres canónicos PBI)

| Test | Crate |
|------|-------|
| `thought_roundtrip_after_reopen` | lancedb-thought |
| `thought_children_filtered_by_parent` | lancedb-thought |
| `thought_knn_orders_known_vectors` | lancedb-thought |
| `evolution_roundtrip_after_reopen` | lancedb-evolution |
| `duplicate_ids_are_idempotent` | lancedb-evolution |
| `wrong_vector_dimension_is_rejected` | ambos / core |
| `schema_mismatch_is_rejected` | evolution o thought |
| `embedding_is_nonzero_nonconstant_and_repeatable` | sddia-core-memory |
| `memory_evolution_ingest_persists_to_lancedb` | execute-process |
| `json_fallback_is_not_used` | execute-process |

## 9. Evolution

Registro nuevo `{uuid}.md` contrato 1.1.2 vía proceso autorizado (`sddia-evolution-register` / rehash). `relacionado`: PBI uuid `3c8b1a72-6e4f-4d90-a2c5-7f1e8b3d9a46`, crates, clave Cúmulo. `gate-evolution --range` antes de push.

## 10. Prohibiciones

- Placeholder `Ok(())` / `Ok(None)` / `Ok(vec![])` en adaptadores.
- Comentarios «integración futura».
- JSON como fallback o segunda SSOT post-activación.
- Truncar/pad embeddings.
- Runtime Tokio por operación.
- Nombres de proyecto cliente en Core.
- Mutar store del operador en tests.
- `entity-manager` sobre adapters (no aplica).
- MiniLM con descarga en CI.
