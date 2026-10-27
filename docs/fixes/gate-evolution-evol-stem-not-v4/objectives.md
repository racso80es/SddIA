---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
process: bug-fix
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/gate-evolution-evol-stem-not-v4
pbi_ref: AUT-9
execution_id: "5fb64f21-06f9-4e20-af62-a81d6de226da"
---

# Objetivos — gate-evolution-evol-stem-not-v4

## Misión

Stem de evolution no v4: gate-evolution debe emitir finding de stem/UUID v4, no EVOL_MATERIAL_UNREGISTERED.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
