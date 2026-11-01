---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
process: refactorization
branch_name: Fix_Events_Multiple
tracker_ref: AUT-21
---

# Implementación — AUT-21

## Touchpoints

| Tarea | Archivo |
|-------|---------|
| T1–T3 | `SddIA/engine/execute-process/src/engine/ecst_input_loader.rs` |
| T2 | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` |
| T4 | `tracker_stamp.rs`, `tracker_sync_replay.rs`, `email_digest_preference_reply.rs` |
| T5–T6 | `tests/eda_subscriber_event_push.rs` |
| T6 doc | `SddIA/events/events-contract.md` + evolution `1d37df2e-c11b-4ec1-8816-5b86dc19d5c1` |
| T7 | `t7-reanchor-runbook.md`, `_lab/replay-false-success-stamps.py`, `--reset-subscriber-delivery` en `event_store_query.rs` |
