---

feature_name: aut-66-estado-in-validation
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-66-estado-in-validation
pbi_ref: AUT-66
execution_id: "plan-aut-66-20261103"
---
# Objetivos — AUT-66

## Misión

Que el tablero deje In Progress cuando Tekton termina y entre en validación (Argos y aduanas) sin esperar al PR.

## Inclusión

- `state_map` Core-self y estado de workflow Linear.
- Rango en `tracker_sync_replay::state_rank` (lo consume `tracker-stamp`).
- Emisión al éxito de Tekton en `feature`, `bug-fix`, `refactorization`.
- Sumidero file: `tracker_column: in_validation`.
- Auditoría `ciclo-vida-software-sddia.md` y contratos de proceso.

## Exclusión

- No crear carpetas `in_validation/`.
- No alterar Done, roll-up de HU, AUT-64 ni el punto de ignición de AUT-65.
- No mover `PullRequest_Presented` fuera de `in_review`.
