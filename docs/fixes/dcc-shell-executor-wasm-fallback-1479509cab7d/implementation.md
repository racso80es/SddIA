---
feature_name: dcc-shell-executor-wasm-fallback-1479509cab7d
created: "2026-09-05"
process: bug-fix
branch_name: fix/dcc-shell-executor-wasm-fallback-1479509cab7d
persist_ref: docs/fixes/dcc-shell-executor-wasm-fallback-1479509cab7d
items:
  - capsules/shell_wasm_followup
  - delivery_close/sentinel_lab_binary_suppress
  - enrich_fracture_pbi_kaizen/shell_executor_wasm_fallback_bucket
---

# Implementation — fractura `1479509cab7d`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `capsules.rs` | `shell_wasm_followup` / `shell_wasm_marker_err`. WASM fallido no re-ejecuta WASM. Nativo ausente → literal Ola 3 + sufijo WASI. Vía nativa no emite el centinela. |
| `delivery_close.rs` | `dcc_lab_binary_missing_trace` casa `shell-executor wasm fallback marker`. Literal canónico Ola 3 intacto. |
| `enrich_fracture_pbi_kaizen.rs` | `is_shell_executor_wasm_fallback_trace` → `refactor_tool`. Cubo head-sha intacto. |

## Contrato

- Sin `F-DCC-LAB-BINARY-MISSING`. Sin mutar `phase_capsules.rs`. Sin reabrir `halt_after_push`.
- Genoma DA-2 intacto.
