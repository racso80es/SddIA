---
feature_name: lifecycle-phase-state-name
created: "2026-11-06"
process: bug-fix
branch: fix/lifecycle-phase-state-name
branch_name: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
document_id: PBI-adfcf4cf
uuid: adfcf4cf-ae0e-4dea-95ca-ac91e8e1071d
execution_id: "090e6ae4-6227-4ab7-b25c-64dab6741fe4"
global: APTO
checks:
  ca1_payload_state_name: APTO
  ca2_no_state_key: APTO
  ca3_missing_map_key: APTO
  ca4_aut75_file: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
- SddIA/engine/execute-process/src/engine/lifecycle_phase_action.rs
- SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
- SddIA/evolution/41e8eac1-a71c-4275-99a1-04eb87134576.md
- SddIA/evolution/Evolution_log.md
- docs/fixes/lifecycle-phase-state-name/
---

# Validación — lifecycle-phase-state-name

## Veredicto

**global: APTO** — rama `fix/lifecycle-phase-state-name`.

## Evidencia

| CA | Resultado |
| --- | --- |
| 1 payload ≡ stamp | `state_name` + `team_key` + `project_id` |
| 2 sin `state_key` | `update_issue_state_request_maps_canonical_to_linear_name` y `pr_opened_remote_request_uses_linear_state_name` |
| 3 mapa sin clave | `update_issue_state_request_errors_when_map_lacks_key` |
| 4 file-provider | `aut75` 6/6 |

Adaptador Linear no modificado. Verificación viva de columna: fuera de alcance.

## Cierre

Un PR vía `delivery-close-cycle`. Sin `docs/todos/`.
