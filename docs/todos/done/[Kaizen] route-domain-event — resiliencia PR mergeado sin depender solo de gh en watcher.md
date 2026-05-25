---
document_id: PBI-KAIZEN-ROUTE-DOMAIN-EVENT-PR-MERGED-RESILIENCE
title: "[Kaizen] route-domain-event — resiliencia PR mergeado sin depender solo de gh en watcher"
format: markdown
version: "1.0.0"
created: "2026-05-25"
status: cerrado
closed: "2026-05-25"
priority: alta
process: bug-fix
branch_name: fix/route-domain-event-pr-merged-resilience
persist_ref: docs/fixes/route-domain-event-pr-merged-resilience
upstream:
  - docs/features/kaizen-alert-required-eda-v2/validacion.md
  - https://github.com/racso80es/SddIA/pull/48
related_incident:
  event_id: ce5f287e-4e27-4d18-98f6-b9201596ae00
  subscriber: argos.pull-request-review
  error_trace: "pathspec 'feat/kaizen-alert-required-eda-v2' did not match any file(s) known to git"
related:
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - docs/fixes/route-domain-event-pr-merged-resilience/
---

# [Kaizen] route-domain-event — resiliencia PR mergeado sin depender solo de gh en watcher

**Estatus:** Cerrado  
**Entrega:** `resolve_pull_request_lifecycle` + precheck router antes de `pull-request-review`.

## Resumen

Endurecimiento del router EDA para evitar dead-letter por checkout Git cuando un `PullRequest_Presented` se procesa tras merge del PR y poda de rama remota. Cadena: gh → rama remota → ref `pull/N/head`.

## Artefactos

- `docs/fixes/route-domain-event-pr-merged-resilience/validacion.md` — APTO
- Código: `eda_bus_utils.py`, `route_domain_event_core.py`, tests
