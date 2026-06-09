---
feature_name: argos-pr-audited
process: feature
created: 2026-06-09T00:00:00Z
items:
  - SddIA/events/domain/pull-request-audited.md
  - SddIA/events/domain/index.md
  - SddIA/agents/argos.md
  - SddIA/actions/emit-pr-audited-event.md
  - SddIA/actions/emit-pr-merged-event.md
  - SddIA/core/event-subscriptions.json
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_capsules.py
---

# Implementación — Argos PullRequest_Audited

## Touchpoints

| Ámbito | Cambio |
|--------|--------|
| **ECST** | `pull-request-audited.md` — frontmatter `event_family`, `event_type`, `capabilities`, `hash_signature` |
| **Códice** | `SddIA/events/domain/index.md` — fila `pull-request-audited` |
| **Argos** | `argos.md` — `outputs` alineados con bucle operativo §2 |
| **Emisión** | `emit-pr-audited-event` + handler en `execute-action.py` |
| **Aduana** | `capsule_pr_review_verdict` → `invoke_capsule_action(emit-pr-audited-event)` |
| **Merge DLT** | `emit-pr-merged-event` — `audit_event_reference` resuelto vía input o bus |
| **Suscripciones** | `event-subscriptions.json` → `PullRequest_Audited` |
| **Higiene** | Eliminados `mock-argos-output.json`, `.dev/test-argos-emission.sh` |

## Flujo

1. `pull-request-review` / fase Veredicto → `emit-pr-audited-event` → `.events/pending/`
2. `accept-pr` / `emit-pr-merged-event` → `security_clearance.audit_event_reference` desde evento `PullRequest_Audited` correlacionado
