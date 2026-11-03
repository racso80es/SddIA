---

feature_name: aut-65-work-initiated-fase1
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-65-work-initiated-fase1
pbi_ref: AUT-65
execution_id: "plan-aut-65-20261103"
---
# Objetivos — AUT-65

## Misión

Que el tablero marque In Progress en cuanto existen rama y `persist_ref`, antes de Mayeuta y Dédalo.

## Inclusión

- `feature`, `bug-fix`, `refactorization`.
- Payload `branch`, `persist_ref`, `tracker_ref` o `pbi_ref`.
- Contratos de proceso y `docs/audits/ciclo-vida-software-sddia.md` (§§ 2, 4.3, 6.3, 8).

## Exclusión

- No alterar el sello Done, el roll-up de HU ni AUT-64 (sellado en origen de forja/refino).
- No añadir `tracker-operations` a los procesos de entrega.
