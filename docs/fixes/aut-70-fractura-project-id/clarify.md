---
feature_name: aut-70-fractura-project-id
created: "2026-11-03"
purpose: decision
process: bug-fix
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/fixes/aut-70-fractura-project-id
pbi_ref: AUT-70
execution_id: "plan-aut-70-20261103"
---

# Clarificaciones — AUT-70

## Decisiones

1. El binario que ejecuta el runtime es el de `SddIA/target` recompilado desde el fuente actual. No se parchea el binario del 2026-10-30 a mano.
2. `$projectId`, `$id` y `$stateId` de `list_issues`, `UpdateProject` y `UpdateState` se declaran `ID!`.
3. `Err` de un handler nativo de suscriptor se publica en el acuse y no dispara `invoke_execute_action`.
4. Un fallo de `materialize-fracture-pbi` no se marca `kaizen-finalized` sin issue de fractura. El dead-letter permanece consultable.
5. No se reinyectan las fracturas históricas en este plan. Rama: `refactor/man-26-ciclo-vida-software`.
