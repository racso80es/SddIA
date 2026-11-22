---
feature_name: aut-98-active-executions-dirty-worktree
branch: feat/aut-98-active-executions-dirty-worktree
global: APTO
pbi_archived: false
pbi_ref: AUT-98
created: "2026-11-22"
checks:
  CA-1_path_in_scope_active_executions: APTO
  CA-2_gitignore_active_executions: APTO
  CA-3_regression_test_and_fracture_corpus: APTO
  CA-4_cargo_test_and_build_zero_warnings: APTO
git_changes:
  - .gitignore
  - SddIA/engine/execute-process/src/engine/workspace_init.rs
  - SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json
  - docs/features/aut-98-active-executions-dirty-worktree/clarify.md
  - docs/features/aut-98-active-executions-dirty-worktree/execution.md
  - docs/features/aut-98-active-executions-dirty-worktree/implementation.md
  - docs/features/aut-98-active-executions-dirty-worktree/objectives.md
  - docs/features/aut-98-active-executions-dirty-worktree/plan.md
  - docs/features/aut-98-active-executions-dirty-worktree/spec.md
  - docs/features/aut-98-active-executions-dirty-worktree/validacion.md
---

# Validación — aut-98-active-executions-dirty-worktree

## Veredicto global: APTO

| Criterio | Estado | Evidencia |
|----------|--------|-----------|
| **CA-1** | APTO | `path_in_scope` en `SddIA/engine/execute-process/src/engine/workspace_init.rs` exime `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`. |
| **CA-2** | APTO | `.gitignore` ignora `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`; `git status --short` ya no lista el fichero. |
| **CA-3** | APTO | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib workspace_init::tests` (`21 passed; 0 failed`) incluyendo `active_executions_registry_is_not_reported_as_dirty`, y fixture `5f46810a-workspace-init-active-executions.json` en `tests/fixtures/fracture-corpus/`. |
| **CA-4** | APTO | `cargo build --manifest-path SddIA/Cargo.toml -p execute-process` compilado sin warnings (`Finished dev profile`). |
