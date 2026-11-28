---
feature_name: aut-144-entity-uuid-migrate-forges
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-144-entity-uuid-migrate-forges
persist_ref: docs/fixes/aut-144-entity-uuid-migrate-forges
tracker_ref: AUT-144
execution_id: "d0b4e016-3133-4fdd-ba51-38cd393156cc"
---

# Implementación — AUT-144

## Motor

- `sync_index_entity_uuid` (layout parametrizado) + `with_paths_rollback` para artefacto + índice.
- `migrate_entity_uuid_record` acepta `match_col` / `match_value`; `sync_action_index_entity_uuid` delega en el helper genérico.
- `apply_entity_uuid_migrate` + `merge_migration_handoff` + `index_archivo_md_match`.
- Call sites update en `run_{process,event,agent,skill,suite}_forge` con `with_paths_rollback` y paridad semántica action (AUT-137).

## Tests

- Regresión `migrate_entity_uuid_syncs_index_and_coverage` (action).
- `sync_index_entity_uuid_skills_archivo_fuente_layout`.
- `suite_forge_entity_uuid_migrate_syncs_index_and_coverage` (forge integration).

## Pendiente alcance AC-5

Remediación masiva de los 20 paths + alineación `fail-fast-lab` en genoma: siguiente tanda vía `entity-manager` una vez mergeado el canal (no incluida en este commit de motor).