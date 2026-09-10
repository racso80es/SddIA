---
feature_name: aiua-agy-empty-response-text
created: "2026-09-10"
process: bug-fix
branch_name: fix/aiua-agy-empty-response-text
persist_ref: docs/fixes/aiua-agy-empty-response-text
pbi_ref: docs/todos/done/[FIX] Aiúa — combustión agy SUCCESS sin texto persistible.md
execution_id: "7310a29a-ad26-4022-81c5-b6bbc9900165"
---

# Objetivos — aiua-agy-empty-response-text

## Misión

Kalma2 WUI `#aiua-pulse`: `[error] response_text obligatorio` tras combustión `agy` SUCCESS. Extraer texto de `raw_response` antes de persistir; vacío real → `agy respuesta vacía`. PBI `PBI-FIX-AIUA-AGY-EMPTY-RESPONSE-TEXT`.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
