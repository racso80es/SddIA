---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch: fix/kaizen-tqm-single-flight-pbi
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/done/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
global: APTO
pbi_archived: true
uuid: f3e8a1c2-4b5d-4e6f-9a0b-1c2d3e4f5a6b
checks:
  TQM-CA1: APTO
  TQM-CA2: APTO
  TQM-CA3: APTO
  TQM-CA4: APTO
  TQM-CA5: APTO
  TQM-CA6: APTO
  TQM-CA7: APTO
  TQM-CA8: APTO
  TQM-CA9: APTO
  TQM-CA10: APTO
  TQM-CA11: APTO
  TQM-CA12: PENDIENTE_SMOKE
  CARGO_TEST_TASK_QUEUE_MANAGER: APTO
  PBI_ARCHIVED: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs
  - SddIA/events/orchestration/tqm-dispatch-discarded.md
  - SddIA/events/orchestration/index.md
  - SddIA/core/eda-coverage.json
  - docs/fixes/kaizen-tqm-single-flight-pbi/
  - docs/todos/done/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
non_blocking_findings:
  - TQM-CA12 smoke post-merge
---

# Validación — TQM single-flight por PBI

**global: APTO** — 13 tests `task_queue_manager` en verde; clave por `document_id`; proof durable + ECST `TQM_Dispatch_Discarded`; PBI archivado en rama.
