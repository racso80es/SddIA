---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
base: main
scope: pref-store-lancedb-migration
version_spec: "1.0.0"
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
---

# Especificación — pref-store-lancedb-migration

## 1. Contratos vigentes

| Pieza | Hecho |
|-------|--------|
| `user-preference-core` | Modelo + funciones libres JSON; crate WASI-safe |
| `spec.md` memoria-preferencias | Trait declarado, no implementado; `embedding` documental |
| Adaptadores | thought/evolution host, `lancedb = "=0.37.1"`, URI `{vectorStore}/lancedb/` |
| `execute-process` | `lancedb_uri(repo)`; ingest evolution ya físico |
| Skill `user-preference-store` | JSON; DA-2; este ciclo no la toca |
| `adapters-contract` v1.0.0 | ficha `{name}.md` + `index.md`; no `entity-manager` |

## 2. Hexágono

```text
user-preference-core          ← modelo, QuerySpec, trait, JsonUserPreferenceStore, fachada libre
infrastructure/adapters/lancedb_preferences_repo
                              ← connect, schema Arrow, merge_insert, KNN, migrate
execute-process handlers/user_preference.rs
                              ← interruptor table_exists; dual-write si tabla
email_triage.rs               ← sin LanceDB; regresión
```

Prohibido `lancedb` / `tokio` / `arrow` en `user-preference-core`.

### 2.1 Trait

```text
put_revision(pref) -> Result<UserPreference>
get_active(preference_id) -> Result<Option<UserPreference>>  // excluye revoked/superseded
query(spec) -> Result<Vec<UserPreference>>
query_context_block(spec) -> Result<Value>
purge_preference(preference_id) -> Result<()>
```

`Error: Display`. JSON usa `String`. LanceDB mapea `MemoryStoreError` → `String` en el bound del trait **o** expone `MemoryStoreError` como `Error` del adaptador (el trait es genérico).

Fachada: `put_revision(repo, pref)` delega en JSON siempre; si `user_preferences_table_exists(lancedb_uri(repo))`, replica al adaptador. Tests sin directorio LanceDB = path actual.

### 2.2 QuerySpec

Campos actuales + `query_embedding: Option<Vec<f32>>` + `query_text: Option<String>` (`#[serde(default)]`). JSON ignora vectoriales. LanceDB: si hay embedding/text, KNN post-filtro; si no, filtro SQL equivalente al JSON.

### 2.3 Embedding en dominio

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub embedding: Option<Vec<f32>>,
```

JSON no genera vectores. Adaptador genera si falta.

## 3. Persistencia LanceDB

### 3.1 URI y tabla

| Clave | Valor |
|-------|--------|
| URI | `{repo}/{paths.vectorStore}/lancedb/` (`lancedb_uri`) |
| Tabla | `user_preferences` |
| Clave merge | `revision_id` |

`open(path)` crea tabla si falta (migrate / tests adapter). `table_exists(path)` lista nombres **sin** crear.

### 3.2 Schema

Columnas del PBI v1.2.0 §3.2: identidad + value JSON + status/authority/sensitivity + opcionales Utf8 + `embedding` FixedSizeList 384 no nulo + `embedding_model` / `embedding_dim` / `embedding_norm`.

Enums persistidos en snake_case serde (`explicit_user`, `active`, `channel`, …). `Debug` de Rust **prohibido** como canon.

### 3.3 Upsert / query / KNN / purge

- `merge_insert(&["revision_id"])` matched update all / not-matched insert all.
- Prefiltro status como PBI §3.2.
- `nearest_to` + `only_if` + `limit`.
- `validate_embedding_dim`; dim ≠ 384 → `DimensionMismatch`.
- Texto: `{subject_kind}:{predicate}:{scope_type}:{to_string(value)}`.
- Purge: borrar fila(s) de ese `preference_id` si tabla existe; JSON `purge_preference` sigue siendo obligatorio.

### 3.4 Migración

`migrate_json_to_lancedb(repo) -> Result<usize>`: lee JSON store, upsert cada head revision, cuenta filas escritas. Idempotente. No borra JSON.

## 4. Resolución runtime

```text
query_context_block_with_capsule_fallback:
  1. cápsula QUERY_CONTEXT si success (vigente)
  2. si table_exists → LanceDB query_context_block; Err → {schema_version, preferences:[]}
  3. else JSON query_context_block
```

Paso 1 se conserva. Si la cápsula responde, no se exige LanceDB (la cápsula es JSON). Para tests de handler email, el path JSON permanece.

Dual-write en fachada `put_revision` / `purge_preference` del crate **no** puede depender de `execute-process`. Opciones:

A. Dual-write solo en `user_preference.rs` (`run_capsule` / ingest) + función `put_revision` JSON pura.
B. `user-preference-core` recibe un hook opcional — rechazo: contaminaría el crate.

**Laudo D-WIRE:** A. Dual-write y lectura LanceDB viven en `execute-process` (`user_preference.rs`). `user-preference-core::put_revision` permanece JSON. Tests `email_triage` que llaman `user_preference_core::put_revision` no tocan LanceDB. Ingest host (`run_capsule` del handler / `put_revision` reexportado) debe wrappear: JSON + replica si tabla.

El handler ya hace `pub use user_preference_core::*`, que reexporta `put_revision`. Tests de email importan `user_preference_core::put_revision` directo — OK.

Ingest en `user_preference.rs` debe llamar una función local `put_revision_durable` (JSON + replica) en el camino de producción, no solo la libre.

`query` de producción del handler: interruptor tabla.

## 5. Gobernanza adaptador

| Campo | Valor |
|-------|--------|
| id / name | `lancedb-preferences-repo` |
| uuid | `4c0103c3-5683-4219-aafd-1370c2b63c69` |
| type | `infrastructure-adapter` |
| version | `1.0.0` |
| status | `active` |
| crate_name | `sddia-infrastructure-lancedb-preferences` |
| impl_dir | `lancedb_preferences_repo` |
| contract | `adapters-contract v1.0.0` |
| port | `UserPreferenceStore` |

Fila nueva en `index.md`. Member en `SddIA/Cargo.toml`. `execute-process` path-dep al crate.

## 6. Tests de aceptación

| ID | Verificación |
|----|----------------|
| CA-1 | serde JSON histórico sin `embedding` |
| CA-2 | impls JSON + LanceDB del trait |
| CA-3 | ficha + índice + crate |
| CA-4 | count_rows estable tras re-upsert |
| CA-5 | revoked en tabla, ausente en KNN |
| CA-6 | orden L2 conocidos, `max_results` |
| CA-7 | dim 8 rechazada |
| CA-8 | migrate; JSON intacto; query filtro paridad IDs/scope |
| CA-9 | `cargo test -p execute-process --lib -- email_triage` |
| CA-11 | `Cargo.toml` de `user-preference-core` sin lancedb/core-memory |

CA-10 = check GitHub del PR. No APTO documental sin `run_id`.
