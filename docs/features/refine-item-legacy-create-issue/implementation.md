---
feature_name: refine-item-legacy-create-issue
created: "2026-11-01"
process: refactorization
branch_name: Fix_Events_Multiple
persist_ref: docs/features/refine-item-legacy-create-issue
tracker_ref: AUT-35
execution_id: "27ab6def-6796-4baa-a3e6-c569f96efd72"
items_applied:
  - T1
  - T2
---

# Implementación — AUT-35

| Área | Cambio |
|------|--------|
| `handlers/refine.rs` | Eliminados `try_create_hu_issue`, `emit_create_issue_failed`, `backlog_state_name`, `title_from_hu`. `run_hu` no llama a `create_issue`. |
| `handlers/refine_item/tracker_ref.rs` | Predicado `resolve_remote_tracker_ref` + tests unitarios. |
| `handlers/refine_item/run.rs` | Delega resolución remota de `tracker_ref` al predicado. |
| `handlers/refine.rs` tests | `ac_0c_creates_tracker_ref_when_missing` exige `create_issue_invoked: false` y sin `tracker_ref` inventado. |
