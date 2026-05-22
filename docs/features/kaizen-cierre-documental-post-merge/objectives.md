---
feature_name: kaizen-cierre-documental-post-merge
created: "2026-05-22"
process: feature
branch_name: feat/kaizen-cierre-documental-post-merge
persist_ref: docs/features/kaizen-cierre-documental-post-merge
pbi_ref: docs/todos/pending/[Kaizen] cierre documental post-merge — norma bug-fix, validacion.md y regla Cursor.md
---

# Objetivos — Kaizen cierre documental post-merge

## Misión

Evitar que el cierre de tareas (`bug-fix`, `feature`, `refactorization`) quede incompleto tras el merge: PBI sin archivar y `validacion.md` sin datos de merge commiteados.

## Objetivos

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Norma proceso | `bug-fix.md` v1.3.0 documenta fase post-merge obligatoria |
| **O2** | Patrón documental | `features-documentation-pattern` v1.1.0 define validación pre/post merge |
| **O4** | Regla operador IA | `.cursor/rules/task-closure-documental.mdc` — merge ≠ done |

## No objetivos (fases posteriores)

- Opción 3: checklist en plantillas PBI (futuro)
- Opción 5–6: cápsula `archive-task-pbi` y script `verify-task-closure.py`

## Ley aplicada

- `features-documentation-pattern`
- Proceso `feature` v1.2.0
