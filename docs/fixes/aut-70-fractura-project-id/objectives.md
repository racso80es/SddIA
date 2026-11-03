---

feature_name: aut-70-fractura-project-id
created: "2026-11-03"
process: bug-fix
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/fixes/aut-70-fractura-project-id
pbi_ref: AUT-70
execution_id: "plan-aut-70-20261103"
---
# Objetivos — aut-70-fractura-project-id

## Misión

Una fractura con Linear y `project_id` deja de morir en `GRAPHQL_VALIDATION_FAILED` y el router no borra el fallo como kaizen cerrado.

## Alcance

- Query `list_issues` y mutaciones `UpdateProject` / `UpdateState` con tipos `ID!`.
- Acuse de `route-domain-event` cuando el nativo devuelve `Err`.
- Barrido que no trate el fallo de materialización como `kaizen-finalized` sin PBI.
