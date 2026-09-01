---
feature_name: dcc-pr-title-metachar-451dc8707819
created: "2026-09-01"
process: bug-fix
branch_name: fix/dcc-pr-title-metachar-451dc8707819
persist_ref: docs/fixes/dcc-pr-title-metachar-451dc8707819
items:
  - phase_capsules/sanitize_shell_argv_token
  - phase_capsules/classify_delivery_error_by_index
  - delivery_close/title_metachar_fracture_suppression
  - enrich_fracture_pbi_kaizen/shell_metachar_bucket
---

# Implementation — fractura `451dc8707819`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `phase_capsules.rs` | `sanitize_shell_argv_token`; `pr_title` saneado antes de argv; preflight head/base; `PR_TITLE_METACHAR` ≠ `PR_BODY_METACHAR` |
| `phase_capsules.rs` | `delivery_phase_failed` sella `friction_id` (`F-DCC-PR-TITLE-METACHAR` / body / shell) |
| `delivery_close.rs` | `dcc_title_metachar_block_suppresses_fracture` — Apertura blocked/failed tipada no emite Kintsugi |
| `enrich_fracture_pbi_kaizen.rs` | cubo `is_shell_metachar_fracture_trace` sobre `error_trace`; `process_fix`; no hook |

## Contrato (implementado)

- Specimen `>` en título → token argv seguro (`gt`); no `arguments[3]`.
- `arguments[9]` sigue `PR_BODY_METACHAR` (K2).
- Mayeuta: traza `[PR_BODY_METACHAR] arguments[3]…` no es recursión ni catch-all.
- Allowlist `shell-executor` intacta. Sin `SDDIA_HOOK_DELIVERY_CLOSE` nuevo.

## Fuera de alcance (respetado)

- Sin mutación de genoma `delivery-close-cycle.md` / `shell-executor`.
- Sin relajar `assert_safe_token`.
- Sin reabrir K2 `--body-file`.
