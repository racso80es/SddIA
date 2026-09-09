---
feature_name: aiua-audit-findings
created: "2026-09-09"
process: feature
purpose: Estabilización Filtro A PBI hallazgos auditoría live Aiúa
version_clarify: "1.0.0"
execution_id: "1532d612-85d6-4b8e-9b9c-6c4979c3cce4"
pbi_ref: docs/todos/pending/[KAIZEN] Aiúa — hallazgos auditoría live y thinking HIGH.md
document_id: PBI-AIUA-AUDIT-FINDINGS-20260908
pbi_uuid: "2fa76082-d3b1-49ed-af8e-0c1c33b7dd44"
---

# Clarificación — aiua-audit-findings

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `1532d612-85d6-4b8e-9b9c-6c4979c3cce4`. Rama `feat/aiua-audit-findings`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| L-THINK | `request.thinking_level` (alias `thinkingLevel`) ≻ `SDDIA_GEMINI_THINKING_LEVEL` ≻ omitir. Normalizar a `HIGH`\|`MEDIUM`\|`LOW`. Valor inválido → error. Vacío/ausente → no se envía `thinkingConfig` (default del modelo = `medium`). |
| L-NO-SLUG | Cero model id en Rust ni como valor-ley en `.env.example`. L-MODEL intacto. |
| L-GENCFG | `temperature` y `thinkingConfig` conviven en el mismo `generationConfig`. No clobber. Lab-mock no construye payload HTTP. |
| L-FORGE-ACTION | EM `update` con `action_body` + I/O (`patch_action_content_update`). UUID inmutable. Prohibido `update` genérico de create-template. |
| L-FORGE-TOOL | Crate = delivery de ED ya forjada. Tool `{name}.md`: EM `markdown_body_replacements` **no** existe en `run_tool_forge`. Cuerpo se sella con `hash_refresh_only` **solo si** se toca el `.md`; si no, crate + starter-kit. Prohibido `update` genérico (regenera UUID `7a8da3ad-…`). |
| L-PREFACE | Prefacio **antes** del genoma, derivado del frontmatter de `aiua_core.md` (`name`, `entity_type`). Sin HTTP. Fallback si falta `name`: «Eres la Aiúa del ecosistema SddIA. Hablas en primera persona.» Cero literal `Tormentosa` en Rust. |
| L-CONST | **No** inyectar `CONSTITUTION_CORE.md` en `assembled_prompt`. Constitución §1: leyes vs identidad. Inyectar la Constitución en cada latido es ruido (Filtro C) y mezcla jurisdicción. |
| L-EM-REVOKED | `entity-manager` ∈ revoked instancia. Forja con relay lab; si Cerbero aborta → stop (DA-2). |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- Vector de combustión = `gemini-http-infer`. No `antigravity-cli-executor`.
- 503 Gemini ≠ CA de Core. Sin retry en handler.
- PR #270 / `validacion.md` APTO del arranque: no reabrir.
- `gemini-http-infer` no tiene `system_prompt`/`user_prompt`.
