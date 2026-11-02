---
feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
execution_id: "00df8b64-e5b1-42e0-be0f-5d684f7eefc9"
---

# Objetivos — aut-52-refine-fracture-route

## Misión

[FIX] refine-item — fractura sistémica al sellar AUT-25

Al reentrar el sellado de `refine-item` sobre AUT-25 (execution `ffd581d2-5f0c-4df5-b56b-13563b3bd50a`, HEAD `84c2ee24`), el proceso colapsa con `Database already open. Cannot acquire lock.` sobre `.SddIA/runtime/events.redb` (~3041 ms, `success: false`). En esa pasada no se depositó `System_Fracture_Detected` (la emisión abre el mismo store).

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
