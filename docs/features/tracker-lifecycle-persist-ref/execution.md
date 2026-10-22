---
feature_name: tracker-lifecycle-persist-ref
created: "2026-10-22"
process: feature
items_applied:
  - proof-work-initiated
  - proof-file
  - proof-forge-context
branch_name: feat/tracker-lifecycle-persist-ref
persist_ref: docs/features/tracker-lifecycle-persist-ref
execution_id: "76a499d1-59eb-4b02-8e18-5586b41190f0"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
---

# Ejecución — tracker-lifecycle-persist-ref

`CARGO_TARGET_DIR=target cargo test -p execute-process --lib handlers::tracker_stamp::tests::work_initiated_records_persist_ref_without_done`

Junto con `file_provider_work_initiated_skips_comment` y `forge_processes_keep_tracker_context_out`. Tres tests, todos `ok`.
