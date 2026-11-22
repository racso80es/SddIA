---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
process: feature
items_applied:
  - T1
  - T2
  - T3
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Registro de ejecución — aut-98-active-executions-dirty-worktree

- **T1:** Exención de `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp` en `SddIA/engine/execute-process/src/engine/workspace_init.rs` (`path_in_scope`) y `.gitignore`.
- **T2:** Añadido test unitario `active_executions_registry_is_not_reported_as_dirty` en `workspace_init.rs` y fixture `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json`.
- **T3:** Ejecutado `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib workspace_init::tests` (`21 passed; 0 failed`) y compilación release de `execute-process`.
