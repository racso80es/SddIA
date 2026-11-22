---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Clarificaciones y decisiones de diseño para el fix AUT-98 (dirty-worktree por active-executions.json)
branch_name: feat/aut-98-active-executions-dirty-worktree
persist_ref: docs/features/aut-98-active-executions-dirty-worktree
pbi_ref: AUT-98
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Clarificación — aut-98-active-executions-dirty-worktree

## Contexto de la fractura

- **Trace hash:** `5f46810af5ec`
- **Origen:** Al iniciar `MAN-48` con `./sddia-run.sh --process feature`, `executor::run_generic_phases` invocó `flush_active(...)` antes de entrar en la Fase 0 (`workspace_init`), escribiendo `.SddIA/state/active-executions.json` (`execution_registry.rs`).
- **Colisión:** `workspace_init::dirty_paths_outside_scope` consultó `git status` vía `git-manager` y rechazó `.SddIA/state/active-executions.json` porque no estaba en `.gitignore` ni en `path_in_scope(...)`.

## Decisiones adoptadas

1. **Doble defensa (`workspace_init.rs` + `.gitignore`):**
   - Eximir `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp` en `workspace_init::path_in_scope`.
   - Añadir ambas rutas a `.gitignore` junto a `.SddIA/state/tracker-outbound/`.
2. **Test de regresión y corpus:**
   - Añadir test unitario en `workspace_init.rs` validando `path_in_scope` y `dirty_paths_outside_scope` con `.SddIA/state/active-executions.json` presente.
   - Registrar el fixture `5f46810a-workspace-init-active-executions.json` en `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/`.
