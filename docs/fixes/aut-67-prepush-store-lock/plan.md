---
feature_name: aut-67-prepush-store-lock
created: "2026-11-02"
process: bug-fix
phase: blueprint
status: implemented
agents: dedalo
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-67-prepush-store-lock
pbi_ref: AUT-67
document_id: PBI-3d736171
uuid: e46d8e0c-b9fa-4d18-a035-99e9cf66ac07
tracker_ref: AUT-67
execution_id: "e5c2418a-ce87-45ae-a522-2246568fd632"
---

# Plan — aut-67-prepush-store-lock

Diseño sellado. Ejecución (código del hook y del watcher) queda para Tekton en un estímulo posterior. Rama de trabajo: `Fix_Events_Multiple_02` (el init canónico intentó `fix/aut-67-prepush-store-lock` desde `main`; esa rama se eliminó).

## T1 — Cesión del watcher

1. En `event-watcher`, garantizar que `reconcile_unstamped` y `claim_store_fractal` sueltan `EventStore` antes de cualquier `route-domain-event` hijo del mismo tick.
2. No abrir el store de fractal mientras `claim_store_pending` aún tenga el handle, ni mientras el book marque rutas en vuelo que abren el mismo fichero, si eso alarga la retención más allá de la lista.
3. Test de unidad o lab: dos aperturas secuenciales no solapadas en un tick con spawn simulado.

## T2 — Reintento del hook

1. En `pre_push_gate.sh`, si `route-domain-event` de `Local_QA_Requested` falla con `StoreError::Locked` / `Database already open` / `Cannot acquire lock`, reintentar la inyección con presupuesto del hook (independiente de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`).
2. Agotar ese presupuesto: exit distinto y mensaje `STORE_LOCK_CONTENTION`. No usar la cadena `QA no aprobada`.
3. Un fallo de ruta que no sea lock sigue el bloqueo actual.

## T3 — Prueba

1. Lab: poseedor breve del fichero + pre-push (o función extraída) → la inyección acaba en acuse de ruta.
2. Lab: poseedor que no suelta → mensaje de contención.
3. Confirmar que el default de presupuesto de `EventStore::open` no cambia en el diff.

## T4 — Cierre

1. Evolution del uuid `e46d8e0c-b9fa-4d18-a035-99e9cf66ac07` cuando haya mutación de genoma o de hook versionado.
2. `validacion.md` APTO en esta rama. `done_gate: linear`: AUT-67 a Done lo sella el operador tras el merge. Sin ficheros nuevos en `docs/todos/`.
