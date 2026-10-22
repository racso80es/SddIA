---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
process: feature
items_applied:
  - requires-capability
  - resolved-provider
  - file-noop
branch_name: feat/tracker-lifecycle-stamp
persist_ref: docs/features/tracker-lifecycle-stamp
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
---

# Ejecución — tracker-lifecycle-stamp

`CARGO_TARGET_DIR=target cargo test -p execute-process --lib handlers::tracker_stamp -- --test-threads=1`

Once tests, todos `ok`. Incluyen `ac2_state_stamp_uses_live_op_and_schema` y `file_provider_skips_capsule`.

## Sello

`entity-manager` update. `handoff_hash_signature_new` `sha256:2cf44164ccf886d6c532098eb7d96a032f787a14d6bb2d8d7fb5bccdc0a1692c`. `idempotent: true` sobre `c7edcb8f-560a-42be-972a-38687f1013d0`. Cobertura `correlation_timestamp` `2026-10-22T18:15:47.965Z`.
