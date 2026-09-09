---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
base: main
scope: core
branch_name: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
---

# Spec — aiua-antigravity-cli-vector

## 1. Proceso `aiua-stimulus-processing`

UUID `6c595785-e386-402f-b570-0b2aa6343051` inmutable. Versión `1.1.0`.

`entity-manager` `entity_class: process` `lifecycle_operation: update`:

**Update A — frontmatter** (`process_phases` + `process_inputs` + `process_version: 1.1.0`):

- Fase `Combustion-Inferencia`: `delegates_to: [skill:antigravity-cli-executor]`; intent `Única llamada al LLM vía agy; Peaje Termodinámico del CLI.`
- Inputs: `prompt` (req), `context_query` (opt), `model` (opt; else `SDDIA_GEMINI_MODEL`), `effort` (opt; default=high).
- No enviar `process_contract_version` (vigente `1.4.0`; default EM `1.3.0` destruiría contrato).

**Update B — cuerpo** (`markdown_body_replacements` aparte; factory corta antes de phases):

- `única inferencia Gemini` → combustión vía Antigravity CLI (`skill:antigravity-cli-executor`).

## 2. Handler `aiua_stimulus.rs`

Ruta: `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs`.

| Pieza | Contrato |
|-------|----------|
| Invoke | `invoke_capsule_json(repo, "antigravity-cli-executor", payload, false)` |
| Payload | `{request: {prompt, parameters: {model?, effort, print_timeout?}}}` |
| effort | siempre; cascada L-EFFORT; whitelist `low\|medium\|high` |
| print_timeout | `SDDIA_AGY_TIMEOUT_SECS` ≻ `SDDIA_CLIENT_TIMEOUT_SECONDS` ≻ `300` → `"{N}s"` |
| text | `result.text` |
| tokens | `result.usage` si object con ≥1 key |
| model tel | modelo resuelto handler (no eco skill) |
| duration | `result.durationMs` |
| error | `body.error` ≻ `body.feedback` ≻ `"agy-failed"` |
| report | `"handler": "antigravity-cli-executor"` |
| lab-mock | copiar `antigravity-cli-executor`; aserto `lab-mock-agy:`; no tocar `SDDIA_LAB_MOCK_GEMINI_URL` |

Conservar `invoke_tool_capsule_json` para `thought-graph-access`.

## 3. Bridge `kalma2-bridge`

| Pieza | Contrato |
|-------|----------|
| Auth | substrings `agy authentication required`, `not logged into antigravity`, `please sign in`, `authentication required` (case-insensitive) → mensaje auth castellano |
| Timeout | substring `agy-timeout` → mensaje timeout castellano |
| 503 | `AIUA_PROVIDER_UNAVAILABLE_MSG` intacto |
| No-mask | `timeout motor`, `gemini-model-unavailable` sin remap |
| Timeout secs | `resolve_client_timeout_secs(client, gemini, agy)`; `client_timeout_secs` pasa `SDDIA_AGY_TIMEOUT_SECS`. Si agy unset, no cambia default 120 (chat intacto). |
| WUI | `interfaces/kalma2/app.js` cero diff |

## 4. Starter-kit

Comentar en ambos `.env.example` del starter-kit:

```text
# SDDIA_AGY_EFFORT=      # high|medium|low; vacío = handler default high (Aiúa)
# SDDIA_AGY_TIMEOUT_SECS= # segundos; vacío = print_timeout 300s / no altera default 120 del bridge
```

Cero slug de modelo.

## 5. Tests

- `execute-process --lib -- aiua_stimulus`: preface intactos; lab-mock `lab-mock-agy:`; grep Kalma2; effort inválido si helper extraído.
- `kalma2-bridge`: sanitize auth/timeout; 503 intacto; timeout max con agy; `app.js` grep intacto.

## 6. Fuera

Mayeuta. Crates HTTP/CLI. `--print`. Kitchen router. Anatomía motora. Skip-permissions.
