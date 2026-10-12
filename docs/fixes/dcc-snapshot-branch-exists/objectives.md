---
feature_name: dcc-snapshot-branch-exists
created: "2026-10-12"
process: bug-fix
branch_name: fix/dcc-snapshot-branch-exists
persist_ref: docs/fixes/dcc-snapshot-branch-exists
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (187079f2d82d).md
execution_id: "2b207e9f-8af2-49ae-94cf-4f2a06f7ac08"
---

# Objetivos — dcc-snapshot-branch-exists

## Misión

Candado sobre parche ya en HEAD (ad968d80): test git-manager checkout create_if_not_exists con refs/heads existente; firma F-DCC-SNAPSHOT-BRANCH-EXISTS en fracture-signatures.json; caso fracture-corpus para sello 187079f2d82d. Mandato DCC-BRANCH-EXISTS-CA1/CA2.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
