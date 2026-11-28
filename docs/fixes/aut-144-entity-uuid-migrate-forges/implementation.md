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

- `sync_index_entity_uuid`, `with_paths_rollback`, `apply_entity_uuid_migrate`, `merge_migration_handoff`, `index_archivo_md_match`.
- Call sites migrate en `run_{process,event,agent,skill,suite}_forge` (+ paridad action con `with_paths_rollback` artefacto+índice).

## Tests añadidos

- `process_forge_entity_uuid_migrate_syncs_index_and_coverage`
- `event_forge_entity_uuid_migrate_syncs_index_and_coverage`
- `agent_forge_entity_uuid_migrate_syncs_index_and_coverage`
- `suite_forge_migrate_rolls_back_artifact_on_index_sync_failure`
- `migrate_entity_uuid_record_rejects_non_v4`

## AC-5 (alcance de entrega)

Remediación genómica de 20 paths + `fail-fast-lab` delegada a **AUT-133** (sellado en Linear). Este PR entrega solo el canal atómico y regresiones `execute-process`.