---
document_id: PBI-KAIZEN-CLOSURE-SINGLE-PR
title: "[Kaizen] Cierre documental un solo PR — validacion y PBI sin post-merge"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: listo_para_merge
priority: alta
process: feature
feature_ref_target: docs/features/kaizen-cierre-documental-single-pr
related:
  - docs/features/kaizen-cierre-documental-post-merge/validacion.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/process/bug-fix.md
  - SddIA/process/feature.md
  - .cursor/rules/task-closure-documental.mdc
  - docs/fixes/pr-review-verify-integrity-false-negative/validacion.md
incident_ref: "PR #32 fix + PR #33 docs — segundo PR obligatorio por Fase B y push bloqueado a main"
---

# [Kaizen] Cierre documental un solo PR

## 0. Mandato

Iniciar como **`feature`** bajo `docs/features/kaizen-cierre-documental-single-pr/`.

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Eliminar** dependencia de Fase B post-merge en `validacion.md` | Patrón v1.2.0: campos `merged_pr` / `merge_commit` / `closed` opcionales o derivados; cierre Argos en rama pre-merge |
| **O2** | **PBI en `done/` en el mismo PR** que el código | `pbi_archived: true` antes del merge; sin commit documental extra a `main` |
| **O3** | **Actualizar** procesos y regla Cursor | `bug-fix.md`, `feature.md`, `task-closure-documental.mdc` alineados |
| **O4** | **Opcional** gate `verify-task-closure.py` | Falla si Fase B requerida en diff o PBI solo en `pending/` al abrir PR |

## 1. Incidente

| Campo | Valor |
|-------|--------|
| Contexto | Kaizen previo (#30) exigió Fase B + push a `main` |
| Síntoma | Segundo PR `docs/*` tras cada fix/feature (#31, #33) |
| Causa | `pre-push` bloquea `main`; `validacion.md` y PBI esperan datos solo conocidos post-merge |

## 2. Diseño objetivo (laudo)

```text
Done = un único PR mergeado en main
     + validacion.md APTO en la rama (sin merged_pr obligatorio)
     + PBI ya en docs/todos/done/ en esa misma rama
     + trazabilidad merge vía git/GitHub (no duplicada en frontmatter)
```

## 3. Proceso de inicio

```json
{
  "process": "feature",
  "feature_name": "kaizen-cierre-documental-single-pr",
  "branch_name": "feat/kaizen-cierre-documental-single-pr",
  "persist_ref": "docs/features/kaizen-cierre-documental-single-pr",
  "refined_requirements": "Kaizen: unificar cierre documental en un PR; validacion.md y PBI sin campos post-merge obligatorios; revocar Fase B del patrón v1.1.0.",
  "pbi_ref": "docs/todos/pending/[Kaizen] cierre documental un solo PR — validacion y PBI sin post-merge.md",
  "base_branch": "main"
}
```

## 4. Criterio de cierre del PBI

- [ ] Argos APTO en `docs/features/kaizen-cierre-documental-single-pr/validacion.md` (pre-merge, un PR).
- [ ] Sin PR `docs/cerrar-pbi-*` posterior para esta feature.
- [ ] Este TODO en `docs/todos/done/` en el **mismo** PR que la norma.
