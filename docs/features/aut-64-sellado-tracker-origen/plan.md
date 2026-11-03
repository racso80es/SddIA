---
feature_name: aut-64-sellado-tracker-origen
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-64-sellado-tracker-origen
pbi_ref: AUT-64
execution_id: "exec-aut-64-20261103"
---

# Plan — AUT-64

## T1 — API de sellado en origen

`seal_refined_todo_at_origin`, `seal_forged_backlog_at_origin` en `tracker_stamp.rs`.

## T2 — Integración procesos

`forge_pbi.rs`, `forge_hu.rs`, `refine_item/run.rs`, `refine.rs`.

## T3 — Genoma EDA

Vaciar suscripciones `tracker-stamp` en los cuatro eventos.

## T4 — Tests

Actualizar tests unitarios y `linear_direct_cycle_e2e` (sin `stamp()` redundante post-forja/refino).
