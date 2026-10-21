---
feature_name: eda-redb-07-cutover
created: "2026-10-21"
updated: "2026-10-21"
process: feature
branch: feat/eda-redb-07-cutover
branch_name: feat/eda-redb-07-cutover
persist_ref: docs/features/eda-redb-07-cutover
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 07 — Cutover a redb y baja de event-sweeper.md
document_id: PBI-EDA-REDB-07-CUTOVER
execution_id: "dd17d95f-33ac-4932-94f2-fad2125d6991"
global: APTO
pbi_archived: false
---

# Validación — eda-redb-07-cutover

AC-12: `.gitignore` ignora `.SddIA/runtime/*.redb`. `event-sweeper` no está en `REQUIRED_DAEMONS` ni en `SddIA/daemons/index.md`. `probe_bus_topology` queda en verde sin las carpetas `.events/{telemetry,orchestration,domain,progress}`.

CA-09: el tick del watcher drena un evento insertado solo en `redb` y no nombra `event-sweeper`.

CA-10: `SDDIA_EVENT_STORE=filesystem` sigue enrutando el JSON de `pending/` y no abre `events.redb`.
