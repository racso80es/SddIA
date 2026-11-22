---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
process: feature
items:
  - path: SddIA/engine/execute-process/src/engine/workspace_init.rs
    action: update
    summary: Eximir .SddIA/state/active-executions.json y .tmp en path_in_scope + test de regresión
  - path: .gitignore
    action: update
    summary: Ignorar .SddIA/state/active-executions.json y .SddIA/state/active-executions.json.tmp
  - path: SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json
    action: create
    summary: Fixture fracture-corpus para traza 5f46810af5ec (AUT-98)
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Implementación — aut-98-active-executions-dirty-worktree

## Touchpoints

- `SddIA/engine/execute-process/src/engine/workspace_init.rs`:
  - `path_in_scope`: añadido soporte explícito para `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`.
  - `tests::active_executions_registry_is_not_reported_as_dirty`: verifica `path_in_scope` y `dirty_paths_outside_scope` tras `upsert_active_execution`.
- `.gitignore`:
  - Añadidas `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`.
- `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json`:
  - Registro del incidente `5f46810af5ec` en el corpus de fracturas.
