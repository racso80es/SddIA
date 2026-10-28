---
feature_name: bug-fix-done-before-pr-merge
created: "2026-10-28"
process: bug-fix
tracker_ref: AUT-12
execution_id: "dea619eb-78cc-498c-9747-8cd6e34bb14b"
---

# Spec — AUT-12 Done adelantado con PR abierto

## Síntoma

Cierre de `bug-fix` deja el issue Linear en workflow **Done** mientras el PR sigue abierto.

## Causa raíz

1. **`inputs.delivery_mode: trunk_direct`** sobre proyecto Core (`branch_pr` en manifiesto) hacía que `delivery-close-cycle` emitiera `Delivery_Committed` → `tracker-stamp` sellaba **Done** antes de `PullRequest_Merged`.
2. Cualquier invocación Linear `update_issue_state` → **Done** fuera de `tracker-stamp` / `tracker-sync-replay` (p. ej. operador o proceso lateral en cierre documental).

## Cambios (motor)

| Archivo | Cambio |
|---------|--------|
| `project_binding.rs` | Core-self ignora override `inputs.delivery_mode`; usa solo manifiesto |
| `tracker_stamp.rs` | `Delivery_Committed` no sella Done sin `payload.delivery_mode == trunk_direct` |
| `tracker_operations_gate.rs` + `capsules.rs` | `TRACKER_DONE_GUARD` bloquea Done salvo `tracker-stamp` / `tracker-sync-replay` |

## Criterios de cierre (AUT-12)

- Cierre `bug-fix` / DCC en Core **no** transiciona a Done por `Delivery_Committed` accidental.
- `PullRequest_Presented` sigue dejando `in_review` (sin cambio).
- Done solo tras `PullRequest_Merged` (o `Delivery_Committed` legítimo en `trunk_direct`).

## Tests

- `bind_core_ignores_inputs_trunk_direct_override`
- `delivery_committed_without_trunk_direct_no_ops`
- `done_state_transition_blocked_outside_tracker_stamp`
