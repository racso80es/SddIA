---
feature_name: bug-fix-fracture-1d4115c57471
created: "2026-08-29"
process: bug-fix
branch_name: fix/bug-fix-fracture-1d4115c57471
persist_ref: docs/fixes/bug-fix-fracture-1d4115c57471
pbi_ref: docs/todos/pending/[FIX] bug-fix — fractura sistémica (1d4115c57471).md
execution_id: "9b0ac29e-b064-4e87-a41c-ecfd7d66525a"
---

# Objetivos — bug-fix-fracture-1d4115c57471

## Misión

F-DIRTY-WORKTREE (L-DIRTY-INIT) aborta workspace-init correctamente pero emite System_Fracture_Detected y materializa PBI Kintsugi. El abort es higiene, no colapso. Discriminar: no emitir fractura sistémica ante dirty-worktree; conservar abort Err. Evento 1d4115c57471.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
