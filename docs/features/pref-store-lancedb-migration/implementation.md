---
feature_name: pref-store-lancedb-migration
created: "2026-09-08"
process: feature
items:
  - L1-domain-trait
  - L2-lancedb-adapter
  - L3-migrate-parity
  - L4-execute-process-wire
branch_name: feat/pref-store-lancedb-migration
persist_ref: docs/features/pref-store-lancedb-migration
document_id: PBI-PREF-STORE-LANCEDB-MIGRATION
uuid: "c79e6f1a-821b-4d7a-9a84-0b1e32d56a77"
execution_id: "338b68e4-de96-4303-93a7-d0c0e40acb9c"
---

# Implementación — pref-store-lancedb-migration

## Touchpoints

| Artefacto | Ruta |
|-----------|------|
| Dominio | `SddIA/user-preference-core/src/lib.rs` |
| Adaptador | `SddIA/infrastructure/adapters/lancedb_preferences_repo/` |
| Ficha | `SddIA/infrastructure/adapters/lancedb-preferences-repo.md` |
| Censo | `SddIA/infrastructure/adapters/index.md` |
| Workspace | `SddIA/Cargo.toml` |
| Orquestador | `SddIA/engine/execute-process/src/engine/handlers/user_preference.rs` |
| Dep host | `SddIA/engine/execute-process/Cargo.toml` |

## Contratos

- Pin `lancedb = "=0.37.1"`. Tabla `user_preferences`. URI `{vectorStore}/lancedb/`.
- JSON = SSOT escritura. Interruptor = `table_exists`.
- KNN: `nearest_to` + filtro tombstone en memoria (SQL `only_if` + KNN no devolvió vecinos completos en 0.37.1).
- `finalize_preference_ids` compartido JSON/LanceDB (merge_insert por `revision_id` vacío colapsaba filas).
- Skill WASI no mutada. `user-preference-core` sin `lancedb` ni `sddia-core-memory`.

## Tests locales

```text
cargo test -p user-preference-core --lib
# 9 passed
cargo test -p sddia-infrastructure-lancedb-preferences --lib
# 7 passed
cargo test -p execute-process --lib -- email_triage user_preference
# 30 passed
```
