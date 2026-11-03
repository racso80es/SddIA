---

feature_name: aut-68-equipo-desde-issue
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-68-equipo-desde-issue
pbi_ref: AUT-68
execution_id: "plan-aut-68-20261103"
---
# Objetivos — aut-68-equipo-desde-issue

## Misión

Dejar de filtrar workflow y etiquetas de Linear por el `team_key` del manifiesto cuando el issue (o su padre) pertenece a otro equipo del mismo workspace.

## Alcance

- `SddIA/tools/linear-tracker-adapter`: resolución de equipo, `resolve_workflow_state_id`, `resolve_label_ids`.
- Callers que hoy inyectan `team_key` de `.SddIA/project.md` en `update_issue_state` y sellos de labels: el adapter ignora ese filtro si hay issue o padre.
- Comparación de nombres sin distinguir mayúsculas.
- Error de label ausente con equipo y nombre.

## Fuera de alcance

- Cambiar `tracker.team_key` del manifiesto.
- Crear labels que falten en un equipo.
- Unificar el catálogo de labels entre AUT y MAN.
