---
feature_name: linear-hu-a-01-contract
created: "2026-10-03"
process: feature
branch_name: feat/linear-hu-a-01-contract
persist_ref: docs/features/linear-hu-a-01-contract
pbi_ref: docs/todos/done/[OPERATIVO] Linear HU-A 01 — Contrato de proyecto tracker 1.3.0.md
execution_id: "fc2e58ed-7306-4092-8ad0-e79ada0c9243"
---

# Objetivos — linear-hu-a-01-contract

## Misión

Bump project-config-contract 1.2.0 a 1.3.0: tracker.state_map.todo opcional; tracker.labels fix, kaizen, deuda, spike, editable; tracker.done_gate git|linear|both default git con validador que solo acepta git en esta entrega. project_binding admite 1.3.0 y valida nuevos campos. Tests AC-1 del PBI.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
