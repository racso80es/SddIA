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

# Clarify — eda-redb-07-cutover

Las sondas `daemon-heartbeat-audit` y `system-vitality-probe` ya viven en el tick de `event-watcher` (PBI 03). Este corte no las mueve otra vez.

`filesystem` y `dual` siguen usando `./.events/` como cola. Esas rutas ya no salen de `cumulo.paths.json`: el código las conserva como default cuando el valor del cúmulo es un nombre de tabla o un fichero `*.redb`.

No hay migración de JSON históricos. Un residuo en `pending/` con el default `redb` es fractura `F-EDA-RESIDUAL-FILESYSTEM`.
