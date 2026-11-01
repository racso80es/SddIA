---
feature_name: route-domain-process-effect
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-process-effect
tracker_ref: AUT-61
historia_ref: AUT-20
pbi_uuid: "5cdb6c86-b91d-4c56-8778-a5136890ce59"
document_id: PBI-c5499f70
---

# Objetivos — route-domain-process-effect

## Misión

Cerrar la cláusula de prueba del criterio 1 de AUT-20: cada clave de proceso del catálogo de dominio tiene un despacho de router cuyo `delivery_status` es `success`.

## Alcance

Rama `Fix_Events_Multiple`. Kaizen AUT-61, hijo de AUT-20. Sin alta en `docs/todos/`.

Dentro: tabla y aserciones en `SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs`. Copia puntual al repo temporal solo si un handler falla por un árbol ausente. Entorno heredado por el hijo: `SDDIA_EVENT_STORE=redb`, `SDDIA_EXECUTE_PROCESS_BIN`, `SDDIA_LAB_MOCK_OUTBOUND`, `SDDIA_LAB_ROUTE_SYNC`.

Fuera: cambios de ensamblado o de handlers, criterios 2 a 7, Linear/Telegram/GitHub reales, pasar AUT-20 a Done.
