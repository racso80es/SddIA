---
feature_name: aut-65-work-initiated-fase1
created: "2026-11-03"
purpose: decision
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-65-work-initiated-fase1
pbi_ref: AUT-65
execution_id: "plan-aut-65-20261103"
---

# Clarificaciones — AUT-65

## Decisiones

1. El punto de ignición es el éxito de Fase 1 (`workspace-init`), no la entrada de Tekton.
2. Una sola emisión por ejecución. La fase Tekton no vuelve a emitir.
3. `SDDIA_LAB_SKIP_WORK_INITIATED` se conserva.
4. `tracker-stamp` no cambia la regla anti-retroceso de `in_progress`.
5. La entrega se planifica en `refactor/man-26-ciclo-vida-software`. No se abre otra rama.
6. Padre: MAN-26. El desfase que cierra es D2 de la HU de ciclo de vida.
