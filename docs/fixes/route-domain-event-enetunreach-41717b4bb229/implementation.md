---
feature_name: route-domain-event-enetunreach-41717b4bb229
created: "2026-09-08"
process: bug-fix
branch_name: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref: docs/fixes/route-domain-event-enetunreach-41717b4bb229
items:
  - dlt-transient-predicate
  - mayeuta-dlt-bucket
  - unit-tests
---

# Implementación — fractura `41717b4bb229`

## Touchpoints

| Ítem | Path | Cambio |
|------|------|--------|
| Predicado F2 | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` `dlt_transient_network_trace` | Tokens `enetunreach`, `etimedout`, `enotfound`, `network is unreachable`, `connection timed out`. Guardia al inicio de `emit_dlt_batch_fracture`. |
| Cubo F3 | `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` | `is_dlt_publish_error_trace` → `process_fix`; catch-all `failed` niega el cubo. |
| Tests | ambos `#[cfg(test)]` | Traza PBI suprime fractura; cola sí; `config-missing` emite; Kaizen sin ajuste de operador. |

## Fuera

Genoma tools/process. `server.mjs` / `relay-error.mjs`. DCC. Retry. Retry IOTA.
