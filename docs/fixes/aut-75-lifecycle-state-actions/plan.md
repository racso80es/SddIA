---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T8
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
document_id: PBI-38a1cf1a
uuid: bd2a24b3-9800-46e5-9724-303359242323
parent: AUT-20
execution_id: "e7138ac5-71fe-4407-b025-ab1eca978f35"
---

# Plan — aut-75-lifecycle-state-actions

Parada actual (**L0**): `objectives.md`, `clarify.md`, `spec.md`, `plan.md`. Commit de planificación antes de mutar código. Rama: `Fix_Events_Multiple_03`.

## T0 — Sellar planificación (esta entrega)

| Paso | Acción |
| --- | --- |
| T0.1 | Frontmatter coherente (`branch_name`, `pbi_ref`, `document_id`, `uuid`, `parent`). |
| T0.2 | Commit: `docs(AUT-75): cascada planificación acciones de estado` |
| T0.3 | Anotar en `execution.md` el refino `6e09811f-…` y el init `e7138ac5-…`. |

## T1 — Writer único

1. Extraer la aplicación de estado+metadatos a una función de fase (mismo módulo que `tracker_stamp` o hermano invocado por los handlers).
2. Hitos: `forge_started`, `forge_closed`, `validation_apto`, `validation_no_apto`, `pr_opened`, `review_ci_green`, `review_ci_red`.
3. Refino: no duplicar `seal_refined_todo_at_origin`.
4. Si `tracker_ref` está presente y el `state_map` no tiene la clave, el informe de fase lo dice. Prohibido no-op silencioso.

## T2 — Estado Validated

1. Constante de columna `validated` junto al resto en `item_lifecycle_frontmatter`.
2. `state_map.validated: Validated` en `.SddIA/project.md`.
3. Precondición de operador: el workflow Linear del equipo AUT tiene el estado `Validated`. Sin eso T1 falla en claro.

## T3 — Enganche de fases

1. Cierre exitoso de workspace-init → `forge_started` (In Progress) en la fase.
2. Entrada de Verificación: si `persist_ref` tiene `implementation.md` y `execution.md`, aplicar `forge_closed` aunque Ejecución esté `skipped` o `simulated`.
3. Tras `validacion.md` con global leído por la fase Argos → `validation_apto` o `validation_no_apto`.
4. Fase de `delivery-close-cycle` que abre el PR → `pr_opened`.
5. Triaje de PR ya existente (`pull-request-review` o el camino de merge): una lectura del check → `review_ci_green` o `review_ci_red`. Sin bucle de CI.
6. Si el texto de `bug-fix` / `feature` / `refactorization` debe dejar de decir que el evento escribe el estado, forja por `entity-manager`.

## T4 — Suscriptor

1. En `stamp_event`, `Work_Initiated`, `Validation_Started`, `PullRequest_Presented` y `PullRequest_Merged` pasan a no-op de estado («sellado en origen»), en remoto y en sumidero file.
2. El file sink de columna queda en la acción de T1.

## T5 — Metadatos

1. Escribir en el frontmatter de la descripción los campos de **L-META**.
2. No renombrar `refined`. Conservar `refined_by` del refino.

## T6 — Auditoría de ciclo

1. Nueva versión de `docs/audits/ciclo-vida-software-sddia.md`: la fase escribe el estado; nombrar In Validation y Validated; el bus no es el writer de estos hitos.

## T7 — Tests

1. `aut75_forge_closed_when_execution_skipped`.
2. `aut75_argos_apto_sets_validated` y `aut75_argos_no_apto_returns_in_progress`.
3. `aut75_stamp_subscriber_does_not_move_state` para los cuatro eventos de T4.
4. `cargo test -p execute-process aut75`.

## T8 — Cierre

1. `implementation.md` y `execution.md` por tarea (T1→T7).
2. Argos → `validacion.md` `global: APTO`, `branch: Fix_Events_Multiple_03`.
3. `delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`.
4. Linear AUT-75 → Done tras merge del PR único.

## Orden

T0 → T1 → T2 → T7 (tests rojos) → T3 → T4 → T5 → T6 → T7 verde → T8.

## Pendiente de operador (fuera del diff)

Estado de workflow Linear `Validated` en el equipo AUT, antes de ejercer `validation_apto` contra el tablero real.
