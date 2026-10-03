---
feature_name: linear-hu-a-02-create-issue
created: "2026-10-03"
process: feature
branch_name: feat/linear-hu-a-02-create-issue
persist_ref: docs/features/linear-hu-a-02-create-issue
pbi_ref: docs/todos/done/[OPERATIVO] Linear HU-A 02 — Cápsula create_issue.md
execution_id: "77c0aa27-ccd0-46da-addb-e8667bbb12f3"
---

# Objetivos — linear-hu-a-02-create-issue

## Misión

PBI-LINEAR-A-02-CREATE-ISSUE: linear-tracker-adapter 1.1.0 operación create_issue (team_key, title, description, labels[], parent_ref, project_id, state_name, priority). Errores LINEAR_LABEL_UNKNOWN, LINEAR_PARENT_NOT_FOUND. Lab mock y tests AC-0a/AC-0a2/AC-9.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
