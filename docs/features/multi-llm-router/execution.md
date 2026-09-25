---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
execution_id: "31a62dff-7b1e-454f-9539-eccc0b16eb3a"
document_id: PBI-MULTI-LLM-ROUTER
items_applied:
  - T0-contract
  - T1-registry
  - T2-adapters
  - T3-router
  - T4-aiua
  - T5-vault-docs
---

# Ejecución — multi-llm-router

## Init

`execution_id` `31a62dff-7b1e-454f-9539-eccc0b16eb3a`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`). Commit planificación `df20a69`. Laudos 2026-09-25: `L-TERM`, `L-LOCUS`, `L-VAULT`.

## Forja (entity-manager / EDA)

| Entidad | Operación | Sello | hash_new | UUID |
|---------|-----------|-------|----------|------|
| `tool:llm-router` 1.0.0 | create | `101d09fd-c862-4e00-9fc7-f6652654335c` Domain_Entity_Created | `sha256:d99a4a38…` | `3836c0da-…` |
| `tool:gemini-http-infer` 1.1.0 | hash_refresh | `325fa592-65b5-4b73-9548-c894c65b04ff` Domain_Entity_Updated | `sha256:68392e22…` | `7a8da3ad-…` |
| `process:aiua-stimulus-processing` 1.3.0 phases | update | `caeadda9-a163-4240-b2cd-79dc044535e8` Domain_Entity_Updated | `sha256:b0ad8622…` | `6c595785-…` |
| cuerpo replacements | update | idempotente (mismo event_id) | `sha256:b0ad8622…` | inmutable |
| `skill:antigravity-cli-executor` 1.1.0 | emit-domain-mutation (creator update recrearía) | `69046e07-3744-438c-abdf-2aedacd61b2a` | `sha256:eb736da3…` | `d8b07e6f-…` |

## Fuentes Rust (edición directa)

- `gemini-http-infer`: `systemInstruction`, `effort`→thinking, `timeout_ms`, `classify_error_code`, `telemetry_receipt`.
- `antigravity-cli-executor`: merge `llm.infer`; classify `dial tcp`/`network` antes de timeout genérico.
- `llm-router`: selección, ciclo, salto L-SALTO, stubs de fallback/auth/registro ausente.
- `aiua_stimulus.rs`: `infer_via_router` → `invoke_tool_capsule_json("llm-router")`; lab `copy_router_stack` + fallback `rate_limited`.

## Tests locales (pre-PR)

```text
cargo test -p llm-router -p gemini-http-infer -p antigravity-cli-executor -p execute-process
```

Veredicto en `validacion.md` (PENDIENTE-CI hasta run GitHub verde).
