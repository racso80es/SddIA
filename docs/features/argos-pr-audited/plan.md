---
feature_name: argos-pr-audited
process: feature
created: 2026-05-31T08:00:00Z
---

# Plan

1. Documentación en `docs/features/argos-pr-audited/`.
2. Contrato `SddIA/events/domain/pull-request-audited.md` + fila en `domain/index.md`.
3. Acción `emit-pr-audited-event` + handler en `execute-action.py`.
4. Cableado en `capsule_pr_review_verdict` y resolución `audit_event_reference` en merge.
5. Suscriptor `PullRequest_Audited` en `event-subscriptions.json`.
