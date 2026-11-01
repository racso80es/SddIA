---
feature_name: reconcile-unstamped-open-budget
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/reconcile-unstamped-open-budget
pbi_ref: AUT-55
tracker_ref: AUT-55
document_id: PBI-53612d08
execution_id: "decb2cc8-7efb-4462-8de0-df6f9c772a66"
historia_ref: MAN-14
---

# Objetivos — reconcile-unstamped-open-budget

## Misión

Cerrar AUT-55 en HEAD `e5919040`: `reconcile_unstamped` no retiene `events.redb` durante todo el plan de filas, y un `open` que agota el presupuesto nombra `StoreError::Locked` y deja `elapsed_ms` en el tick.

## Alcance

- `SddIA/sddia-eda-catalog`: soltar el store entre la lectura y las escrituras. Informe con `elapsed_ms`, filas de dominio, filas de entrega y causa de `open`.
- `SddIA/daemons/event-watcher`: línea de tick con ese informe. Se conserva el `eprintln` de error de AUT-32.
- `SddIA/sddia-event-store`: `Display` de `Locked` antepone `StoreError::Locked`.
- Nota en `SddIA/evolution/` vinculada a AUT-55.

## Fuera

- Logs de claim y de ruta de AUT-32 (`docs/features/event-watcher-redb-queue-logs-metrics/`).
- `drop(store)` previo al spawn de `route-domain-event`.
- Store nuevo. Lectura del ECST para drenar.
- Cambiar el default de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`.

## Ley aplicada

- Git vía `skill:git-manager` en la entrega. Esta sesión no hace checkout: `SDDIA_LAB_SKIP_GIT=1`. HEAD es `Fix_Events_Multiple`.
- El motor nombra `fix/reconcile-unstamped-open-budget`. Esa rama no se abre.
- Provider `linear`: cierre en el issue, sin fichero nuevo en `docs/todos/pending/` ni `docs/todos/done/`.
