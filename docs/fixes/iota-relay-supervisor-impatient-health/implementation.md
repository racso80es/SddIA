---
feature_name: iota-relay-supervisor-impatient-health
created: "2026-08-30"
process: bug-fix
branch_name: fix/iota-relay-supervisor-impatient-health
persist_ref: docs/fixes/iota-relay-supervisor-impatient-health
items:
  - iota-publish-relay/decide_supervisor_tick
  - iota-publish-relay/child_spawned_at
---

# Implementation — fractura `701c77ebeab8`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/daemons/iota-publish-relay/src/main.rs` | `GRACE_SECS=10`, `child_spawned_at`, `in_grace`, `decide_supervisor_tick`, bucle con tick condicional y kill post-gracia |

## Contrato implementado

- Gracia anclada a spawn exitoso (`child_spawned_at`), no a `last_restart`.
- `health_ok ∨ in_grace` → `centinela.tick()`.
- `!health_ok ∧ ¬in_grace ∧ child_alive` → kill + respawn en ticks siguientes.
- Un solo `child_alive` por tick; reap limpia `child` y `child_spawned_at`.

## Tests añadidos

- `grace_refused_does_not_kill` (RELAY-CA1 / T-A)
- `post_grace_refused_kills_and_omits_tick` (RELAY-CA4 / T-B / T-C)
- `healthy_ticks_no_kill`
- `grace_boundary_eq_is_outside`
