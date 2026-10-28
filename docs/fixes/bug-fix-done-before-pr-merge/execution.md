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

## Entrega (DCC)

| Campo | Valor |
|-------|--------|
| `execution_id` (DCC) | `a3d5bcf4-41e4-4d2e-a427-42fa8b7cc7ed` |
| PR | https://github.com/racso80es/SddIA/pull/406 |
| `PullRequest_Presented` | `9a5da911-1cab-4869-af9c-e5fabfd28a32` → `.events/pending/` (routed post-acuse) |
| Snapshot DCC | `6d9f2ad5` (higiene instance revertida en `c54f522a`) |
| Evolution | `6a4a49f0-24e8-446a-b5e5-f20c9bd68319` · gate `EVOL_OK` en `60f7f64b` |

## Commits (motor)

- `32e5a56b` — fix AUT-12 (guards Done / delivery_mode)
- `c54f522a` / `f15b806e` / `60f7f64b` — evolution + rehash + correlato e2e

## Verificación

`cargo test -p execute-process --lib bind_core_ignores_inputs_trunk_direct_override delivery_committed_without_trunk_direct_no_ops done_state_transition_blocked_outside_tracker_stamp`

## Linear (AUT-12) — auditoría 2026-10-28

Consulta `fetch_issue` (`linear-tracker-adapter`): **In Progress** (`updated_at` 2026-10-28T18:39:24Z). Enlace: https://linear.app/oscar-perez/issue/AUT-12/bug-el-cierre-adelanta-done-con-el-pr-abierto

Tras merge PR #406 → esperar **Done** solo vía `PullRequest_Merged` / `tracker-stamp` (no cierre manual).
