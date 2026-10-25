---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
branch_name: fix/ci-integrity-protoc-jobs
persist_ref: docs/fixes/ci-integrity-protoc-jobs
tracker_ref: AUT-4
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
---

# Objetivos — ci-integrity-protoc-jobs

## Misión

AUT-4. [BUG] Pipeline: cápsulas ausentes y protoc fuera del job E2E.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
