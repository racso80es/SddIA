---
feature_name: dcc-push-non-fast-forward-4c01d65b972f
created: "2026-09-28"
process: bug-fix
branch_name: fix/dcc-push-non-fast-forward-4c01d65b972f
persist_ref: docs/fixes/dcc-push-non-fast-forward-4c01d65b972f
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (4c01d65b972f).md
execution_id: "55be0f6b-b30f-47b4-a8df-5853e2b5d0cf"
---

# Objetivos — dcc-push-non-fast-forward-4c01d65b972f

## Misión

Clasificar push `non-fast-forward` en DCC como `blocked` (`F-DCC-PUSH-NON-FAST-FORWARD`) sin `System_Fracture_Detected`. Mayeuta ya diagnostica vía catálogo; cerrar supresión en motor y documentación de escalado.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
