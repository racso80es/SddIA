---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
process: bug-fix
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: docs/fixes/gate-evolution-evol-stem-not-v4
pbi_ref: AUT-9
document_id: PBI-e90c2357
execution_id: "5fb64f21-06f9-4e20-af62-a81d6de226da"
tracker_ref: AUT-9
---

# Especificación — `EVOL_ID_NOT_UUID_V4` (AUT-9)

## Problema

Un `.md` bajo `SddIA/evolution/` en el diff con stem distinto de UUID v4 no entra en correladores (`is_record_file`). El gate acusa `EVOL_MATERIAL_UNREGISTERED` sobre producto o, si solo toca ese fichero, sale en verde por L-SELF.

## Cambio

En `sddia-evolution-register::verdict`, tras validar correladores v4 del diff y antes del cotejo material:

1. Detectar paths de evolution en el diff (excluir contrato, índice, borrador por nombre o frontmatter).
2. Si el stem no es UUID v4 → `EVOL_ID_NOT_UUID_V4` con el path y mensaje explícito.
3. Documentar el código en `evolution_contract.md` §8.

## Criterios de aceptación (Linear)

| ID | Verificación |
|----|----------------|
| AC-1 | Test `non_v4_stem_in_diff_reports_id_not_uuid_v4` |
| AC-2 | Test `non_v4_stem_precedes_material_unregistered` |
| AC-3 | Test `material_covered` sin cambio |
| AC-4 | Exclusiones vía `is_excluded_evolution_md_fname` / `is_borrador` |
| AC-5 | Fila `EVOL_ID_NOT_UUID_V4` en contrato + tests 1 y 3 |

## Fuera de alcance

Aceptar stems no v4 como correladores; cambiar L-EXCL.
