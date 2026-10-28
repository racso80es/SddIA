---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
process: feature
branch_name: feat/interaction-audit-session-karma2
persist_ref: docs/features/interaction-audit-session-karma2
pbi_ref: AUT-10
tracker_ref: AUT-10
execution_id: "c924b989-968c-434d-a9ad-5cb5edc2ae12"
related_feature: docs/features/llm-interaction-audit
---

# Objetivos — interaction-audit-session-karma2

## Misión

Cumplir kaizen AUT-10: alinear la auditoría de sesión LLM (`docs/audits/{tracker_ref}.md`) al contrato `interaction_audit` de `karma2-token`, reutilizando el proceso `llm-interaction-audit` y sin mutar el contrato del token.

## Fricción

- Tabla narrativa heredada (AUT-5) sin cuatro campos por interacción.
- Timestamps a minuto; `invoked_by` / `entity_id` en bloque de sesión.
- Typo histórico `Interaccones_llm.md`; uuids de ejemplo no v4.

## Propuesta

- Filas de interacción con `entity_type`, `entity_id`, `invoked_by`, `timestamp` (traza real).
- Alta de artefacto `docs/audits/AUT-10.md` con frontmatter v4 y tabla conforme.
- Ajuste mínimo del handler nativo si la tabla actual no serializa los cuatro campos.

## Métrica de mejora

Un estímulo registrado con `llm_audit: active` solo es APTO si los cuatro campos están presentes y válidos; cero reescritura del cierre AUT-5; cero diff en `karma2-token.md`.

## Criterios de aceptación

1. `record_interaction` (o equivalente) persiste los cuatro campos cuando el ítem tiene `llm_audit: active`.
2. Smoke manual: una invocación `./sddia-run.sh` fallida/exitosa deja fila conforme en `docs/audits/AUT-10.md`.
3. Tests Rust del módulo `llm_interaction_audit` actualizados y verdes.
4. `validacion.md` global APTO en rama `feat/interaction-audit-session-karma2`.

## Ley aplicada

- Git vía `skill:git-manager`; genoma vía procesos/cápsulas cuando aplique.
- Cierre: un PR con documentación en `persist_ref`.
