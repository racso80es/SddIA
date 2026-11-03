---
feature_name: aut-70-fractura-project-id
created: "2026-11-03"
phases: graphql, router, sweep, tests
process: bug-fix
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/fixes/aut-70-fractura-project-id
pbi_ref: AUT-70
execution_id: "plan-aut-70-20261103"
---

# Plan — aut-70-fractura-project-id

## T1 — Tipos ID

En `linear-tracker-adapter`, `list_issues` con `project_id` usa `$projectId: ID!`. `UpdateProject` y `UpdateState` usan `ID!` en `$id`, `$projectId` y `$stateId`. Recompilar el binario que resuelve el runtime.

## T2 — Acuse nativo

En el router de dominio, `Err` de `try_run_native` se publica en el acuse y no cae a `invoke_execute_action`.

## T3 — Barrido

Un fallo de `materialize-fracture-pbi` no se sella `kaizen-finalized` sin issue. El dead-letter permanece consultable.

## T4 — Tests

- Query de lista con proyecto contiene `ID!`.
- Mutaciones de proyecto y estado contienen `ID!`.
- Suscriptor nativo en error no relanza el action.
- Sweep de materialización fallida no marca kaizen finalizado.

## Orden

T1 antes del binario. T2 y T3 independientes. T4 cierra.
