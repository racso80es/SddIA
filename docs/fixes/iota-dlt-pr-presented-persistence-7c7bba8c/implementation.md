---
feature_name: iota-dlt-pr-presented-persistence-7c7bba8c
created: "2026-07-20"
process: bug-fix
branch_name: fix/iota-dlt-pr-presented-persistence-7c7bba8c
persist_ref: docs/fixes/iota-dlt-pr-presented-persistence-7c7bba8c
---

# Implementación — iota DLT opaco PullRequest_Presented

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `capsule_error_trace` lee `error`→`feedback`→`message`; IOTA vía `invoke_tool_capsule_json`; tests |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | Sello Presentación exige `pr_url`; test rechazo |
| `SddIA/sddia-io/src/lib.rs` | `emit_error` popula `feedback` y `error` (paridad DLT) |
| `.SddIA/services/iota-publish-relay/` | Relay HTTP instancia (SDK Testnet) para `IOTA_PUBLISH_RELAY_URL` |
| `.SddIA/.dev/.env` (no versionar) | `MOCK` vacío + `IOTA_PUBLISH_RELAY_URL=http://127.0.0.1:8787/v1/publish` |

## Decisiones

1. No mutar genoma `tools/iota-immutable-publisher.md` — el defecto era de lectura en route-domain + envelope IO.
2. Gate duro de `pr_url` en delivery-close evita ECST incompleto (causa DLT Argos correlacionado).
3. Persistencia física: relay de instancia (no mock); mock vacío porque precede al relay en la cápsula.
