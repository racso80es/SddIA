---
feature_name: process-creator-full-contract-forge
created: "2026-08-16"
process: bug-fix
branch_name: fix/process-creator-full-contract-forge
persist_ref: docs/fixes/process-creator-full-contract-forge
pbi_ref: docs/todos/pending/[FIX] process-creator — materialización contractual completa (EV-AUD-003).md
---

# Objetivos — process-creator-full-contract-forge

## Misión

EV-AUD-003: process-creator v1.2.0 / run_process_forge materializa stub Fase inicial y omite inputs, outputs, workspace_template y process_phases solicitados. Hash no coincide con artefacto. Segregado de ola heartbeat 20260812 (PR #177). Forja nativa debe persistir el contrato completo.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
