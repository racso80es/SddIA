---
feature_name: aut-64-sellado-tracker-origen
created: "2026-11-03"
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-64-sellado-tracker-origen
pbi_ref: AUT-64
---

# Objetivos — AUT-64

## Norte

Atomicidad entre descripción sellada en Linear y transición de columna + comentario, eliminando la carrera EDA `forge/refine` → `tracker-stamp`.

## Entregables

- Funciones `seal_refined_todo_at_origin` / `seal_forged_backlog_at_origin` en el motor.
- Cableado en forge/refine.
- Catálogo `event-domain-subscriptions.json` sin `tracker-stamp` en los cuatro eventos de alta/refino.
- Tests lab (`tracker_stamp`, `linear_direct_cycle_e2e`) verdes.
