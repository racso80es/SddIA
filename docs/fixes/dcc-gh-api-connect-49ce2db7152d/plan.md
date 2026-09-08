---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
phases:
  - extend-f4c-predicate
  - stamp-ok-failed-forge
  - verify-unit
  - evolution
  - document-dcc
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
---

# Plan — fractura `49ce2db7152d`

## Fase 1 — Predicado (CA-1/CA-3/CA-4)

`dcc_transient_network_trace`: añadir `error connecting to api.github.com` y `check your internet connection or https://githubstatus.com`.

Test: positivos nuevos; negativo opaco `no se pudo resolver pr_url desde gh` intacto.

## Fase 2 — Sello en Ok(failed) (CA-2)

Tras merge de `execute_delivery_close_phase` `Ok`, si la traza es F4c invocar `stamp_dcc_network_block` (hoy solo en rama `Err`).

`emit_dcc_phase_fractures` ya suprime si el predicado matchea; no tocar F4b.

Tests: fixture traza PBI → pending vacío; `stamp_dcc_network_block` sobre esa traza → `blocked` + `F-DCC-DNS-UNRESOLVED`; `dcc_fracture_emits_on_failed_forge_phase` verde.

## Fase 3 — Verificación

```text
cd SddIA && cargo test -p execute-process --lib -- dcc_transient dcc_fracture stamp_dcc_network
```

## Fase 4 — Evolution + cierre

`sddia-qa evolution-register` (`modificacion`). `implementation.md` / `execution.md` / `validacion.md`. PBI → `done/`. DCC. CA-CI. `accept-pr`.
