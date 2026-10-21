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

# Implementation — eda-redb-07-cutover

`entity-manager` delete de `event-sweeper`: `execution_id` `bea7b76d-a3fc-4e38-bd91-dc16f9fa02f9`, `Domain_Entity_Deleted` `4431ddad-3f03-441f-872e-60ebe5e3af16`. El índice queda en 7 centinelas. Evolución `36588064-c783-4b1b-933c-bb254fbb10ff`.

Un binario release previo de `event-sweeper` materializó `.SddIA/runtime/events.redb` como directorio al leer el cúmulo nuevo. Se detuvo ese proceso y el `event-watcher` release hermano antes del sello. El store resultante es un fichero `redb`.
