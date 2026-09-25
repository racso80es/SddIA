---
feature_name: multi-llm-router
created: "2026-09-25"
process: feature
items:
  - T0-contract
  - T1-registry
  - T2-adapters
  - T3-router
  - T4-aiua
  - T5-vault-docs
branch_name: feat/multi-llm-router
persist_ref: docs/features/multi-llm-router
execution_id: "31a62dff-7b1e-454f-9539-eccc0b16eb3a"
document_id: PBI-MULTI-LLM-ROUTER
---

# Implementation — multi-llm-router

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/library/norms/capability-taxonomy.md` | v1.0.8 término `llm:infer` / contrato `llm.infer` |
| `SddIA/library/norms/capability-contracts/llm.infer.schema.json` | Contrato adaptador (request + `error_code` + `telemetry_receipt`) |
| `SddIA/library/norms/capability-contracts/llm-registry.schema.json` | Esquema de registro de oráculos |
| `SddIA/core/cumulo.paths.json` | `instance.llm_registry` → `.SddIA/llm-registry.json` |
| `SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json` | `oracle-agy` → `oracle-gemini`; `model: ""` |
| `SddIA/tools/gemini-http-infer/{src/main.rs,.md}` | v1.1.0 `llm.infer` + `error_code` + `provides llm:infer` |
| `SddIA/skills/antigravity-cli-executor/{src/main.rs,.md}` | v1.1.0 ídem; classify network antes de timeout |
| `SddIA/tools/llm-router/{Cargo.toml,src/main.rs,.md}` | Alta `tool:llm-router` 1.0.0 (entity-manager create) |
| `SddIA/process/aiua-stimulus-processing.md` | v1.3.0 `Combustion-Inferencia` → `tool:llm-router` |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | `infer_via_router`; telemetría `provider`/`routing_attempts`/`cognitive-degraded` |
| `SddIA/scripts/qa/build-wasi-capsules.sh` | Excluye `llm-router` (subprocess nativo) |
| `README.md` | Fila DI `llm:infer` |
| Ambos `.env.example` | Bloque registro / `SDDIA_LLM_REGISTRY_PATH` |
| `SddIA/evolution/d2e44083-ccdf-45af-b477-f6c71833fc31.md` | Registro uuid PBI |

## Runtime

1. Aiúa ensambla prompt (`invoke-aiua-core`) y llama `tool:llm-router` con `affinity: aiua`.
2. Router lee `.SddIA/llm-registry.json` (`SDDIA_LLM_REGISTRY_PATH` o default Cúmulo). Selección: `oracle_id`, luego affinity, luego primer `active`.
3. Salto solo `rate_limited|timeout|upstream_unavailable|network`. `auth`/`malformed_response` cortan la cadena.
4. `attempts[]` + `telemetry_receipt.provider`. `cognitive-degraded` si hubo hop.
5. Adaptadores aceptan `request` `llm.infer` y payload legacy. Sin `provides llm:interact`.
