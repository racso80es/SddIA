---
feature_name: dcc-hook-evol-overescalation-0c5268362b9a
created: "2026-08-31"
process: bug-fix
branch_name: fix/dcc-hook-evol-overescalation-0c5268362b9a
persist_ref: docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (0c5268362b9a).md
execution_id: "0b5db925-9f68-4698-9afa-9f68b698f418"
---

# Objetivos — dcc-hook-evol-overescalation-0c5268362b9a

## Misión

F-DCC-HOOK-EVOL-OVERESCALATION: Publicación remota failed por `SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed` escala a System_Fracture_Detected (hueco F4b). F-MAYEUTA-PREPUSH-EVOL-COLLISION: cubo hook clasifica cualquier `pre-push` como recursión y propone reimplementar SDDIA_HOOK_DELIVERY_CLOSE (ya existe). F-DCC-OPERATOR-PUSH-NO-GUARD: DCC operador no exporta HOOK_DELIVERY_CLOSE; is_delete_push testa remote_sha cero (ref nueva) como delete → F4c en primer push. F0 (index.md sin correlato) cerrado en PR #236; fuera de alcance. Prohibido reimplementar SDDIA_HOOK_DELIVERY_CLOSE. Prohibido bypass raw.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
