---
feature_name: argos-validation-on-main
created: "2026-10-23"
process: bug-fix
branch_name: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
pbi_ref: docs/todos/pending/[FIX] Argos sella validacion en main y bloquea el init.md
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
---

# Especificación — Argos no sella validacion.md sin handoff

## Problema

La fase Verificación despachaba a Argos con `branch_name` o `execution_id` vacíos. El agente escribía `validacion.md` en el checkout actual (`main`) y el gate `dirty-worktree` abortaba el `workspace-init` siguiente.

## Cambio

| ID | Entrega |
|----|---------|
| FX-CA1 | Handoff de Verificación sin `branch` o sin `execution_id`: fase `failed` (`verification-handoff-incomplete`) y el runtime no arranca. |
| FX-CA2 | Con `branch` y ref local ausente, `skill:git-manager` `checkout` (`create_if_not_exists: false`) antes del spawn. Si el checkout falla, no hay spawn. |
| FX-CA4 | `dirty-worktree` no cambia de contrato. |

T20 y el manifiesto de Paciente 0 quedan fuera.
