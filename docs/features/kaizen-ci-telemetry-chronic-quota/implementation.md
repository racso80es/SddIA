---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
items:
  - T1-threshold
  - T2-quota-engine
  - T3-forge-event-action
  - T4-handler-subs
  - T5-tests
  - T6-evolution
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
runtime_execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
---

# Implementation — kaizen-ci-telemetry-chronic-quota

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/agents/radamanto.thresholds.json` | v1.3.0 bloque `ci_failures` `{ per_job_limit: 3, job_entity_map: {} }` (DA-4, no EM) |
| `SddIA/engine/execute-process/src/engine/fractal_bus.rs` | Default in-code del objeto umbrales: mismo bloque |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | Cuota por `job_name`; sello `alerts` post-`write_fractal_event` OK; retry duplicado sin sello; `status` aditivo |
| `SddIA/events/domain/ci-chronic-failure-detected.md` | Forja EM `event-creator`. UUID `c55ef8cc-41b8-42af-a524-c58b847039a8`. Sello `Domain_Entity_Created` `0af54af4-b1db-4c4f-9941-43687d829f9a` |
| `SddIA/actions/materialize-ci-chronic-failure-pbi.md` | Forja EM `action-creator`. UUID `a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92`. Sello `Domain_Entity_Created` `9d336aaa-d5f6-45a0-8b99-c37a85982b8e` |
| `SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs` | Handler nativo; idempotencia `pending/` + `done/` |
| `SddIA/engine/execute-process/src/engine/actions.rs` | `try_run_native` |
| `SddIA/engine/execute-process/src/engine/mod.rs` | `pub mod materialize_ci_chronic_failure_pbi` |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `CONSUMER_SKIP_FORGE_ACTIONS` |
| `SddIA/core/event-domain-subscriptions.json` | `CI_Chronic_Failure_Detected` → Cúmulo / `materialize-ci-chronic-failure-pbi` |
| `SddIA/core/eda-coverage.json` | Upsert EM create (evento + acción) |

## Fuera de este PR

Pares `job_entity_map` laudoados (CA9 positivo). `radamanto.md` (agent-creator update regenera UUID). Compactación ledger / L-RESET. Cierre documental / DCC.
