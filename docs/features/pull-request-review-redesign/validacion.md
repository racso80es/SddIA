---
feature_name: pull-request-review-redesign
branch: feat/pull-request-review-redesign
global: true
checks:
  - id: V1
    name: Genoma v2 + hash
    status: pass
  - id: V2
    name: Smoke positivo aduana
    status: pass
  - id: V3
    name: Smoke rechazo documental
    status: pass
  - id: V4
    name: E2E bus Presented → aduana
    status: pass
git_changes:
  - SddIA/process/pull-request-review.md
  - SddIA/core/event-subscriptions.json
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/daemons/event-watcher.py
---

# Validación — Aduana `pull-request-review`

## Criterios de aceptación

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| V1 | Genoma v2.0.0 + `verify-process-integrity` | ✅ | hash `4408f797…` |
| V2 | Smoke positivo → `verdict: aprobado` | ✅ | `_smoke-pr-review-presented.json` |
| V3 | Violación simulada → `delivery_state: failed` | ✅ | `SDDIA_LAB_PR_REVIEW_DOC_FAIL` |
| V4 | `PullRequest_Presented` → watcher → aduana | ✅ | evento `62bcb6e1-…` |
| V5 | Kaizen no bloqueante | ✅ | `SDDIA_LAB_PR_REVIEW_KAIZEN` |
| V6 | Handoff → `accept-pr` (sin merge en aduana) | ✅ | default `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=0`; cierre PR #15 |

## Eventos bus (E2E)

| event_id | event_type | Terminal | delivery_state |
|----------|------------|----------|----------------|
| `62bcb6e1-f995-4edf-95d6-3745c7503303` | `PullRequest_Presented` | `processed/` | `argos: success`, `cumulo: success` |

## Pendiente post-feature

- Purge refs `validate-pull-requests` en labs `SddIA_1`…`SddIA_4`
- Comentarios atómicos Argos en forja (runtime IDE)
