---
feature_name: eda-redb-07-cutover
created: "2026-10-21"
process: feature
branch_name: feat/eda-redb-07-cutover
persist_ref: docs/features/eda-redb-07-cutover
pbi_ref: docs/todos/done/[ARQUITECTURA] EDA Redb 07 — Cutover a redb y baja de event-sweeper.md
document_id: PBI-EDA-REDB-07-CUTOVER
execution_id: "dd17d95f-33ac-4932-94f2-fad2125d6991"
---

# Plan — eda-redb-07-cutover

1. Cambiar el default y el guard de rutas antes de reescribir el cúmulo.
2. Sellar la baja con `entity-manager` mientras el crate aún existe.
3. Retirar crate, `event-sweeper.sh`, `event-sweeper.bat` y la unidad `sddia-event-sweeper@.service` de `start-sddia.sh`.
4. Quitar el paso `event-sweeper --once` del job `eda-bus-e2e-smoke`.
