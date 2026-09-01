---
feature_name: dcc-pr-title-metachar-451dc8707819
created: "2026-09-01"
process: bug-fix
branch_name: fix/dcc-pr-title-metachar-451dc8707819
persist_ref: docs/fixes/dcc-pr-title-metachar-451dc8707819
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (451dc8707819).md
execution_id: "3326bf22-765a-4305-8fdf-a200b23cad10"
---

# Objetivos — dcc-pr-title-metachar-451dc8707819

## Misión

PBI-FIX-FRACTURE-451dc8707819 uuid bc16d090-2f7c-4845-8134-032989b094dc v1.1.0. Apertura en forja colapsa: [PR_BODY_METACHAR] arguments[3] contains forbidden shell metacharacters. arguments[3] es pr_title, no body. Specimen: pr_title con '>' (feat: kaizen CI — steps >1 min). F1: preflight argv + saneo determinista de titulo (no relajar allowlist; no --title-file). F2: error_code PR_TITLE_METACHAR

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
