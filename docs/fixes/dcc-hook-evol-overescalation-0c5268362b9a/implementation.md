---
feature_name: dcc-hook-evol-overescalation-0c5268362b9a
created: "2026-08-31"
process: bug-fix
branch_name: fix/dcc-hook-evol-overescalation-0c5268362b9a
persist_ref: docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a
items:
  - delivery_close/hook_evol_fracture_suppression
  - delivery_close/stamp_friction_hook_evol
  - enrich_fracture_pbi_kaizen/strict_hook_cube
  - pre_push_gate/is_delete_push_local_sha
  - capsule_delivery_remote_push/hook_delivery_close_env
---

# Implementation — fractura `0c5268362b9a`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `delivery_close.rs` | `dcc_hook_evol_gate_trace` + `dcc_hook_evol_block_suppresses_fracture` |
| `delivery_close.rs` | `stamp_dcc_hook_evol_block`: Publicación remota + traza hook evolution → `blocked` + `F-DCC-HOOK-EVOL-OVERESCALATION` |
| `delivery_close.rs` | `emit_dcc_phase_fractures` no emite dominio si el predicado aplica |
| `enrich_fracture_pbi_kaizen.rs` | cubo hook: `delivery-close-cycle failed for` / `recurs` / `re-entrada`; sin `pre-push`/`hook` unarios |
| `pre_push_gate.sh` | `is_delete_push "$local_sha"` |
| `hook_common.sh` | nota: ceros = delete sobre SHA local |
| `phase_capsules.rs` | `capsule_delivery_remote_push` exporta `SDDIA_HOOK_DELIVERY_CLOSE=1` acotado al push |

## Contrato (implementado)

- Pre-push evolution gate en Publicación remota: veredicto accionable, no `System_Fracture_Detected`.
- F4b y F4c DNS intactos. Forja `pr_url` opaco sigue emitiendo.
- Mayeuta: traza canónica `0c5268362b9a` no es recursión; re-entrada real (`delivery-close-cycle failed for`) sí. Prohibido proponer reimplementar la guarda.
- SSOT AEL-CA9: DCC posee evolution en entrega; hook F4c solo si DCC no corre. Ref nueva no se clasifica como delete.

## Fuera de alcance (respetado)

- Sin mutación de genoma `delivery-close-cycle.md`.
- Sin `SDDIA_SKIP_HOOKS` global. Sin reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`.
- F0 / PR #236 fuera de código.
