---
feature_name: revision-gestion-eventos-kaizen
created: "2026-05-23"
process: bug-fix
branch_name: fix/revision-gestion-eventos-kaizen
persist_ref: docs/fixes/revision-gestion-eventos-kaizen
related_incident: "Eventos PullRequest_Presented #30/#31 en dead-letter tras kaizen single-PR; padres duplicados pending+dead-letter"
pbi_ref: docs/todos/pending/revision_de_gestion_eventos_pr_merge_despues_de_kaicen_evitar_2_PR
---

# Objetivos — revision-gestion-eventos-kaizen

## Misión

Analizar y corregir la gestión EDA de eventos que quedaron en estado erróneo tras el kaizen de **cierre documental en un solo PR** (PR #34). Los síntomas observados incluyen eventos `PullRequest_Presented` de PRs del flujo post-merge obsoleto (#30, #31) con fallo en `argos.pull-request-review` y copias del padre simultáneas en `pending/` y `dead-letter/`.

## Hallazgos iniciales (pre-Dedalo)

| Evento | PR | Rama | Estado |
|--------|-----|------|--------|
| `19d44586-…` | #30 | `feat/kaizen-cierre-documental-post-merge` | dead-letter + pending |
| `fe567363-…` | #31 | `docs/cerrar-pbi-kaizen-pr30` | dead-letter + pending |

Suscriptor fallido en ambos: `argos.pull-request-review` — `error_trace`: *«aduana bloqueó materialización»* (rama ya fusionada / flujo docs post-merge incompatible con single-PR).

Referencia cruzada: `docs/fixes/pr-review-verify-integrity-false-negative/eda-retroactive-manifest.json` (PRs #30/#31 documentados con `route_presented_note`).

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Diagnóstico raíz | `spec.md` identifica si el error es residual (PRs pre-kaizen), regresión del bus, o gap en cierre pending/dead-letter |
| **O2** | Higiene bus | Tras fix: padres `#30`/`#31` sin duplicado pending+dead-letter; estado terminal coherente |
| **O3** | Compatibilidad single-PR | Nuevos cierres con `pbi_archived: true` en rama no generan eventos huérfanos ni dead-letter evitable |
| **O4** | Retroactivo documentado | Manifiesto o procedimiento para eventos legacy post-merge en `validacion.md` |
| **O5** | Regresión | Smoke EDA (emit → watcher → sweep) sin padres stale tras enrutamiento OK |

## No objetivos

- Reabrir el debate del kaizen single-PR (ya cerrado en #34).
- Re-merge de PRs #30/#31 (ya fusionados; solo higiene EDA).
- Refactor global del bus ni cambio de contrato ECST.

## Ley aplicada

- Proceso `bug-fix` v1.4.0
- `features-documentation-pattern` v1.2.0 (cierre documental en rama)
- `SddIA/events/events-contract.md` §4 (ciclo de vida V3+)
- `.cursor/rules/task-closure-documental.mdc`
