---
feature_name: aiua-audit-findings
created: "2026-09-09"
process: feature
branch_name: feat/aiua-audit-findings
persist_ref: docs/features/aiua-audit-findings
pbi_ref: docs/todos/pending/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
execution_id: "1532d612-85d6-4b8e-9b9c-6c4979c3cce4"
document_id: PBI-AIUA-AUDIT-FINDINGS-20260908
pbi_uuid: "2fa76082-d3b1-49ed-af8e-0c1c33b7dd44"
---

# Objetivos — aiua-audit-findings

## Misión

Saldar deuda residual de la auditoría live de `nucleo-aiua-tormentosa-motor` (PR #270 APTO): thinking HIGH en `gemini-http-infer`, cuerpos de las tres acciones alineados al handler, prefacio de identidad en el ensamblado, laudo H-LLM-4.

## Alcance

1. **H-LLM-1b:** `gemini-http-infer` acepta `thinking_level` vía `request` o `SDDIA_GEMINI_THINKING_LEVEL`. Vacío = no enviar `thinkingConfig` (default del modelo). Cero slug de modelo en Rust.
2. **H-LLM-2:** `entity-manager` `update` de `invoke-aiua-core`, `retrieve-active-context`, `persist-thought-record` — cuerpo `{name}.md` completo (inputs/outputs/delegación) alineado al handler. UUID inmutable.
3. **H-LLM-3:** Prefacio de identidad en `assembled_prompt` desde frontmatter de `aiua_core.md` (primera persona, Filtro B). Sin HTTP extra.
4. **H-LLM-4:** Laudo en `clarify.md` (no inyectar `CONSTITUTION_CORE.md`).
5. Lab-mock intacto: no exige thinking ni red.
6. Un PR. `validacion.md` APTO solo con CI verde. `accept-pr` post-CI.

## Fuera de gate

Reabrir PR #270. Puente Kalma2. Anclaje IOTA. Agente Tormentosa. `antigravity-cli-executor`. DNS. Retry/backoff 503 Gemini.

## Ley aplicada

- PBI `PBI-AIUA-AUDIT-FINDINGS-20260908`.
- `features-documentation-pattern` v1.2.1: un PR; `global: APTO` solo con `run_id` verde.
- DA-2: `{name}.md` de actions/tools vía `entity-manager`. Crate = delivery de ED ya forjada.
- L-MODEL intacto (`request.model` ≻ `SDDIA_GEMINI_MODEL` ≻ error).
- Constitución §1: frontera leyes/identidad.

## Criterios

| ID | Criterio |
|----|----------|
| CA-1 | Thinking `high` vía env/request; payload `thinkingConfig.thinkingLevel`. Vacío = omitido. Cero slug en Rust. |
| CA-2 | Lab-mock no exige thinking ni red. |
| CA-3 | Tres acciones: cuerpo `{name}.md` completo alineado al handler. UUID inmutable. |
| CA-4 | `assembled_prompt` incluye prefacio de identidad (1ª persona) desde `aiua_core.md`. |
| CA-5 | H-LLM-4 laudo en `clarify.md`. |
| CA-CI | Checks GitHub del PR verdes (`run_id`). |
