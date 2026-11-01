---
feature_name: telegram-route-ecst-gate
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/telegram-route-ecst-gate
tracker_ref: AUT-60
historia_ref: AUT-20
pbi_uuid: "1abdc419-a121-4759-b6a6-9c6cda237126"
document_id: PBI-be8ad205
---

# Objetivos — telegram-route-ecst-gate

## Misión

Que los tests `route_domain_telegram_*` crucen la aduana ECST y afirmen el despacho: texto vacío en `skipped-empty-text`, texto presente sin un verde que acepte `ecst-gate: failed`.

## Alcance

Rama `Fix_Events_Multiple`. Un PBI: AUT-60, hijo de AUT-20. Sin alta en `docs/todos/`.

Dentro: el esqueleto del repo temporal, los dos fixtures de Telegram y las aserciones de esos dos tests.

Fuera: el validador ECST, el contrato de `telegram-message-received.md`, los otros tests `route_domain_*`, las marcas AUT-59, el cierre de AUT-20.
