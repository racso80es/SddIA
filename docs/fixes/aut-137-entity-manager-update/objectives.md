---
feature_name: aut-137-entity-manager-update
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-137-entity-manager-update
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-137-entity-manager-update
pbi_ref: AUT-137
execution_id: "727f8de1-4875-4cb3-a7e7-f11cf64cc9a0"
---

# Objetivos — aut-137-entity-manager-update

## Misión

[FIX] entity-manager update: BOM, contract sin SemVer, migración UUID legacy y suite sin markdown_body_replacements

La remediación de entidades vía `./sddia-run.sh --process entity-manager` (DA-2) queda bloqueada por **cuatro defectos empíricos** en el motor de update/forja. Origen: bloqueantes de AUT-133 (Lote 1 T4; familia action: 1/4 acciones remediadas; gate 322→321). Sin estos canales operativos no se puede avanzar la remediación de ~141 entidades anémicas sin mutación manual de genoma.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
