---
feature_name: argos-validation-on-main
created: "2026-10-23"
process: bug-fix
branch_name: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
pbi_ref: docs/todos/pending/[FIX] Argos sella validacion en main y bloquea el init.md
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
---

# Objetivos — argos-validation-on-main

## Misión

PBI-FIX-ARGOS-VALIDATION-ON-MAIN (f126631d-3d98-43c4-b251-911410e06001). Argos escribió docs/features/tracker-lifecycle-sink/validacion.md en main porque el handoff de Verificación llegó sin branch ni execution_id. Corte: handoff vacío no escribe validacion.md y falla; con branch y ref local ausente, checkout vía git-manager antes del sello. No ampliar el allowlist dirty-worktree. T20 y SddIA_AP f

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
