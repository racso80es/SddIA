---
feature_name: kaizen-cierre-documental-single-pr
created: "2026-05-22"
process: feature
branch_name: feat/kaizen-cierre-documental-single-pr
persist_ref: docs/features/kaizen-cierre-documental-single-pr
pbi_ref: docs/todos/pending/[Kaizen] cierre documental un solo PR — validacion y PBI sin post-merge.md
---

# Objetivos — Kaizen cierre documental un solo PR

## Misión

Eliminar el **segundo PR documental** obligatorio tras cada fix/feature: `validacion.md` y el PBI deben cerrarse **en la misma rama** que el código, sin campos que solo existen después del merge.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Patrón documental v1.2.0 | `features-documentation-pattern`: fase única de validación; Fase B post-merge revocada o opcional |
| **O2** | PBI pre-merge en `done/` | `pbi_archived: true` en `validacion.md` con PBI ya movido en la rama del PR |
| **O3** | Procesos y Cursor | `bug-fix.md` / `feature.md` + regla `task-closure-documental.mdc` sin exigir push a `main` post-merge |
| **O4** | Gate opcional | `verify-task-closure.py` rechaza `merged_pr` obligatorio vacío en ramas `feat/*` / `fix/*` |

## Relación con Kaizen anterior

La feature `kaizen-cierre-documental-post-merge` (PR #30) introdujo Fase B para visibilidad. Este Kaizen **corrige el efecto colateral** (PR #31, #33) manteniendo trazabilidad vía `pr_url` + git/GitHub.

## No objetivos

- Automatizar escritura de `merged_pr` vía bot en `main` (sustituido por inferencia externa).
- Cambiar soberanía `pre-push` sobre `main`.

## Ley aplicada

- `features-documentation-pattern` (evolución v1.1.0 → v1.2.0)
- Proceso `feature` v1.2.0
