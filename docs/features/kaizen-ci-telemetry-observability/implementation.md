---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
items:
  - T1-entity-class
  - T-CONTRACT
  - T2-bridge-sensor
  - T3-accumulator
  - T4-tests
  - T5-evolution
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
runtime_execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
---

# Implementation — kaizen-ci-telemetry-observability

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/events/telemetry/ci-job-failed.md` | Forja `entity-manager` → `event-creator`. UUID `1c026b2b-5ee1-40ff-940d-e214ba98c494`. Sello `Domain_Entity_Created` `f09faca4-4d96-4cdf-ad60-9cb665e3b3f9`. |
| `SddIA/events/telemetry/index.md` | Catálogo (creator) + prosa emisores (T-CONTRACT) |
| `SddIA/events/events-contract.md` | §6: CLI + centinelas de Clase |
| `SddIA/events/index.md` | Emisor familia telemetry |
| `SddIA/core/event-telemetry-subscriptions.json` | `CI_Job_Failed` → `radamanto-batch` |
| `SddIA/core/cumulo.paths.json` | `radamanto.ci_failures` |
| `SddIA/sddia-daemon-runtime/src/github_bridge.rs` | `processed_check_run_ids`, parse/emit/idempotencia |
| `SddIA/daemons/github-bridge-watcher/src/main.rs` | Check Runs / fixture lab; no mezcla con `process_pr` |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | Rama `CI_Job_Failed` → ledger, no `stats.json` |
| `SddIA/engine/execute-process/src/engine/fractal_bus.rs` | Default `ci_failures` |
| `README.md` | Trinidad emisores telemetry |

## Fuera de este PR

`radamanto.thresholds.json`. `Domain_Entity_Degraded`. Comentarios PR. CA8/CA9.
