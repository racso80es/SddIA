---
feature_name: tracker-lifecycle-sink
created: "2026-10-23"
process: feature
branch: feat/tracker-lifecycle-sink
branch_name: feat/tracker-lifecycle-sink
persist_ref: docs/features/tracker-lifecycle-sink
pbi_ref: docs/todos/pending/[ARQUITECTURA] HU-003 PBI-05 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
uuid: "9635ef7a-00e5-4183-91a6-6ce68b5071c4"
global: NO_APTO
pbi_archived: false
checks:
  T05-T19: APTO
  T20: NO_APTO
git_changes:
  - SddIA/engine/execute-process/src/engine
  - SddIA/daemons/linear-bridge-watcher/src/main.rs
  - SddIA/process
  - SddIA/norms/execution-contexts.md
  - SddIA/library
  - docs/features/tracker-lifecycle-sink
  - SddIA/evolution/94980ba1-0a1e-4a16-9e8f-5cef4a2da5ce.md
---

# Validación — tracker-lifecycle-sink

T05–T19 están en la rama. El camino `file` queda cubierto por los tests de materialización. El camino `linear` exige `project_slug`.

T20: Core-self ya tiene `provider: linear`. `/home/racso/Proyectos/SddIA_AP/.SddIA/project.md` no está en disco. El PBI no se archiva.
