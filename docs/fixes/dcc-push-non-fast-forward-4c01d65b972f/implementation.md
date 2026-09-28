---
feature_name: dcc-push-non-fast-forward-4c01d65b972f
created: "2026-09-28"
process: bug-fix
branch_name: fix/dcc-push-non-fast-forward-4c01d65b972f
persist_ref: docs/fixes/dcc-push-non-fast-forward-4c01d65b972f
items_applied:
  - fracture-signatures-dcc-suppress-non-ff
  - delivery_close_stamp_non_ff
  - entity-manager-delivery-close-cycle
  - obediencia-non-ff-escalado
---

# Implementation — fractura `4c01d65b972f`

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/core/fracture-signatures.json` | `F-DCC-PUSH-NON-FAST-FORWARD`: `dcc_suppress`, `fracture_policy`, `operator_hint` |
| `delivery_close.rs` | `dcc_non_ff_block_suppresses_fracture`, `stamp_dcc_non_ff_block`, hook en fase y `emit_dcc_phase_fractures` |
| `delivery-close-cycle.md` | Nota F4c non-FF vía `entity-manager` (UUID `5417c92c-da7f-4d46-b245-55cf1b17961a`) |
| `obediencia-procesos.md` | § Push non-fast-forward v1.4 |

Mayeuta (C-RC2): cubierto por Kaizen previo (`fracture_corpus_regression` `4c01d65b972f`).
