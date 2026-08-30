---
feature_name: ael-ca9-dcc-evolution-phase
created: "2026-08-30"
process: bug-fix
branch_name: fix/ael-ca9-dcc-evolution-phase
persist_ref: docs/fixes/ael-ca9-dcc-evolution-phase
items:
  - phase_capsules/evolution_gate_sync_base
  - pre_push_gate/hook_delegates
  - entity-manager/delivery-close-cycle-1.4.0
---

# Implementation — AEL-CA9 residual

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `phase_capsules.rs` | `evolution_gate_args()` = `--json --range --sync-base`; `SDDIA_LAB_SKIP_EVOLUTION_GATE` |
| `pre_push_gate.sh` | `run_evolution_gate` solo si `#branches==0` |
| `hook_common.sh` | `pre_push_hook_runs_evolution_gate` |
| `delivery-close-cycle.md` | v1.4.0 vía `entity-manager` (notas + intent `--sync-base`) |

## Fuera de alcance (respetado)

- Sin mutación manual previa al creator.
- Sin `--require-synced-base` en DCC local.
