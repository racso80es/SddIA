---
feature_name: aut20-ecst-router-verify
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-router-verify
tracker_ref: AUT-57
historia_ref: AUT-20
pbi_uuid: "e211b361-4811-4cbd-a366-eb6a2a21e5b0"
document_id: PBI-53d86323
---

# Objetivos — aut20-ecst-router-verify

## Misión

Cerrar los criterios 1 y 4 de AUT-20 en el router vivo `route-domain-event`: cada proceso de `event-domain-subscriptions.json` recibe el ECST del bus, y ninguna escritura Linear sale de un ECST que no coincida con ese registro.

## Alcance

Rama `Fix_Events_Multiple`. Un PBI: AUT-57, hijo de AUT-20. Sin alta en `docs/todos/`.

Dentro: `dispatch_subscriber`, la carga de `memory-evolution-ingest` y `tracker-markdown-apply` cuando no hay fichero pending, y la verificación previa a Linear en `tracker-sync-replay` y `tracker-markdown-apply`.

Fuera: criterios 2, 3, 5, 6 y 7 de AUT-20; router fractal `route-domain`; suscriptores de telemetría; pasar AUT-20 a Done.
