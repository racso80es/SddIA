---
feature_name: aut-68-equipo-desde-issue
created: "2026-11-03"
phases: equipo, match, error, tests
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-68-equipo-desde-issue
pbi_ref: AUT-68
execution_id: "plan-aut-68-20261103"
---

# Plan — aut-68-equipo-desde-issue

## T1 — Equipo del issue

En `linear-tracker-adapter`, si hay `issue_ref`, leer `team.key` del issue y usarlo en `resolve_workflow_state_id` y `resolve_label_ids`. En `create_issue` con padre, usar el equipo del padre. `team_key` de entrada solo si no hay issue ni padre.

## T2 — Match insensible

Comparar nombres de estado y de etiqueta sin distinguir mayúsculas dentro del equipo resuelto.

## T3 — Error nombrado

`LINEAR_LABEL_UNKNOWN` incluye clave de equipo y nombre buscado. Sin alta implícita de la label.

## T4 — Tests

- Mock: `update_issue_state` de un issue cuyo equipo no es el `team_key` de la petición resuelve el estado en el equipo del issue.
- Mock: label `pbi` casa con `Pbi`.
- Mock: label ausente devuelve código y mensaje con equipo y nombre.

## Orden

T1 antes de T2. T3 con T2. T4 cierra.
