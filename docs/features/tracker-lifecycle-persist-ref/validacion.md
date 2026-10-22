---
feature_name: tracker-lifecycle-persist-ref
created: "2026-10-22"
process: feature
branch: feat/tracker-lifecycle-persist-ref
branch_name: feat/tracker-lifecycle-persist-ref
persist_ref: docs/features/tracker-lifecycle-persist-ref
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-04 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
uuid: "c62ed63e-28b5-4a6b-b742-90661afcfed8"
execution_id: "76a499d1-59eb-4b02-8e18-5586b41190f0"
global: APTO
pbi_archived: true
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - docs/features/tracker-lifecycle-persist-ref/objectives.md
  - docs/features/tracker-lifecycle-persist-ref/clarify.md
  - docs/features/tracker-lifecycle-persist-ref/spec.md
  - docs/features/tracker-lifecycle-persist-ref/plan.md
  - docs/features/tracker-lifecycle-persist-ref/implementation.md
  - docs/features/tracker-lifecycle-persist-ref/execution.md
  - docs/features/tracker-lifecycle-persist-ref/validacion.md
  - docs/todos/done/[ARQUITECTURA] HU-003 PBI-04 — tracker lifecycle.md
  - docs/features/tracker-lifecycle-stamp/validacion.md
  - SddIA/evolution/a1c3e8d2-6b47-4f19-9c05-7e2d4b8a91f6.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — tracker-lifecycle-persist-ref

AC-1: `forge_processes_keep_tracker_context_out` pasó. `feature.md`, `bug-fix.md` y `refactorization.md` no contienen `tracker-operations` ni `tool:`.

AC-2: `work_initiated_records_persist_ref_without_done` pasó. El tramo nuevo del jsonl tiene `update_issue_state` y `create_comment`, y no contiene `done`.

AC-3: el diff de genoma es cero. El spec nombra `workspace_init.rs`, `tracker_stamp.rs` y `tracker-stamp.md`. Solo el handler gana tests.

`file_provider_work_initiated_skips_comment` pasó: con `provider: file` no se crea el jsonl.
