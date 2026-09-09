---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
purpose: Estabilización Filtro A PBI combustión Aiúa vía antigravity-cli
version_clarify: "1.0.0"
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
pbi_ref: docs/todos/pending/[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli.md
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
pbi_uuid: "d15e82ed-ccad-4e5a-80ea-c6e808afa086"
---

# Clarificación — aiua-antigravity-cli-vector

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `e35155cd-fdfa-4612-92a4-121e12b4c3f9`. Rama `feat/aiua-antigravity-cli-vector`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| L-VECTOR | Combustión Aiúa = `skill:antigravity-cli-executor`. `tool:gemini-http-infer` sale de `aiua-stimulus-processing`. Argos HTTP intacto. |
| L-ARGV | Skill vigente: `--output-format json` + `-p`. **No** emitir `--print`. Genoma skill.md desfasado = deuda fuera. |
| L-EFFORT | `parameters.effort` **siempre**. Cascada: `inputs.effort` ≻ `SDDIA_AGY_EFFORT` ≻ `SDDIA_GEMINI_THINKING_LEVEL` ≻ `high`. Normalizar `low\|medium\|high` case-insensitive. Inválido → error sin spawn. |
| L-PAYLOAD | Shape skill: `request.prompt` + `request.parameters.{model,effort,print_timeout}`. **No** `request.model` (shape HTTP). |
| L-TIMEOUT | `SDDIA_AGY_TIMEOUT_SECS` es **nueva**. Skill no la lee. Handler inyecta `print_timeout` `{N}s`. Bridge: `max(client, gemini_http, agy)` si agy>0. **No** subir default 120 de `/api/chat`. Default print_timeout handler: client ≻ `"300s"`. |
| L-USAGE | `telemetry.tokens` ← `result.usage` si objeto no vacío. Cero `usageMetadata`. Cero schema de cuatro claves. Lab-mock `usage: {}` → omitir tokens. |
| L-MODEL-TEL | Skill no emite `result.model`. `telemetry.model` = modelo resuelto en handler. |
| L-FORGE | EM `update` ×2. (1) `process_phases` + `process_inputs` + `process_version: 1.1.0`. (2) `markdown_body_replacements`. UUID `6c595785-e386-402f-b570-0b2aa6343051` inmutable. No reenviar `process_contract_version` default 1.3.0. |
| L-SANITIZE | Auth CLI ≠ 503 HTTP. Mensajes castellanos propios. No reusar `AIUA_PROVIDER_UNAVAILABLE_MSG`. No enmascarar `timeout motor` ni `gemini-model-unavailable`. |
| L-NO-SLUG | Cero `gemini-3.8-flash` en Rust/genoma. Starter-kit: claves AGY comentadas vacías. |
| L-SKIP-PERM | `SDDIA_AGY_ALLOW_SKIP_PERMISSIONS` no se activa para el latido. Default `--sandbox`. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- Chat Mayeuta / `llm:interact` / `kalma2-interact`.
- Mutar crates `gemini-http-infer` o `antigravity-cli-executor`.
- Restaurar `--print` en la skill.
- Router `PBI-MULTI-LLM-ROUTER`.
- Anatomía motora / function calling.
- Retry / sleep / polling (DA-5).
- `GEMINI_API_KEY` como credencial de `agy`.
