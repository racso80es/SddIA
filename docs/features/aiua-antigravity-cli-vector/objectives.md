---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
branch_name: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
pbi_ref: docs/todos/pending/[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli.md
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
pbi_uuid: "d15e82ed-ccad-4e5a-80ea-c6e808afa086"
---

# Objetivos — aiua-antigravity-cli-vector

## Misión

Laudar **H-AIUA-VECTOR**: el latido `aiua-stimulus-processing` combustiona vía `skill:antigravity-cli-executor` (`agy` headless), no vía `tool:gemini-http-infer`. Identidad = `aiua_core.md`. Modelo y effort = bóveda de instancia. Mandato biológico: Gemini 3.8 Flash High vía env + `--effort high`, cero slug en Core.

## Alcance

1. Mutar genoma del proceso vía `entity-manager` (DA-2): `Combustion-Inferencia` → `skill:antigravity-cli-executor`; input `effort`; cuerpo sin «inferencia Gemini».
2. Handler nativo: `invoke_capsule_json`, resolución effort/timeout, telemetría `result.usage`, lab-mock con binario de la skill.
3. Epidermis `kalma2-bridge`: sanitizar auth/timeout CLI; alinear timeout si `SDDIA_AGY_TIMEOUT_SECS` está set. `interfaces/kalma2/app.js` intacto.
4. Starter-kit: `SDDIA_AGY_EFFORT` y `SDDIA_AGY_TIMEOUT_SECS` comentadas, vacías, sin slug.
5. Un PR. `validacion.md` APTO solo con CI verde. `accept-pr` post-CI.

## Fuera de gate

Chat Mayeuta. Rebind `llm:interact`. Mutar crates HTTP/CLI. Router kitchen. Anatomía motora. Restaurar `--print`. Red Google en CI. Skip-permissions para Aiúa.

## Ley aplicada

- PBI `PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR` v1.2.0.
- `features-documentation-pattern` v1.2.1: un PR; `global: APTO` solo con `run_id` verde.
- DA-2: process vía `entity-manager`. DA-5: cero retry. DA-6: un finding CI → un parche → un push.
- L-MODEL: slug solo en bóveda.

## Criterios

| ID | Criterio |
|----|----------|
| CA1 | Genoma: `skill:antigravity-cli-executor`; cero `tool:gemini-http-infer` en ese process. UUID inmutable. |
| CA2 | Handler: `invoke_capsule_json` + `parameters.effort` siempre. |
| CA3 | `telemetry.tokens` ← `result.usage` si objeto no vacío. |
| CA4 | Lab-mock: `lab-mock-agy:`; copia skill, no HTTP. |
| CA5 | Bridge sanitiza auth/timeout CLI. Tests herméticos. |
| CA6 | Cero slug `gemini-3.8-flash` en Rust/genoma. |
| CA7 | Argos HTTP intacto. |
| CA8 | Ceguera Kalma2 + `app.js` intacto. |
| CA9 | Live instancia (no gate CI). |
| CA10 | PBI en `done/` + `pbi_archived: true` en el PR. |
| CA-CI | Checks GitHub del PR verdes (`run_id`). |
