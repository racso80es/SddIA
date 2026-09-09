---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
items:
  - forge-process-em
  - handler-antigravity
  - bridge-sanitize-timeout
  - starter-kit-env
  - tests-lab-mock
branch_name: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
---

# Implementation — aiua-antigravity-cli-vector

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/process/aiua-stimulus-processing.md` | EM update 1.1.0; UUID `6c595785-…`; `skill:antigravity-cli-executor`; input `effort` |
| `SddIA/process/index.md` | Fila 1.1.0 |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | `invoke_capsule_json`; effort/timeout; `result.usage`; lab-mock `lab-mock-agy:` |
| `SddIA/interfaces/kalma2-bridge/src/main.rs` | Sanitize auth/timeout `agy`; timeout 3-techos |
| `SddIA/scripts/starter-kit/.dev/.env.example` | `SDDIA_AGY_EFFORT` / `SDDIA_AGY_TIMEOUT_SECS` comentadas |
| `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` | Igual |

## Runtime

1. `effort` siempre (`high` default). `HIGH` env → `high`.
2. `print_timeout` `{N}s` desde `SDDIA_AGY_TIMEOUT_SECS` ≻ `SDDIA_CLIENT_TIMEOUT_SECONDS` ≻ `300`.
3. Lab-mock: `SDDIA_LAB_MOCK_OUTBOUND=1` → `lab-mock-agy:`; tokens omitidos si `usage {}`.
4. Bridge: `SDDIA_AGY_TIMEOUT_SECS` unset no sube el default 120 de `/api/chat`.
