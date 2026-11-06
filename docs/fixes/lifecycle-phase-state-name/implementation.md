---
feature_name: lifecycle-phase-state-name
created: "2026-11-06"
process: bug-fix
phase: implementation
agents: tekton
branch_name: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
document_id: PBI-adfcf4cf
uuid: adfcf4cf-ae0e-4dea-95ca-ac91e8e1071d
execution_id: "090e6ae4-6227-4ab7-b25c-64dab6741fe4"
---

# Implementación — lifecycle-phase-state-name

## T1 — Helper único

`tracker_stamp::update_issue_state_request`: `canonical` → `state_name` vía `state_map`; request con `state_name`, `team_key`, `project_id`, `issue_ref`. Sin `state_key`.

Consumidores: `tracker_stamp::tool_update_state`, `lifecycle_phase_action::tool_update_state`, `tracker_sync_replay` en `update_issue_state`.

`apply_file` sigue con clave canónica.

## T2 — Tests

- `update_issue_state_request_maps_canonical_to_linear_name`
- `update_issue_state_request_errors_when_map_lacks_key`
- `pr_opened_remote_request_uses_linear_state_name`
- filtro `aut75`: 6/6

## T3 — Evolution

`41e8eac1-a71c-4275-99a1-04eb87134576`
