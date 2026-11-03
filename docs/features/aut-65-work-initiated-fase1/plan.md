---
feature_name: aut-65-work-initiated-fase1
created: "2026-11-03"
phases: motor, contratos, tests
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-65-work-initiated-fase1
pbi_ref: AUT-65
execution_id: "plan-aut-65-20261103"
---

# Plan — aut-65-work-initiated-fase1

## T1 — Motor

Tras el éxito de `run_workspace_init`, llamar a `emit_work_initiated_fail_soft` para `feature`, `bug-fix` y `refactorization`. Quitar la llamada en `maybe_emit_work_initiated_on_tekton_entry` (o dejarla como no-op si ya hubo emisión en la ejecución). Mantener el flag de laboratorio.

## T2 — Contratos y auditoría

Actualizar `feature.md`, `bug-fix.md` y `refactorization.md` por el canal de genoma, y `docs/audits/ciclo-vida-software-sddia.md` §§ 2, 4.3, 6.3 y 8: Mayeuta y Dédalo operan en In Progress.

## T3 — Tests

Invertir `workspace_init_does_not_emit_work_initiated`. Conservar `maybe_emit_work_initiated_only_once` adaptado al nuevo punto. Cubrir parada en Fase 2 con un solo evento. Smoke `run-eda-e2e-lab`.

## Orden

T1 antes de T3. T2 en el mismo PR que T1.
