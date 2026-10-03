---
feature_name: kalma2-tracker-backlog
process: feature
branch: feat/tracker-operations-context
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — backlog Kalma2.md
---

# Validación — kalma2-tracker-backlog

- `tracker-backlog-query` + handler nativo síncrono.
- `GET /api/backlog` delega en `execute-process` (no cápsula directa).
- UI Kalma2: aviso sin tracker, filtros kind/state, recarga al cambiar proyecto.
