---
feature_name: kaizen-cierre-documental-single-pr
created: "2026-05-22"
process: feature
base: main
scope: kaizen-cierre-documental-single-pr
version_spec: "1.0.0"
---

# Especificación — Kaizen cierre documental un solo PR

## Hito 1 — Norma `features-documentation-pattern` v1.2.0

- Sustituir § «Validación en dos fases» por **«Validación en fase única (pre-merge)»**.
- Campos `merged_pr`, `merge_commit`, `closed`: **opcionales**, no gate de cierre.
- Añadir § «Trazabilidad de merge»: GitHub PR / `git merge-base` fuera del frontmatter obligatorio.
- Deprecar explícitamente Fase B v1.1.0 (nota de migración).

## Hito 2 — Procesos `bug-fix.md` y `feature.md`

- Eliminar o reemplazar fase YAML **«Cierre documental post-merge»** por **«Cierre documental en rama»**:
  - Mover PBI a `done/` en la rama del PR.
  - `validacion.md` con `pbi_archived: true` y `pr_url` si existe.
- Actualizar § Done operativo (texto alineado a clarify §2.3).

## Hito 3 — Regla `.cursor/rules/task-closure-documental.mdc`

- `Done` = un PR; sin push documental a `main`.
- Eliminar pasos 1–3 post-merge; sustituir por checklist pre-merge en rama.

## Hito 4 — Gate opcional `verify-task-closure.py`

| Regla | Acción |
|-------|--------|
| Rama `feat/*` / `fix/*` con `validacion.md` exigiendo `merged_pr` no nulo pre-merge | WARN (migración) |
| `pbi_archived: true` sin fichero en `docs/todos/done/` | FAIL |
| PBI solo en `pending/` y `global: APTO` en validacion | FAIL |

Integrar en `pull-request-review` triaje documental (evaluar acoplamiento mínimo).

## Hito 5 — Validación de esta feature (meta)

- Entregar **un único PR** que incluya norma + procesos + regla + PBI en `done/` + `validacion.md` sin `merged_pr` obligatorio.
- Demostrar que no se requiere PR #N+1 `docs/cerrar-pbi-*`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `features-documentation-pattern` v1.2.0 sin Fase B obligatoria |
| CA2 | `bug-fix.md` / `feature.md` § Done un PR |
| CA3 | Regla Cursor actualizada |
| CA4 | PBI Kaizen en `done/` en el mismo PR que la norma |
| CA5 | `validacion.md` de esta feature con `pbi_archived: true`, sin `merged_pr` requerido |
