---
feature_name: bug-fix-done-before-pr-merge
created: "2026-10-28"
process: bug-fix
branch_name: fix/bug-fix-done-before-pr-merge
execution_id: "dea619eb-78cc-498c-9747-8cd6e34bb14b"
items_applied:
  - core_delivery_mode_lock
  - delivery_committed_trunk_guard
  - tracker_done_guard
---

# Ejecución

Rama `fix/bug-fix-done-before-pr-merge`. Init `dea619eb-78cc-498c-9747-8cd6e34bb14b`.

Verificación: `cargo test -p execute-process --lib bind_core_ignores delivery_committed_without done_state_transition_blocked`.
