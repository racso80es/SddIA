---





feature_name: linear-hu-b-06-e2e
persist_ref: docs/features/linear-hu-b-06-e2e
document_id: PBI-LINEAR-B-06-E2E
execution_id: "f48c946d-3b59-4540-8f86-34338e7699c0"
---
# Spec — HU-B 06

| AC | Verificación |
|----|----------------|
| AC-21 | `linear_done_gate_both_e2e` — merge stamp → `Tracker_Issue_Changed` → archive en `todos_done` |
| AC-21b | outbound anti-eco tras `tracker-stamp` |
| AC-21c | sin `Tracker_Sync_Failed` / `TRACKER_STATE_DIVERGED` en el ciclo |
| AC-21d | fixture `lab-done-gate-project-both` permanece en `both` |

CI: `sddia-qa run-linear-done-gate-both-e2e-lab`.
