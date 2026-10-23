---
feature_name: argos-validation-on-main
created: "2026-10-23"
process: bug-fix
branch_name: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
---

# Implementación

`invoke_agent_phase` en `agent_runtime.rs`, solo si `phase_name == "Verificación"` y el runtime de agentes va a spawn:

1. `branch_name` vacío (también si solo venía `pr_branch` vacío) o `execution_id` ausente → `verification-handoff-incomplete`. No se lanza el CLI.
2. Ref local ausente (`.git` presente, `branch_list` sin esa rama) → `checkout` vía `git-manager`. Error → `verification-branch-checkout` y no hay spawn.
3. Árbol sin `.git` (tests de payload) no llama a git.

El allowlist de `workspace_init` no se toca.
