---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
items_applied:
  - T1-entity-class
  - T-CONTRACT
  - T2-bridge-sensor
  - T3-accumulator
  - T4-tests
---

# Ejecución — kaizen-ci-telemetry-observability

## Init

`execution_id`: `88cff2d5-39c5-41e6-8ca8-2a68049c4344`. Relé IDE. Commit Diseño: `8ea44a2`.

## T1

`entity-manager` create `ci-job-failed`. `handoff_entity_uuid` `1c026b2b-5ee1-40ff-940d-e214ba98c494`. `event_id` `f09faca4-4d96-4cdf-ad60-9cb665e3b3f9`.

## T-CONTRACT / T2 / T3

Sensor Check Runs + fixture lab. Ledger `.SddIA/radamanto/ci_failures.json`. Suscripción `CI_Job_Failed` → `radamanto-batch`.

## T4 tests locales

```text
cargo test -p sddia-daemon-runtime --lib github_bridge
# 2 passed (failure_emits_once_cancelled_skipped, unknown_job_maps_workflow_github_actions)
cargo test -p execute-process --lib ci_job_failed_writes_ledger_not_stats
# 1 passed
cargo build -p github-bridge-watcher
# ok
```
