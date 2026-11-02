---
feature_name: aut-53-accept-pr-hygiene
created: "2026-11-02"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-53-accept-pr-hygiene
pbi_ref: AUT-53
execution_id: "c2dd1bc0-3556-4149-90ed-52cf13d44b8c"
---

# Objetivos — aut-53-accept-pr-hygiene

## Misión

Fix: accept-pr borra la rama remota si el sello aborta la fusión

`accept-pr` de `fix/bridge-presented-tracker-ref` (AUT-25, PR 424, execution `dc1ca12a-24dd-41f8-9eb2-74fceb262eed`) salió 1 en Fusión Soberana y aun así borró la rama remota. GitHub cerró el PR 424 el 2026-10-30T10:37:03Z con `mergedAt` null. `main` quedó en `84c2ee24`. La rama local sobrevivió en `1b170489` porque el worktree la ocupa.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
