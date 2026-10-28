---
feature_name: bug-fix-done-before-pr-merge
created: "2026-10-28"
process: bug-fix
branch_name: fix/bug-fix-done-before-pr-merge
persist_ref: docs/fixes/bug-fix-done-before-pr-merge
tracker_ref: AUT-12
execution_id: "dea619eb-78cc-498c-9747-8cd6e34bb14b"
---

# Objetivos — bug-fix-done-before-pr-merge

## Misión

El cierre de bug-fix no debe transicionar el issue Linear a Done antes de PullRequest_Merged. PullRequest_Presented debe dejar in_review.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
