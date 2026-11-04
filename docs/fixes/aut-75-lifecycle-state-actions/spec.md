---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
process: bug-fix
phase: design
agents: dedalo
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
document_id: PBI-38a1cf1a
uuid: bd2a24b3-9800-46e5-9724-303359242323
parent: AUT-20
execution_id: "e7138ac5-71fe-4407-b025-ab1eca978f35"
status: dedalo_locked
---

# Spec — aut-75-lifecycle-state-actions

## 1. Misión

Kaizen **AUT-75** (historia **AUT-20**): cada hito del ciclo escribe estado y metadatos del ítem como acción de la fase. El suscriptor `tracker-stamp` deja de ser el writer de esos hitos.

## 2. Problema verificado

| Hecho | Evidencia |
| --- | --- |
| Refino remoto ya sella Todo en la fase | `seal_refined_todo_at_origin`; el suscriptor hace no-op en `PBI_Refined` / `HU_Refined` |
| In Progress, In Validation, In Review y Done los escribe el suscriptor al consumir el evento | `stamp_event` → `Work_Initiated`, `Validation_Started`, `PullRequest_Presented`, `PullRequest_Merged` |
| In Validation no sale en relevo IDE | `maybe_emit_validation_started_after_tekton_agent` exige `status: executed`; Dedalo `simulated` levanta barrera y Ejecución queda `skipped` |
| No existe estado Validated | `state_map` en `.SddIA/project.md` no tiene clave `validated`; columnas en `item_lifecycle_frontmatter` tampoco |
| CI rojo no devuelve el ítem a In Progress | el sello de cierre conocido es `PullRequest_Merged` → Done |
| La auditoría 1.2.0 describe el bus como quien mueve el estado | `docs/audits/ciclo-vida-software-sddia.md` |
| Testigo | AUT-74: In Progress reparado; sin In Validation ni In Review pese a PR #434 |

## 3. Laudos

| Ref | Decisión |
| --- | --- |
| **L-ACTION** | Un solo writer de estado de ciclo, invocado por la fase. Hitos: `forge_started`, `forge_closed`, `validation_apto`, `validation_no_apto`, `pr_opened`, `review_ci_green`, `review_ci_red`. El refino sigue en `seal_refined_todo_at_origin`. |
| **L-EVENT** | El evento puede emitirse. `tracker-stamp` no cambia estado en esos hitos (no-op «sellado en origen», mismo patrón que el refino remoto). El sumidero file escribe la columna desde la acción, no desde el evento. |
| **L-FORGE-CLOSE** | In Validation cuando existen `implementation.md` y `execution.md` en `persist_ref`, aunque Ejecución haya quedado `simulated` o `skipped`. La entrada de Verificación aplica `forge_closed` antes de Argos. |
| **L-ARGOS** | `validacion.md` con `global: APTO` → Validated. Cualquier otro global cerrado → In Progress y `validated_status: no apto`. |
| **L-PR** | La fase de `delivery-close-cycle` que abre el PR aplica `pr_opened` (In Review) en esa fase. |
| **L-CI** | El proceso que ya triajea el PR aplica `review_ci_green` (Done) o `review_ci_red` (In Progress) con una lectura del check, sin bucle de espera. El merge que hoy sella Done por `PullRequest_Merged` deja de ser el writer. |
| **L-META** | Metadatos en frontmatter de la descripción. No renombrar `refined`. Campos nuevos: `forge_date`, `forge_by`, `validated_status`, `validated_date`, `validated_by`, `in_review_date`, `in_review_by`, `done_status`, `done_date`, `done_by`. |
| **L-STATE** | Clave `validated` → nombre Linear `Validated` en `state_map`, más constante de columna. El estado de workflow lo da de alta el operador en el equipo AUT. Si falta, la acción falla en claro en el informe de fase. |
| **L-DOC** | `docs/audits/ciclo-vida-software-sddia.md` pasa a versión nueva. El cuerpo dice que la fase escribe el estado. |
| **L-GENOME** | Si hay que cambiar el texto de `bug-fix`, `feature` o `refactorization`, la mutación va por `entity-manager`. El handler no se edita como sustituto de ese contrato cuando el contrato deba decir otra cosa. |
| **L-KAIZEN** | Sin `docs/todos/`. Sin reabrir AUT-74. Sin merge del PR #434. |
| **L-GIT** | Mutaciones git en `Fix_Events_Multiple_03`. |

## 4. Criterios de aceptación

1. Con `implementation.md` y `execution.md` presentes y la fase Ejecución en `skipped` o `simulated`, la entrada de Verificación deja el ítem en In Validation y escribe `forge_date` y `forge_by`.
2. `validacion.md` `global: APTO` deja el ítem en Validated con `validated_status: apto`. Un global no apto lo devuelve a In Progress con `validated_status: no apto`.
3. La apertura de PR en la fase de cierre deja el ítem en In Review con fecha y agente.
4. Check verde → Done y `done_status: ok`. Check rojo → In Progress y `done_status: false`.
5. Tras la acción de fase, un `Work_Initiated`, `Validation_Started`, `PullRequest_Presented` o `PullRequest_Merged` consumido por `tracker-stamp` no vuelve a cambiar el estado.
6. Test `aut75_*` en `execute-process` cubre forja cerrada sin status `executed`, Argos APTO / no APTO, y no-op del suscriptor.
7. `ciclo-vida-software-sddia.md` en versión nueva describe este mapa, In Validation y Validated.
8. `validacion.md` APTO en `Fix_Events_Multiple_03` con `pbi_ref: AUT-75`. Issue en Done tras el merge del PR único.

## 5. Fuera de alcance

- Defecto MCP `target_path` de AUT-74 y merge del PR #434.
- Alta de estados de workflow Linear desde el motor.
- Renombrar la clave `refined`.
- Reprocesar históricos del bus.
- Ficheros nuevos en `docs/todos/`.
