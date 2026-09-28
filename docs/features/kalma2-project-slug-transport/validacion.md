---
feature_name: kalma2-project-slug-transport
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — transporte project_slug en Kalma2.md
---

# Validación — transporte project_slug

- UI: selector inerte (`/api/projects`, solo slug/id/label); `POST /api/execute` envía `project_slug` opcional.
- `kalma2-bridge`: reenvío a `kalma2-interact` + `GET /api/projects` sin `project_root`.
- `kalma2-interact` 1.1.2 + evento `Kalma2_Process_Requested` 1.1.0 (`payload.project_slug`).
- `route_domain_core::sdlc_process_request_inputs` y `task-queue-manager` propagan a `inputs.project_slug`.
- AC-1: sin `project_root` literal en `interfaces/kalma2/`.
- Tests: `kalma2_event_carries_project_slug`, `aiua_process_requested_maps_like_kalma2` (rama slug).
