---
feature_name: llm-interaction-audit
created: "2026-10-26"
process: feature
branch_name: feat/llm-interaction-audit
persist_ref: docs/features/llm-interaction-audit
pbi_ref: AUT-7
tracker_ref: AUT-7
execution_id: "c858e7f9-76c1-4637-a618-c33a5aac597a"
---

# Objetivos — llm-interaction-audit

## Misión

Proceso de dominio llm-interaction-audit (entity-manager). Campo llm_audit active|inactive en frontmatter del ítem en ciclo. Artefacto {auditsPath}/{tracker_ref}.md con tabla de interacciones y sección Fricción. No reescribir Interaccones_llm.md. Criterios en issue AUT-7.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
