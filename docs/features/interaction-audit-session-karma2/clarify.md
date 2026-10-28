---
feature_name: interaction-audit-session-karma2
created: "2026-10-28"
process: feature
branch_name: feat/interaction-audit-session-karma2
persist_ref: docs/features/interaction-audit-session-karma2
pbi_ref: AUT-10
tracker_ref: AUT-10
execution_id: "c924b989-968c-434d-a9ad-5cb5edc2ae12"
---

# Clarify — AUT-10

## Contexto

Kaizen hijo de AUT-5 / proceso `llm-interaction-audit` (AUT-7). El peaje termodinámico y `record_interaction` ya existen; el gap es **forma** del artefacto `docs/audits/{tracker_ref}.md` frente a `karma2-token` § `interaction_audit`.

## Laudo

- Extender escritura de auditoría (handler `llm-interaction-audit` / `record_interaction`) para que cada interacción persista `entity_type`, `entity_id`, `invoked_by`, `timestamp` (ISO de traza, no minuto redondeado).
- `entity_type` ∈ {skill, tool, action, process}.
- Frontmatter del artefacto: `uuid` v4 real; nombre de fichero `{tracker_ref}.md` (sin typo `Interaccones`).
- **Fuera:** reescribir `docs/audits/Interaccones_llm.md`; ampliar schema de `karma2-token`; cambiar contrato ECST del token.

## Dependencias

- SSOT contrato: `SddIA/tokens/karma2-token.md`.
- Proceso existente: `llm-interaction-audit` + frontmatter `llm_audit: active` en ítem en ciclo (AUT-10 ya lo lleva en Linear).
