---
feature_name: aut-70-fractura-project-id
created: "2026-11-03"
process: bug-fix
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/fixes/aut-70-fractura-project-id
pbi_ref: AUT-70
execution_id: "plan-aut-70-20261103"
---

# Spec — aut-70-fractura-project-id

## GraphQL

| Operación | Variables | Tipo |
| --- | --- | --- |
| `list_issues` con proyecto | `$projectId` | `ID!` |
| `UpdateProject` | `$id`, `$projectId` | `ID!` |
| `UpdateState` | `$id`, `$stateId` | `ID!` |

El runtime usa el binario recompilado. Un test afirma que la query de lista con proyecto contiene `$projectId: ID!`.

## Router

`dispatch_subscriber`: si `try_run_native` devuelve `Err`, el acuse lleva ese error y no se invoca `invoke_execute_action`.

## Barrido

Si `materialize-fracture-pbi` falla, `try_sweep_event` no marca `kaizen-finalized` ni purga el pendiente como cierre de kaizen. El dead-letter sigue legible. No se exige reemisión automática de `System_Fracture_Detected`.

## Aceptación

1. `list_issues` + `project_id` no produce validación GraphQL `String` vs `ID`.
2. Mutaciones de proyecto y estado declaran `ID!`.
3. Error nativo visible en el acuse, un solo camino.
4. Fallo de materialización distinto de kaizen finalizado.
