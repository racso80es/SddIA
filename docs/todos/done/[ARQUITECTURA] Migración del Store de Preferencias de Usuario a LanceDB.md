---
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
title: "[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB"
format: markdown
version: "1.2.0"
created: "2026-09-06"
updated: "2026-09-08"
status: "done"
refinement_status: refined
priority: baja
type: architecture
process: feature
dispatch: false
suggested_branch: feat/pref-store-lancedb-migration
persist_ref_suggested: docs/features/pref-store-lancedb-migration
depends_on: []
blocks_on: []
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
architectural_constraints:
  - host-nativo-lancedb-hashing-embed-ssot-vectorstore
  - adapters-contract v1.0.0
  - L-ONTOLOGY-SPLIT
  - L-FAIL-POLICY
  - L-TOMBSTONE
  - L-CUMULO-PATH
  - L-NO-DLT-VALUE
related:
  - SddIA/user-preference-core/
  - SddIA/infrastructure/adapters/adapters-contract.md
  - SddIA/infrastructure/adapters/index.md
  - SddIA/infrastructure/adapters/lancedb_evolution_repo/
  - SddIA/infrastructure/adapters/lancedb_thought_repo/
  - SddIA/core/memory/
  - SddIA/skills/user-preference-store.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/handlers/user_preference.rs
  - docs/features/memoria-preferencias-usuario/spec.md
  - docs/features/lancedb-real-vector-memory/
---

# [ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB

## 1. Origen y Justificación

Deuda técnica originada durante el refinamiento de `PBI-EMAIL-TRIAGE-HEURISTIC` (cold-start; PBI archivado en `docs/todos/done/`) y anticipada en `docs/features/memoria-preferencias-usuario/spec.md` §4 (*«LanceDB opcional después de que el path JSON pase reapertura»*). El path JSON ya tiene tests de reapertura (`put_and_reopen_active_preference`). Este PBI materializa el adaptador LanceDB.

El adaptador MVP (`SddIA/user-preference-core/`) persiste revisiones en ficheros JSON bajo `paths.userPreferencesStore` (`.SddIA/vector_store/user_preferences/revisions/*.json` + `head_index.json`). Limitaciones estructurales reales:

1. **Escaneo lineal:** `query` / `query_context_block` recorren `head_index.json` y leen un fichero por revisión head.
2. **Sin KNN:** el triaje actual filtra por `subject_key` (SHA-256 del addr). No hay ranking por proximidad. Este ciclo **no** añade reglas semánticas de triaje; solo el puerto y tests de ordenación vectorial.
3. **Sin puerto hexagonal en código:** `memoria-preferencias-usuario/spec.md` §4 declara el trait; `user-preference-core` expone funciones libres (`put_revision`, `purge_preference`, `query`, `query_context_block`, `run_capsule`). No existe `get_active`.

---

## 2. Fe de Erratas

### 2.1 v1.0.0 → v1.1.0 (conservadas)

| Elemento previo | Inexactitud | Realidad en el repositorio | Corrección |
|---|---|---|---|
| Modelo | *«campo `embedding` ya existe»* | `UserPreference` tiene 16 campos; **no** hay `embedding`. Solo figuraba en `spec.md` §3. | Añadir `embedding: Option<Vec<f32>>` con `#[serde(default, skip_serializing_if = "Option::is_none")]`. |
| Trait | *«trait existente con put/get_active/query/purge»* | Funciones libres. No hay trait ni `get_active`. | Trait `UserPreferenceStore` + `JsonUserPreferenceStore` + adaptador LanceDB. `get_active` proviene de `spec.md` §4, no del código. |
| Tabla thought | *«tabla thought»* | `TABLE_THOUGHT = "thought_graph_collection"`. | Tabla de preferencias: nombre único (ver §3.2). |
| CA KNN | *«recall ≥ 80% vs filtro duro»* | Dominios disjuntos (hash vs vector). | Fixtures de distancia + `EMBEDDING_DIM = 384` + prefiltro tombstone. |
| Gobernanza | Omitida | `adapters-contract.md` v1.0.0 exige ficha YAML + censo `index.md`. | Ficha + fila en índice. |
| WASI | Omitida | LanceDB no compila en `wasm32-wasip1`. | Dominio puro en `user-preference-core`; adaptador host nativo. |

### 2.2 v1.1.0 → v1.2.0 (este refinamiento)

| Elemento v1.1.0 | Inexactitud / incoherencia | Realidad | Corrección v1.2.0 |
|---|---|---|---|
| `A-HOST-NATIVE-LANCEDB` | ID de laudo inventado; no existe en evolution ni en `spec.md` de preferencias. | El laudo real es `host-nativo-lancedb-hashing-embed-ssot-vectorstore` (`docs/features/lancedb-real-vector-memory/clarify.md`). | Sustituir el ID ficticio por ese laudo derivado. |
| `L-FAIL-POLICY` = fallback JSON si LanceDB cae | Distorsión. El laudo original: **escritura fail-closed**; **consulta fail-open → bloque vacío** (nunca «permitir todo»). | `query_context_block` ya devuelve `{preferences:[]}` si `query` err. El fallback cápsula→JSON en `user_preference.rs` es otro mecanismo (cápsula ausente). | Dual-run: JSON sigue siendo SSOT de escritura. LanceDB es réplica + KNN. Fallback JSON en lectura **solo** si la tabla no existe. Si la tabla existe y LanceDB falla: contexto vacío (`L-FAIL-POLICY`), no «permitir todo». |
| «JSON inalterado» + cutover de escritura a LanceDB | Si se deja de escribir JSON y se hace fallback de lectura a JSON, el fallback sirve datos **stale**. | `put_revision` es el chokepoint de tests y handlers. | Presencia de tabla = interruptor. Sin tabla: JSON only (tests actuales intactos). Con tabla: dual-write JSON+LanceDB; lectura prefiere LanceDB. |
| Cutover en `email_triage.rs` **y** `user_preference.rs` | Redundante. | `email_triage.rs` ya llama `query_context_block_with_capsule_fallback`; tests usan `user_preference_core::put_revision`. Test real de exención: `p_exempt_requires_explicit_active_high`, no `p_exempt_c` (`p_exempt_c` es función privada). | Un chokepoint: `user_preference.rs` + funciones libres. `email_triage.rs` = batería de regresión, no cableado LanceDB. |
| «Ninguna revocada es indexada» vs CA-5 | Contradicción. | `L-TOMBSTONE`: `REVOKE` = revisión `status: revoked` que **permanece**. `PURGE` borra físico. El JSON ya escribe revoked en `head_index`. | Persistencia de todos los status. KNN/query aplican prefiltro SQL/lógico. No omitir filas revoked en `merge_insert`. |
| Tabla `preferences` **o** `user_preferences` | Ambiguo en catálogo compartido (`evolution`, `thought_graph_collection`). | Un `const` por adaptador hermano. | `TABLE_PREFERENCES = "user_preferences"`. |
| Schema sin `embedding_model` / `embedding_dim` / `embedding_norm` | Divergencia respecto a thought/evolution. | Ambos adaptadores persisten esas columnas; `EMBEDDING_MODEL = "sddia-local-hashing-v1"`, `EMBEDDING_NORM = "l2"`. | Añadir las tres columnas. `embedding` no nulo en tabla (se genera al upsert si falta). |
| Paridad «total» JSON vs LanceDB | KNN no existe en JSON. | `QuerySpec` actual: filtros duros. | CA-8 = paridad de **consultas por filtro**. KNN es superficie nueva (CA-6). |
| `user-preference-core` genera embeddings | Arrastraría `sddia-core-memory` al crate WASI de la skill. | `user-preference-core/Cargo.toml`: chrono, hex, serde, sha2. Skill `user-preference-store` es cápsula WASI. `execute-process` ya depende de `sddia-infrastructure-lancedb-evolution`. | Embeddings **solo** en el adaptador host. Trait y JSON store **sin** LanceDB ni `core/memory`. |
| Trait en `core/memory` (`L-ONTOLOGY-SPLIT`) | El laudo original ubicaba el puerto en memory Core. | El MVP vivió en `user-preference-core` (ortogonal a `ThoughtNode`). | Trait permanece en `user-preference-core`. No contaminar `ThoughtNode` / `EvolutionEvent`. |
| Texto de proyección `format!(... pref.scope_type ...)` | `ScopeType` no impl `Display`; `Debug` emite `Channel`, no `channel`. | Serde `rename_all = "snake_case"`. | Canon snake_case + `serde_json::to_string(&value)`. Nunca addr en claro ni `subject_key` (hash ruidoso para el embedder). |
| Skill como superficie de cutover | Implícito. | Cápsula WASI no puede abrir LanceDB. | Skill permanece JSON. Cutover host en `execute-process`. |
| `index.md` «mantenido por Cúmulo» | Cúmulo no muta el censo en este ciclo. | El ciclo LanceDB real actualizó ficha + `index.md` en el mismo PR. | Tekton actualiza ficha e índice en este PR (adapters ∉ DA-2). |
| Enlaces `file:///home/racso/...` | Ruta de host. | Viola ceguera espacial. | Rutas lógicas de repositorio. |

---

## 3. Arquitectura

### 3.1 Dominio (`user-preference-core`) — crate puro

1. **Campo nuevo** en `UserPreference`:

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub embedding: Option<Vec<f32>>,
```

JSON histórico sin clave `embedding` → `None`. El store JSON **no** calcula vectores.

2. **Trait** (alineado a `spec.md` §4; `purge_preference` conserva el nombre del código):

```rust
pub trait UserPreferenceStore: Send + Sync {
    type Error: std::fmt::Display;
    fn put_revision(&self, pref: UserPreference) -> Result<UserPreference, Self::Error>;
    fn get_active(&self, preference_id: &str) -> Result<Option<UserPreference>, Self::Error>;
    fn query(&self, spec: &QuerySpec) -> Result<Vec<UserPreference>, Self::Error>;
    fn query_context_block(&self, spec: &QuerySpec) -> Result<serde_json::Value, Self::Error>;
    fn purge_preference(&self, preference_id: &str) -> Result<(), Self::Error>;
}
```

`get_active`: head por `preference_id`; `None` si ausente, `revoked` o `superseded`. `proposed` se devuelve (el spec excluye solo tombstones).

3. **`JsonUserPreferenceStore`**: envuelve `repo: PathBuf`; delega en la lógica de ficheros actual. Las funciones libres (`put_revision(repo, …)`, etc.) permanecen como fachada para no romper `email_triage` tests.

4. **`QuerySpec`**: campos nuevos opcionales con default:

- `query_embedding: Option<Vec<f32>>`
- `query_text: Option<String>`

Sin esos campos, `query` es filtro duro (paridad JSON). Con ellos, solo el adaptador LanceDB ejecuta KNN **después** del prefiltro de status. El JSON store ignora los campos vectoriales (no alucina KNN).

### 3.2 Adaptador host (`sddia-infrastructure-lancedb-preferences`)

| Pieza | Valor |
|-------|--------|
| Directorio | `SddIA/infrastructure/adapters/lancedb_preferences_repo/` |
| Ficha | `SddIA/infrastructure/adapters/lancedb-preferences-repo.md` |
| `crate_name` | `sddia-infrastructure-lancedb-preferences` |
| `impl_dir` | `lancedb_preferences_repo` |
| Workspace | alta en `SddIA/Cargo.toml` `members` |
| URI | `{paths.vectorStore}/lancedb/` — helper ya existente `lancedb_uri` en `memory_evolution_ingest_core.rs` |
| Tabla | `user_preferences` |
| Runtime | un `tokio::Runtime` por `open()`, `block_on` en métodos sync del trait |
| Pin | `lancedb = "=0.37.1"` (paridad thought/evolution; no inventar 0.38) |

**Schema Arrow** (paridad de metadatos de embedding con hermanos):

| Columna | Tipo | Nulo |
|---------|------|------|
| `preference_id` | Utf8 | no |
| `revision_id` | Utf8 | no (clave `merge_insert`) |
| `subject_kind` | Utf8 | no |
| `subject_key` | Utf8 | no |
| `predicate` | Utf8 | no |
| `value` | Utf8 | no (JSON compacto) |
| `scope_type` | Utf8 | no (`global`\|`domain`\|`project`\|`channel`) |
| `scope_id` | Utf8 | sí |
| `status` | Utf8 | no |
| `authority` | Utf8 | no (`explicit_user`\|`inferred`) |
| `sensitivity` | Utf8 | no |
| `valid_from` / `valid_until` / `supersedes` | Utf8 | sí |
| `provenance` | Utf8 | no |
| `recorded_at` | Utf8 | no |
| `embedding` | `FixedSizeList<Float32, 384>` | no |
| `embedding_model` | Utf8 | no (`sddia-local-hashing-v1`) |
| `embedding_dim` | UInt16 | no (`384`) |
| `embedding_norm` | Utf8 | no (`l2`) |

Upsert: `merge_insert(&["revision_id"]).when_matched_update_all(None).when_not_matched_insert_all()`. Vector: `validate_embedding_dim`; si `embedding` ausente o vacío, `LocalHashingEmbedder.generate_embedding(canonical_text)`. Dim ≠ 384 → error, sin pad/truncado.

**Texto canónico de embedding** (sin PII en claro, sin `subject_key`):

```text
{subject_kind}:{predicate}:{scope_type_snake}:{json_compact_value}
```

`scope_type_snake` = serde (`global`, no `Debug`). `json_compact_value` = `serde_json::to_string(&pref.value)`.

**Prefiltro (L-TOMBSTONE):** toda query/KNN:

```text
status != 'revoked' AND status != 'superseded'
```

y `status != 'proposed'` si `include_proposed == false`. Las filas revoked **sí** se persisten.

**KNN:** `nearest_to` + `only_if(prefiltro)` + `limit(max_results)`. Métrica nativa L2 (euclidiana), misma que thought/evolution.

### 3.3 Fronteras de proceso

| Capa | Persistencia |
|------|----------------|
| Skill WASI `user-preference-store` | JSON (sin cambio de contrato). Mutar la skill exige `entity-manager` (DA-2); **este ciclo no la muta**. |
| `execute-process` (host) | Resolución: tabla `user_preferences` existe → LanceDB (lectura) + dual-write en `put_revision`/`purge`; si no existe → JSON only. |
| `email_triage.rs` | Sin imports LanceDB. Regresión vía chokepoint existente. |

`L-ONTOLOGY-SPLIT` se respeta en el sentido de «cero campos en `ThoughtNode`». Ubicación física del trait: `user-preference-core`, no `SddIA/core/memory/`.

---

## 4. Estrategia de coexistencia (no cutover SSOT)

Este ciclo **no** declara LanceDB como SSOT de escritura. JSON sigue siendo la fuente durable. LanceDB es réplica + superficie KNN.

1. **Backfill:** `migrate_json_to_lancedb(repo) -> Result<usize, Error>` lee `head_index.json` + `revisions/*.json`, genera embedding si falta, upsert idempotente. No borra JSON.
2. **Interruptor:** `table_exists(user_preferences)` bajo `{vectorStore}/lancedb/`.
3. **Escritura:** sin tabla → solo JSON. Con tabla → JSON (fail-closed) y LanceDB (si LanceDB falla, JSON ya persistió; paridad/tests detectan drift; no abortar el orquestador en query).
4. **Lectura:** sin tabla → JSON. Con tabla → LanceDB; error LanceDB → contexto vacío (`L-FAIL-POLICY`). Capsule fallback actual se conserva para `QUERY_CONTEXT` cuando la cápsula está compilada.
5. **Reversibilidad:** retirar la tabla restaura el path JSON. Un PBI futuro podrá convertir LanceDB en SSOT y deprecar JSON.

---

## 5. Contratos y consumidores

- Schemas `memory.pref_write` / `memory.pref_query`: `additionalProperties: true` en salida; `QuerySpec` añade campos opcionales; consumidores por `subject_key` no cambian.
- Triaje: cero reglas semánticas nuevas. CA-9 = tests existentes de `email_triage.rs` en verde.
- `L-NO-DLT-VALUE`: sin cambios de eventos. Embedding no viaja en ECST.

---

## 6. Slices

### Slice 1 — Dominio y puerto

- [ ] Campo `embedding` + test de deserialización histórica.
- [ ] Trait `UserPreferenceStore` + `JsonUserPreferenceStore`.
- [ ] `get_active`.
- [ ] `QuerySpec.query_embedding` / `query_text` (ignorados por JSON).
- [ ] Funciones libres intactas en firma (`repo: &Path`).

### Slice 2 — Adaptador y gobernanza

- [ ] Crate + ficha YAML (`uuid` v4 nuevo, `status: active`) + fila `index.md` + member workspace.
- [ ] Schema, `open`, `merge_insert`, filtro, KNN, dim check.
- [ ] Tests `tempfile`: reopen, idempotencia `revision_id`, `preferences_knn_orders_known_vectors`, tombstone omitido en KNN, dim rechazada.

### Slice 3 — Migración y paridad de filtro

- [ ] `migrate_json_to_lancedb`.
- [ ] Test: JSON intacto + mismos IDs/valores/orden de scope en `query` filtro vs LanceDB.

### Slice 4 — Resolución en `execute-process`

- [ ] Dependencia `sddia-infrastructure-lancedb-preferences`.
- [ ] Chokepoint en `user_preference.rs` (y fachada `put_revision` usada por ingest): interruptor por existencia de tabla. **No** cablear LanceDB dentro de `email_triage.rs`.
- [ ] `cargo test -p user-preference-core --lib`
- [ ] `cargo test -p sddia-infrastructure-lancedb-preferences --lib`
- [ ] `cargo test -p execute-process --lib -- email_triage`

---

## 7. Criterios de aceptación

- [ ] **CA-1** JSON sin `embedding` deserializa a `None`.
- [ ] **CA-2** Trait compilado; impl JSON + LanceDB.
- [ ] **CA-3** Ficha `lancedb-preferences-repo.md` conforme a `adapters-contract` v1.0.0; fila en `index.md`; `crate_name` / `impl_dir` coinciden.
- [ ] **CA-4** Re-upsert misma `revision_id` → `count_rows` inmutable.
- [ ] **CA-5** KNN omite `revoked` y `superseded` pese a vector cercano. Las filas tombstone **existen** en tabla.
- [ ] **CA-6** `preferences_knn_orders_known_vectors`: vecinos en orden L2 ascendente; respeta `max_results`.
- [ ] **CA-7** Vector dim ≠ 384 → error explícito (sin pad).
- [ ] **CA-8** Migración no borra JSON; paridad de **query por filtro** (IDs, values, precedencia de scope).
- [ ] **CA-9** `cargo test -p execute-process --lib -- email_triage` verde, incluidos `mute_active_closes_preference_without_llm` y `p_exempt_requires_explicit_active_high`.
- [ ] **CA-10 (CI)** Checks del PR verdes (`run_id`); `validacion.md` no declara `global: APTO` sin ello.
- [ ] **CA-11** Skill WASI no depende de LanceDB. `user-preference-core` no declara `lancedb` ni `sddia-core-memory`.

Fuera de alcance: reglas de triaje semántico; mutación EM de `user-preference-store`; convertir LanceDB en SSOT de escritura; recrypt at-rest (`DEUDA-PREF-CRYPTO`).
