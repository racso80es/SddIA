---
feature_name: aut-69-unificar-etiqueta-fix-bug
created: "2026-11-03"
phases: migrar, manifiesto, borrar, verificar
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-69-unificar-etiqueta-fix-bug
pbi_ref: AUT-69
execution_id: "plan-aut-69-20261103"
---

# Plan — aut-69-unificar-etiqueta-fix-bug

## T1 — Migrar issues

Para cada identifier de la lista del objetivo, leer `labelIds`, añadir el id de `Bug`, quitar el id de `Fix`, y `issueUpdate`. No tocar título, estado ni descripción.

## T2 — Manifiesto

En `.SddIA/project.md`, `tracker.labels.fix: Bug`. Un commit en `refactor/man-26-ciclo-vida-software`.

## T3 — Borrar la label

Repetir el listado de issues AUT con label `Fix`. Si el total es cero, borrar la label `Fix` del equipo AUT. Si queda alguno, no borrar y registrar el identifier.

## T4 — Verificación

- Consulta GraphQL: cero issues AUT con `Fix`; la label `Fix` no está en el equipo.
- El manifiesto contiene `fix: Bug`.
- No se exige un `create_issue` real de prueba en AUT. El criterio es que el valor leído por `tracker_label_for_inputs` sea `Bug`.

## Orden

T1 antes de T3. T2 puede ir en paralelo con T1. T3 es el último paso en Linear.
