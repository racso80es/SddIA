---
feature_name: dcc-shell-executor-wasm-fallback-1479509cab7d
created: "2026-09-05"
process: bug-fix
phases:
  - capsules-fallback-no-wasm-retry
  - delivery-close-sentinel-suppress
  - mayeuta-wasm-fallback-bucket
  - verify-unit
  - evolution-and-docs
branch_name: fix/dcc-shell-executor-wasm-fallback-1479509cab7d
persist_ref: docs/fixes/dcc-shell-executor-wasm-fallback-1479509cab7d
---

# Plan — fractura `1479509cab7d`

Corte Diseño: **spec + plan + commit**. Ejecución (fases 1–5) en el mismo ciclo hasta PR verde y `accept-pr`.

## Fase 1 — Fallback `shell-executor` (D1, D2, CA1, CA5)

`capsules.rs`:

- Constante pública del centinela.
- `shell_executor_native_missing_msg()` — literal Ola 3 + sufijo WASI.
- `shell_wasm_followup(wasm_err, native_available) -> ShellWasmFollowup` (`UseNative` | `Fail(String)`).
- `run_shell(..., emit_fallback_marker: bool)`: solo WASM convierte a centinela.
- `invoke_shell_executor`: si WASM `Err` y followup `UseNative` → nativo sin marcador; si `Fail` → return (nunca re-ejecutar WASM).

Tests unitarios de `shell_wasm_followup` y de que el mensaje nativo con marcadores **no** se reescribe a centinela (función de conversión, no e2e wasmtime).

## Fase 2 — Aduana Kintsugi (D3, CA2, CA3)

`delivery_close.rs`: `dcc_lab_binary_missing_trace` añade `shell-executor wasm fallback marker`. Test positivo del centinela + `emit_dcc_phase_fractures` vacío en `Apertura en forja`. Negativos Ola 3 intactos. Literal canónico `shell-executor` sigue positivo (regresión).

## Fase 3 — Mayeuta (D4, CA4)

`enrich_fracture_pbi_kaizen.rs`: `is_shell_executor_wasm_fallback_trace` **antes** del catch-all `timeout|failed`. Veredicto `refactor_tool`. Test centinela: cero `Head sha` / `Halt de Apertura`. Test `analyze_fracture_kaizen_head_sha_blank_not_hook` intacto.

## Fase 4 — Tests

```text
cargo test -p execute-process -- shell_wasm_followup dcc_lab_binary_missing dcc_fracture_suppressed analyze_fracture_kaizen_shell_executor
```

Ampliar a `cargo test -p execute-process` si el filtro es verde.

## Fase 5 — Evolution + cierre documental

`sddia-qa evolution-register` (`modificacion`, motor `execute-process`). `implementation.md` / `execution.md` / `validacion.md`. PBI → `docs/todos/done/`. CA de CI `PENDIENTE-CI` hasta `run_id` verde. Entonces `delivery-close-cycle` y, tras checks verdes, `accept-pr`.
