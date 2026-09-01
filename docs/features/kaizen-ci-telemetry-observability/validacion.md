---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
phase: validate
agents: argos
branch: feat/kaizen-ci-telemetry-observability
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
pbi_ref: docs/todos/done/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/249
ci_run_id: "33516407073"
ci_run_event: pull_request
checks:
  CA1: APTO
  CA2: APTO
  CA3: APTO
  CA4: APTO
  CA5: APTO
  CA6: APTO
  CA7: APTO
git_changes:
  - README.md
  - SddIA/core/cumulo.paths.json
  - SddIA/core/eda-coverage.json
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/daemons/github-bridge-watcher/src/main.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/events/events-contract.md
  - SddIA/events/index.md
  - SddIA/events/telemetry/ci-job-failed.md
  - SddIA/events/telemetry/index.md
  - SddIA/evolution/Evolution_log.md
  - SddIA/evolution/f8661783-55b7-4419-b659-e96369c02410.md
  - SddIA/sddia-daemon-runtime/src/github_bridge.rs
  - docs/features/kaizen-ci-telemetry-observability/clarify.md
  - docs/features/kaizen-ci-telemetry-observability/objectives.md
  - docs/features/kaizen-ci-telemetry-observability/spec.md
  - docs/features/kaizen-ci-telemetry-observability/plan.md
  - docs/features/kaizen-ci-telemetry-observability/implementation.md
  - docs/features/kaizen-ci-telemetry-observability/execution.md
  - docs/features/kaizen-ci-telemetry-observability/validacion.md
  - docs/features/kaizen-ci-telemetry-observability/finalize-process.md
  - docs/features/kaizen-ci-telemetry-observability/_smoke-remote-ci-failure.json
  - docs/todos/done/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
---

# Validacion — kaizen-ci-telemetry-observability

`global: APTO`. Tests locales T4 + CI `pull_request` run [33516407073](https://github.com/racso80es/SddIA/actions/runs/33516407073) (cinco jobs SUCCESS). PBI archivado. PR https://github.com/racso80es/SddIA/pull/249.

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| CA1 | APTO | `failure_emits_once_cancelled_skipped`: un `CI_Job_Failed` en `./.events/telemetry/` |
| CA2 | APTO | segundo `assimilate_failed_check_runs` → 0 emisiones |
| CA3 | APTO | Clase `ci-job-failed.md` + catálogo; `emitter_agent=github-bridge-watcher`; familia telemetry |
| CA4 | APTO | `cancelled`/`skipped` no emiten |
| CA5 | APTO | `entity-manager` `handoff_entity_uuid` `1c026b2b-5ee1-40ff-940d-e214ba98c494`; `radamanto.thresholds.json` intacto |
| CA6 | APTO | `ci_job_failed_writes_ledger_not_stats`: ledger sí, `stats.json` no |
| CA7 | APTO | Fixture `_smoke-remote-ci-failure.json`; lab sin token; `SDDIA_LAB_SIMULATE_REMOTE_PR` |

## Tests

```text
cargo test -p sddia-daemon-runtime --lib github_bridge   # 2 passed
cargo test -p execute-process --lib ci_job_failed_writes_ledger_not_stats  # 1 passed
```
