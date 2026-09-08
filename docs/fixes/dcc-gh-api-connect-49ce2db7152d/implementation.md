---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
items:
  - f4c-gh-api-tokens
  - stamp-ok-failed-forge
  - unit-tests
---

# Implementación — fractura `49ce2db7152d`

## Touchpoints

| Ítem | Path | Cambio |
|------|------|--------|
| Predicado F4c | `SddIA/engine/execute-process/src/engine/delivery_close.rs` `dcc_transient_network_trace` | Tokens `error connecting to api.github.com` y `check your internet connection or https://githubstatus.com` |
| Sello Ok(failed) | mismo archivo, rama `Ok` de `execute_delivery_close_phase` | `stamp_dcc_network_block` tras merge del phase entry |
| Tests | `delivery_close.rs` `#[cfg(test)]` | Traza PBI suprime fractura; stamp `blocked`; opaco pr_url sigue emitiendo |

## Fuera

Genoma DCC. Retry `gh`. PBI `41717b4bb229`.
