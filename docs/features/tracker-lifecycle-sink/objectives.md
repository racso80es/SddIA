---
feature_name: tracker-lifecycle-sink
process: feature
created: "2026-10-22"
branch: feat/tracker-lifecycle-sink
pbi_ref: docs/todos/pending/[ARQUITECTURA] HU-003 PBI-05 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
---

# Objetivos

Un solo sumidero por provider: `file` escribe `pending/` y `done/`; `linear` escribe el issue. Las tareas T05–T20 de `PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05` salen en esta rama, un commit por tarea.

T20: `.SddIA/project.md` de Core-self ya declara `provider: linear` y `team_key: OSC`. Paciente 0 (`/home/racso/Proyectos/SddIA_AP/.SddIA/project.md`) no está en disco; la tarea no se cierra fabricando ese fichero.
