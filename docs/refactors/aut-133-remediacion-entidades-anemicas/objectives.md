---
feature_name: aut-133-remediacion-entidades-anemicas
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-133-remediacion-entidades-anemicas
persist_ref: docs/refactors/aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
historia_ref: AUT-125
pbi_ref: PBI-81e91c67
execution_id: "5ea79c56-a610-4223-b2b3-7498a510d239"
---
# Objetivos — AUT-133

Liquidar la deuda documental residual del genoma anémico (residuo AUT-125 / PR #480 REJECT): inventario pre-remediación trazable, remediación con contenido real vía `entity-manager`, política explícita de UUID no v4 y promoción del CI de `--if-touched` a gate global solo con `violation_count: 0` sin plantilla.

| ID | Objetivo |
| --- | --- |
| O-1 | Inventario pre-remediación versionado con JSON del gate global sobre `main` (AC-1 / KA-DOC-1); nunca solo post-remediación vacío |
| O-2 | Remediación entidad-a-entidad con cuerpo derivado de implementación real vía `entity-manager` (AC-2 / KA-DOC-2); sin plantilla compartida |
| O-3 | UUID no v4: migración explícita (alias + índices + `eda-coverage`) o excepción formal auditable (AC-3 / KA-UUID-1) |
| O-4 | Promoción CI a gate global cuando `violation_count` sea `0` sin plantilla (AC-4 / KA-GATE-GLOBAL-1) |
| O-5 | Un solo PBI de deuda de contenido (D5): inventario + remediación + UUID + promoción CI en esta `refactorization` |

## Alcance (manifiesto)

Estabilización Mayeuta → diseño Dedalo → ejecución Tekton → verificación Argos. Git solo vía `skill:git-manager`. Fuera: AUT-131 (forjas), AUT-132 (motor del peaje), rediseño de contratos de familia, scripts ad-hoc bajo `docs/`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Documentación: `features-documentation-pattern` (un `.md` por fase, frontmatter YAML).
- KM / `docs/todos/`: solo `agent:cumulo` o `Kaizen_Alert_Required` (Mayeuta no siembra TODOs).
- Mutaciones de entidades: `entity-manager` / proceso autorizado + evolution cuando corresponda (DA-2).
