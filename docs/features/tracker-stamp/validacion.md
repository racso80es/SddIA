---
feature_name: tracker-stamp
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — proceso tracker-stamp.md
---

# Validación — tracker-stamp

- `tracker-stamp` + suscripciones en `event-domain-subscriptions.json`.
- `tracker_ref` opcional en emit PR (presented/merged/audited).
- No-op sin `tracker_ref`; test unitario.
