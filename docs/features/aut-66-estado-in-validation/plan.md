---
feature_name: aut-66-estado-in-validation
created: "2026-11-03"
phases: mapa, evento, stamp, docs, tests
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-66-estado-in-validation
pbi_ref: AUT-66
execution_id: "plan-aut-66-20261103"
---

# Plan — aut-66-estado-in-validation

## T1 — Mapa y Linear

Crear el estado de workflow `In Validation` en el equipo AUT, entre In Progress e In Review. Añadir `in_validation: In Validation` en `.SddIA/project.md` → `tracker.state_map`. Constante `COLUMN_IN_VALIDATION` en el frontmatter de ítems.

## T2 — Rango

En `tracker_sync_replay::state_rank`: `in_validation = 3`, `in_review = 4`, `done = 5`. Actualizar `state_rank_orders_cycle`.

## T3 — Disparador

Tras fase Tekton con `status` ejecutado (no `failed` ni `simulated` de laboratorio que no cerró código), emitir `Validation_Started` una sola vez por ejecución. Registrar la clase en el catálogo EDA y suscribir `tracker-stamp`.

## T4 — Sello

`tracker-stamp`: rama `Validation_Started` → `update_issue_state` canónico `in_validation` + comentario. File sink: `tracker_column: in_validation` en el markdown del ítem, misma carpeta.

## T5 — Contratos y auditoría

Bullets en `feature.md`, `bug-fix.md`, `refactorization.md`. Tablas y diagrama de `docs/audits/ciclo-vida-software-sddia.md`.

## T6 — Tests

- Rango: `in_progress < in_validation < in_review`.
- Descarte `in_validation` → `in_progress`.
- Sello file y mock Linear de `Validation_Started`.
- `PullRequest_Presented` sigue pudiendo subir desde `in_validation`.

## Orden

T1 y T2 antes de T4. T3 antes de T6. T5 en el mismo PR.
