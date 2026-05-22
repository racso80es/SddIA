---
feature_name: remove-route-domain-event-action
created: "2026-05-22"
process: refactorization
branch_name: feat/remove-route-domain-event-action
persist_ref: docs/features/remove-route-domain-event-action
pbi_ref: docs/todos/done/TODO_Remove_Route_Domain_Event_Action.md
---

# Objetivos — Retirada acción `route-domain-event`

## Misión

Completar la deuda K3 del feature `refactor-topologia-eventos-ola-c-v3`: eliminar la acción deprecada `route-domain-event`, su shim en `execute-action.py` y referencias activas en normativa/Core; documentar el pipeline EDA V3+ en `README.md`.

## Alcance

1. Borrar `SddIA/actions/route-domain-event.md` y fila en `actions/index.md`.
2. Retirar handler shim y entrada `ACTION_AGENT` en `execute-action.py`.
3. Actualizar referencias activas (`execution-contexts.md`, `events-contract.md`, plantillas, procesos).
4. Ampliar `README.md` con topología simétrica y pipeline watcher → proceso → sweeper.
5. Sin cambios en manifiestos históricos de backfill EDA (`backfill-manifest.json`).

## Criterio de cierre

- Enrutamiento del bus **solo** vía proceso `route-domain-event` + `event-watcher.py`.
- `execute-action --action route-domain-event` falla (acción inexistente).
- Veredicto Argos APTO en `validacion.md`.
