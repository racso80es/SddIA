---
feature_name: tracker-sync-failed
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — deuda de sincronización EDA.md
---

# Validación — tracker-sync-failed

- `tracker-sync-failed.md` + `emit-tracker-sync-failed` indexados.
- `tracker-sync-replay` registrado; suscripción en `event-domain-subscriptions.json`.
- Tests: emisión sin secretos; replay apply/discard con lab mock.
