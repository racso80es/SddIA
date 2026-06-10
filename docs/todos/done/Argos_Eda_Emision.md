---
document_id: PBI-ARGOS-EDA-EMISION
title: "[ARQUITECTURA] Emisión EDA Nativa y Trazabilidad en Argos (PullRequest_Audited)"
format: markdown
version: "1.0.0"
created: "2026-05-31"
status: cerrado
priority: alta
process: feature
branch_name: feature/argos-domain-event-audited-8966834805803533351
feature_ref_target: docs/features/argos-pr-audited
related:
  - SddIA/events/domain/pull-request-audited.md
  - SddIA/actions/emit-pr-audited-event.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/core/event-subscriptions.json
  - SddIA/agents/argos.md
---

# [ARQUITECTURA] Emisión EDA Nativa y Trazabilidad en Argos (PullRequest_Audited)

**Estatus:** Cerrado  
**Feature:** `docs/features/argos-pr-audited/`  
**Rama:** `feature/argos-domain-event-audited-8966834805803533351`

## Entregables

| Vector | Estado | Evidencia |
|--------|--------|-----------|
| A — Erradicación fósil `TODO: pending_argos_eda_emission` | ✅ | `execute-action.py`, `emit-pr-merged-event.md` |
| B — Contrato ECST `PullRequest_Audited` | ✅ | `pull-request-audited.md`, `domain/index.md` |
| C — Ignición bus vía aduana | ✅ | `emit-pr-audited-event` + `capsule_pr_review_verdict` |

## Criterios de cierre

- [x] Evento `PullRequest_Audited` en códice domain (16 ECST)
- [x] Suscriptor en `event-subscriptions.json`
- [x] `audit_event_reference` real en `PullRequest_Merged.security_clearance`
- [x] `validacion.md` APTO + PBI archivado en `done/`
