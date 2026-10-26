---
feature_name: llm-interaction-audit
created: "2026-10-26"
process: feature
tracker_ref: AUT-7
pbi_ref: AUT-7
branch_name: feat/llm-interaction-audit
---

# Clarify — AUT-7

## Contexto

AUT-5 dejó `docs/audits/Interaccones_llm.md` como espécimen manual. AUT-7 exige proceso de dominio, activación por `llm_audit` en frontmatter del ítem en ciclo, y artefacto `{auditsPath}/{tracker_ref}.md`.

## Laudo

- Alta de `process:llm-interaction-audit` vía `entity-manager` (UUID `21d1c95c-e4e3-41cf-b225-c323459b58ca`).
- Handler nativo en `execute-process` + enganche fail-soft de **Fricción** en peaje termodinámico para `feature` / `bug-fix` / `refactorization` fallidos.
- Sin reescritura de `Interaccones_llm.md`. Sin activación por defecto en todo el equipo.
