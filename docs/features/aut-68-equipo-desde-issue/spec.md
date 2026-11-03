---
feature_name: aut-68-equipo-desde-issue
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-68-equipo-desde-issue
pbi_ref: AUT-68
execution_id: "plan-aut-68-20261103"
---

# Spec — aut-68-equipo-desde-issue

## Resolución de equipo

| Operación | Equipo |
| --- | --- |
| `update_issue_state`, comentarios, labels sobre `issue_ref` | `issue.team.key` del fetch |
| `create_issue` con `parent` / padre resoluble | equipo del padre |
| `create_issue` sin padre | `team_key` de la petición (manifiesto) |

`team_key` entrante no sustituye al equipo del issue cuando `issue_ref` existe.

## Match

`state_name` del `state_map` y cada nombre de `tracker.labels` se comparan con los nombres Linear del equipo resuelto mediante igualdad insensible a mayúsculas. El id aplicado es el del equipo del issue.

## Error

Si ninguna label del equipo coincide:

`LINEAR_LABEL_UNKNOWN` con mensaje que incluye la clave del equipo y el nombre buscado (ejemplo: equipo `MAN`, nombre `kaizen`).

## Aceptación

1. Transición de un issue MAN al nombre de `state_map` (`Todo`, `In Progress`, …) sin que `team_key` sea `MAN`.
2. Alta o sello con label `pbi` encuentra `Pbi` en AUT.
3. Petición de `kaizen` sobre un issue cuyo equipo no tiene esa label falla nombrando el equipo, sin mutar el manifiesto.
