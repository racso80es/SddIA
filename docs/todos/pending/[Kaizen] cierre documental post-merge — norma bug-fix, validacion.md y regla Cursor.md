---
document_id: PBI-KAIZEN-CIERRE-DOCUMENTAL-POST-MERGE
title: "[Kaizen] cierre documental post-merge — norma bug-fix, validacion.md y regla Cursor"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "abierto"
priority: alta
process: feature
incident_ref: "PR #29 — PBI y validacion.md post-merge quedaron sin commitear"
feature_ref_target: docs/features/kaizen-cierre-documental-post-merge
related:
  - SddIA/process/bug-fix.md
  - SddIA/library/norms/features-documentation-pattern.md
  - .cursor/rules/task-closure-documental.mdc
  - docs/fixes/event-pending-sweeper/
---

# [Kaizen] cierre documental post-merge

## Mandato

Implementar opciones **1**, **2** y **4** del laudo post-incidente PR #29:

| Opción | Entregable |
|--------|------------|
| **1** | `bug-fix.md` § Cierre documental post-merge (obligatorio) |
| **2** | `features-documentation-pattern.md` — `validacion.md` en dos fases |
| **4** | Regla Cursor `.cursor/rules/task-closure-documental.mdc` |

## Criterio de cierre

- [ ] Normas actualizadas y versionadas
- [ ] Regla Cursor activa (`alwaysApply: true`)
- [ ] `validacion.md` de la feature Kaizen con veredicto APTO
- [ ] Este PBI movido a `docs/todos/done/` **en el mismo PR o commit post-merge** (dogfooding)

## Inicio

```json
{
  "process": "feature",
  "feature_name": "kaizen-cierre-documental-post-merge",
  "branch_name": "feat/kaizen-cierre-documental-post-merge",
  "persist_ref": "docs/features/kaizen-cierre-documental-post-merge",
  "base_branch": "main"
}
```
