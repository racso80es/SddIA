---
feature_name: lifecycle-phase-state-name
created: 2026-11-06
updated: 2026-11-06T10:05:00Z
process: bug-fix
branch: fix/lifecycle-phase-state-name
branch_name: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
document_id: PBI-adfcf4cf
uuid: adfcf4cf-ae0e-4dea-95ca-ac91e8e1071d
execution_id: 090e6ae4-6227-4ab7-b25c-64dab6741fe4
global: APTO
checks:
  ca1_payload_state_name: APTO
  ca2_no_state_key: APTO
  ca3_missing_map_key: APTO
  ca4_aut75_file: APTO
  ca5_eda_bus_e2e: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
- SddIA/engine/execute-process/src/engine/lifecycle_phase_action.rs
- SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs
- SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
- SddIA/evolution/41e8eac1-a71c-4275-99a1-04eb87134576.md
- SddIA/evolution/2bb70d0e-b01c-4d8c-b972-d58de1be6a57.md
- SddIA/evolution/Evolution_log.md
- docs/fixes/lifecycle-phase-state-name/
validated_base_sha: 8aa6170be59b8cb60a5ffc67c5ccb6e00c07df09
validated_head_sha: f017dd59c7459eea67e7d77e40d9fd6611fcc64e
validated_tree_sha: c620a41029e095d9c342cb6cd4302473c742fec3
diff_digest: sha256:21cc0eb86c0978fec60152c74044983217968bc63672b1e9160d1e0ce6eb8443
---

# Validación — lifecycle-phase-state-name

## Veredicto

**global: APTO** — rama `fix/lifecycle-phase-state-name`.

## Evidencia

| CA | Resultado |
| --- | --- |
| 1 payload ≡ stamp | `state_name` + `team_key` + `project_id` |
| 2 sin `state_key` | `update_issue_state_request_maps_canonical_to_linear_name` · `pr_opened_remote_request_uses_linear_state_name` |
| 3 mapa sin clave | `update_issue_state_request_errors_when_map_lacks_key` |
| 4 file-provider | `aut75` 6/6 |
| 5 eda-bus-e2e-smoke | `TRACKER_DONE_GUARD` permite `accept-pr` en ReviewCiGreen; `linear_direct_cycle_e2e` ac7/ac9 OK (lab local) |

`validated_*` / `diff_digest`: DCC los rellena antes del merge.

## Cierre

`delivery-close-cycle` + un PR. Linear AUT-79. Sin `docs/todos/`.