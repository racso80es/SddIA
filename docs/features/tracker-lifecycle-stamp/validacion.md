---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
updated: "2026-10-22"
process: feature
branch: feat/tracker-lifecycle-stamp
branch_name: feat/tracker-lifecycle-stamp
persist_ref: docs/features/tracker-lifecycle-stamp
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
uuid: "85caf44d-e5d0-4c30-8303-bcb825610f92"
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
global: APTO
pbi_archived: true
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/process/tracker-stamp.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/f74e7f14-5c3e-4fd7-a524-1fb9c84b4921.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/tracker-lifecycle-stamp/objectives.md
  - docs/features/tracker-lifecycle-stamp/clarify.md
  - docs/features/tracker-lifecycle-stamp/spec.md
  - docs/features/tracker-lifecycle-stamp/plan.md
  - docs/features/tracker-lifecycle-stamp/implementation.md
  - docs/features/tracker-lifecycle-stamp/execution.md
  - docs/features/tracker-lifecycle-stamp/validacion.md
  - docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
---

# Validación — tracker-lifecycle-stamp

AC-1: `linear-tracker-adapter` no aparece en `SddIA/process/tracker-stamp.md` ni en `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs`.

AC-2: `ac2_state_stamp_uses_live_op_and_schema` pasó en `cargo test -p execute-process --lib handlers::tracker_stamp` (11 ok, 2026-10-22). El stdout de `update_issue_state` cumple `tracker.lifecycle`.

AC-3: el frontmatter conserva `context: tracker-operations`.

El `NO_APTO` que llegó a `main` con el PR 365 es el veredicto de una sesión de Argos sin cápsulas. No sustituye esta ejecución. El PBI está en `docs/todos/done/`.
