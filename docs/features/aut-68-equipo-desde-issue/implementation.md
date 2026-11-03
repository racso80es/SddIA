---
feature_name: aut-68-equipo-desde-issue
process: refactorization
created: "2026-11-03"
pbi_ref: AUT-68
---

# Implementación — AUT-68

## Cambios

| Área | Detalle |
| --- | --- |
| `linear-tracker-adapter` | `resolve_issue_team_key`; `update_issue_state` y `create_issue` (con padre) usan el equipo del issue/padre, no el `team_key` de la petición para estados y labels |
| Match | `name_eq_ignore_case` en `resolve_workflow_state_id` y `resolve_label_ids` |
| Errores | `LINEAR_LABEL_UNKNOWN` → `label not found on team {TEAM}: {name}`; `LINEAR_STATE_UNKNOWN` incluye equipo y estado |
| Lab | Catálogo de labels por equipo; validación en `create_issue` inline; `with_lab` resetea store entre tests |

## Archivos

- `SddIA/tools/linear-tracker-adapter/src/main.rs`
