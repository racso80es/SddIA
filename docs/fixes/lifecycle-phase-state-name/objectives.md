---
feature_name: lifecycle-phase-state-name
created: "2026-11-06"
process: bug-fix
branch_name: fix/lifecycle-phase-state-name
persist_ref: docs/fixes/lifecycle-phase-state-name
execution_id: "090e6ae4-6227-4ab7-b25c-64dab6741fe4"
---

# Objetivos — lifecycle-phase-state-name

## Misión

lifecycle_phase_action.envía update_issue_state con state_key canónico (in_review). La cápsula Linear exige state_name (In Review) o state_id. tracker-stamp y tracker-sync-replay ya traducen via state_map. DCC AUT-76: LINEAR_GRAPHQL_ERROR request.state_id obligatorio. Extraer helper único; no enseñar state_key al adaptador.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
