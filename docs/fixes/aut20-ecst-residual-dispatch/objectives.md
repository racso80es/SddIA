---
feature_name: aut20-ecst-residual-dispatch
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-residual-dispatch
tracker_ref: AUT-58
historia_ref: AUT-20
pbi_uuid: "19d83353-9bd9-4289-980d-4842bec234ca"
document_id: PBI-5e8122e3
---

# Objetivos — aut20-ecst-residual-dispatch

## Misión

Cerrar el resto del criterio 1 de AUT-20: `telegram-fallback-responder` y `task-queue-manager` reciben el ECST del bus, y hay un test de despacho por cada clave de proceso del catálogo de dominio.

## Alcance

Rama `Fix_Events_Multiple`. Un PBI: AUT-58, hijo de AUT-20. Sin alta en `docs/todos/`.

Dentro: las dos ramas de `dispatch_subscriber` que salen antes del catálogo ECST de AUT-57, y el test que hoy no usa la clave de proceso.

Fuera: criterios 2, 3, 4, 5, 6 y 7; `verify_ecst_matches_bus`; el corte `branch` de `pull-request-review`; pasar AUT-20 a Done.
