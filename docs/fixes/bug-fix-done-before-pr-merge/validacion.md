---
feature_name: bug-fix-done-before-pr-merge
created: "2026-10-28"
process: bug-fix
branch: fix/bug-fix-done-before-pr-merge
global: APTO
tracker_ref: AUT-12
pbi_archived: false
pr_url: https://github.com/racso80es/SddIA/pull/406
checks:
  AC-CORE-MODE: APTO
  AC-DCC-STAMP: APTO
  AC-DONE-GUARD: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/project_binding.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/engine/execute-process/src/engine/linear_direct_cycle_e2e.rs
  - docs/fixes/bug-fix-done-before-pr-merge/
---

# Validación — AUT-12

Tests locales:

- `bind_core_ignores_inputs_trunk_direct_override` — OK
- `delivery_committed_without_trunk_direct_no_ops` — OK
- `done_state_transition_blocked_outside_tracker_stamp` — OK

Criterio AUT-12: cierre `bug-fix` en Core no puede sellar Done vía `Delivery_Committed` por override `trunk_direct`; Done en Linear queda acotado a `tracker-stamp` / replay.

PR #406 abierto; `PullRequest_Presented` (`9a5da911-1cab-4869-af9c-e5fabfd28a32`). Pendiente: merge → `PullRequest_Merged` → Done en Linear (`done_gate: linear`).
