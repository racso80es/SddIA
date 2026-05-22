---
feature_name: kaizen-cierre-documental-post-merge
created: "2026-05-22"
process: feature
base: main
scope: kaizen-cierre-documental-post-merge
version_spec: "1.0.0"
---

# Especificación — Kaizen cierre documental post-merge

## Hito 1 — `bug-fix.md` v1.3.0 (Opción 1)

Añadir fase **«Cierre documental post-merge»** en frontmatter y sección dedicada:

1. Actualizar `validacion.md`: `merged_pr`, `merge_commit`, `closed`, `pbi_archived: true`.
2. Mover PBI de `docs/todos/pending/` → `docs/todos/done/` (mismo `document_id`).
3. Commit atómico en `main` (o PR de cierre inmediato) **antes** de declarar la tarea terminada.

Definición operativa:

```text
Done = merge en main + validacion.md post-merge + PBI en done/ + commit pusheado
```

## Hito 2 — `features-documentation-pattern` v1.1.0 (Opción 2)

### Tabla `validacion.md` ampliada

| Fase | Campos frontmatter | Cuándo |
|------|-------------------|--------|
| Pre-merge | `global`, `checks`, `git_changes`, `branch` | Argos tras Tekton |
| Post-merge | `merged_pr`, `merge_commit`, `closed`, `pbi_archived` | Tras `accept-pr` / merge |

### Restricción

Prohibido considerar cierre definitivo si `pbi_archived: false` o `merged_pr` ausente tras merge conocido.

## Hito 3 — Regla Cursor (Opción 4)

Archivo: `.cursor/rules/task-closure-documental.mdc`

- `alwaysApply: true`
- Contrapeso a «no commitear sin pedir»: **excepción obligatoria** para cierre documental post-merge.
- Aplica a `bug-fix`, `feature`, `refactorization`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `bug-fix.md` incluye fase post-merge en YAML `phases` |
| CA2 | `features-documentation-pattern` documenta campos post-merge |
| CA3 | Regla Cursor existe y es concisa (< 50 líneas) |
