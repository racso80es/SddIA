---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
purpose: Estabilización Mayeuta — PBI-PREF-STORE-LANCEDB-MIGRATION v1.2.0
branch_name: feat/pref-store-lancedb-migration
persist_ref: docs/features/pref-store-lancedb-migration
pbi_ref: docs/todos/pending/[ARQUITECTURA] Migración del Store de Preferencias de Usuario a LanceDB.md
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
mayeuta_verdict: ok
laudo: host-nativo-lancedb-hashing-embed-ssot-vectorstore
---

# Clarificación — pref-store-lancedb-migration

Transcript Mayeuta. Semilla: PBI v1.2.0. Init lab `execution_id` `338b68e4-de96-4303-93a7-d0c0e40acb9c`. Filtro A contra genoma vigente.

---

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 |
| `feature_name` | `pref-store-lancedb-migration` |
| Rama | `feat/pref-store-lancedb-migration` |
| `persist_ref` | `docs/features/pref-store-lancedb-migration` |
| `document_id` | `PBI-PREF-STORE-LANCEDB-MIGRATION` |
| Init | `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1` |
| Stop planning | clarify + objectives + spec + plan + commit |

Toll: un `persist_ref`, un PR.

---

## D1 — Estado físico (Filtro A)

| Hecho | Evidencia |
|-------|-----------|
| `UserPreference` 16 campos, sin `embedding` | `user-preference-core/src/lib.rs` L39–L56 |
| Funciones libres; no trait; no `get_active` | mismo crate |
| `QuerySpec` sin campos vectoriales | L58–L66 |
| JSON store bajo `paths.userPreferencesStore` | `store_root` + `cumulo.paths.json` |
| Skill WASI sobre el crate | `SddIA/skills/user-preference-store.md` |
| Chokepoint lectura | `query_context_block_with_capsule_fallback` en `user_preference.rs` |
| `email_triage` no abre LanceDB | usa fallback + `put_revision` en tests |
| Test exención | `p_exempt_requires_explicit_active_high` (no `p_exempt_c`) |
| Adaptadores LanceDB existentes | thought `thought_graph_collection`, evolution `evolution`, pin `0.37.1` |
| `execute-process` ya depende de evolution adapter | `Cargo.toml` + `lancedb_uri()` |
| `A-HOST-NATIVE-LANCEDB` no existe como laudo | solo en PBI v1.1.0; sustituido |

---

## D2 — Coexistencia vs migración SSOT

JSON ya tiene reapertura. Este ciclo **no** retira JSON como SSOT.

| Fase | Escritura | Lectura |
|------|-----------|---------|
| Sin tabla `user_preferences` | JSON | JSON (+ cápsula si compilada) |
| Tras `migrate_json_to_lancedb` | JSON fail-closed + LanceDB best-effort | LanceDB; error → contexto vacío |
| Futuro (fuera) | LanceDB SSOT | LanceDB |

Prohibido fallback JSON **stale** tras cutover de escritura. Aquí no hay cutover de escritura.

`open()` de hermanos crea la tabla si falta. El interruptor **no** puede llamar `open()`. Debe: `uri.exists()` y `table_names` contiene `user_preferences`, sin `create_empty_table`.

---

## D3 — Hexágono y WASI

- Trait en `user-preference-core` (desvío consciente de la ubicación `core/memory` de `L-ONTOLOGY-SPLIT`; ortogonalidad ThoughtNode).
- Adaptador host: Tokio `OnceLock` + `block_on`, igual que thought/evolution.
- Skill no se muta. Crate dominio sin `lancedb` / `sddia-core-memory`.
- Embeddings solo en adaptador (`LocalHashingEmbedder`, dim 384).

---

## D4 — Tombstone y KNN

`REVOKE` persiste fila. Query/KNN: `status != revoked AND status != superseded` (+ proposed según flag). `PURGE` borra JSON head + fila LanceDB si tabla existe.

KNN: tests con vectores sintéticos L2 (patrón `thought_knn_orders_known_vectors`). No recall vs hash de email.

Texto embedding: `{subject_kind}:{predicate}:{scope_snake}:{json_value}`. Sin addr, sin `subject_key`.

---

## D5 — Gobernanza

Ficha `lancedb-preferences-repo.md` uuid `4c0103c3-5683-4219-aafd-1370c2b63c69`. Censo `index.md` en el mismo PR. Familia adapters ∉ DA-2.

Evolution vía `sddia-qa evolution-register`. `validacion.md` `global: APTO` solo con CA-CI `run_id` verde.
