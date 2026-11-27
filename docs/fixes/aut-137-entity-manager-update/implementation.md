---
feature_name: aut-137-entity-manager-update
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-137-entity-manager-update
persist_ref: docs/fixes/aut-137-entity-manager-update
tracker_ref: AUT-137
execution_id: "727f8de1-4875-4cb3-a7e7-f11cf64cc9a0"
---

# Implementación — AUT-137

## Superficies

- `SddIA/engine/execute-process/src/forges/common.rs` — BOM, contract resolve, migración UUID, tests.
- `SddIA/engine/execute-process/src/forges/factory.rs` — hook `entity_uuid_migrate` en update de acción.
- `SddIA/engine/execute-process/src/engine/entity_manager.rs` — suite `markdown_body_replacements`, seed `entity_uuid_migrate`.
- `SddIA/engine/execute-process/src/engine/eda_coverage.rs` — `transfer_entity_coverage`.

## Evolución

`SddIA/evolution/4393ec29-62aa-43ee-9fc4-4d6d546437a8.md`.