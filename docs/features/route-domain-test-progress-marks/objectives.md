---
feature_name: route-domain-test-progress-marks
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-test-progress-marks
tracker_ref: AUT-59
historia_ref: AUT-20
pbi_uuid: "058e378b-416e-4dc6-8a01-1ddbb3a98a60"
document_id: PBI-ea13233e
---

# Objetivos — route-domain-test-progress-marks

## Misión

Que un observador sepa, mientras corre un test `route_domain_*`, si `route_domain_event` ya fue invocado y con qué evento.

## Alcance

Rama `Fix_Events_Multiple`. Kaizen AUT-59, hijo de AUT-20. Sin alta en `docs/todos/`.

Dentro: helper y dos llamadas por test en `SddIA/engine/execute-process/tests/eda_subscriber_ecst_redb.rs`.

Fuera: router de producción, tests de librería, el fallo de `route_domain_telegram_empty_text_still_skipped`, el pipe `| tail` del operador.
