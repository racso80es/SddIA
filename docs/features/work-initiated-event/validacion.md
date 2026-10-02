---
feature_name: work-initiated-event
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — evento Work_Initiated.md
---

# Validación — work-initiated-event

- `SddIA/events/domain/work-initiated.md` indexado.
- `emit-work-initiated-event` nativo en `actions.rs`.
- `workspace-init` emite con `warn` si falla; procesos sin `tracker-operations`.
