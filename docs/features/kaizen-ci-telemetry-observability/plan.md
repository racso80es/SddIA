---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
phase: planning
agents: dedalo
phases:
  - T1-entity-class
  - T-CONTRACT
  - T2-bridge-sensor
  - T3-accumulator
  - T4-tests
  - T5-evolution
  - T6-tekton-docs
  - T7-argos-archive
  - T8-delivery-close
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
pbi_ref: docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
runtime_execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
---

# Plan — kaizen-ci-telemetry-observability

Blueprint Tekton. Contratos: `spec.md`. Este commit sella Diseño. Ejecución T1–T8 en la misma sesión tras el commit.

Init lab: `execution_id` `88cff2d5-39c5-41e6-8ca8-2a68049c4344` · relevo IDE.

## T1 — Clase ECST (CA5 / DA-2)

`./sddia-run.sh --process entity-manager` con `entity_class: event`, `lifecycle_operation: create`, `entity_name: ci-job-failed`. Semilla: `event_family: telemetry`, `event_type: CI_Job_Failed`, `event_context: system-operations`, payloads de spec §2, `emitter_agents: ["github-bridge-watcher"]`.

Prohibido escribir `SddIA/events/telemetry/ci-job-failed.md` a mano.

## T-CONTRACT — Trinidad

Tras sello `Domain_Entity_Created` de T1: parche `events-contract.md` §6 y prosa emisores de `telemetry/index.md` (spec §5). Sin tocar `radamanto.thresholds.json`.

## T2 — Sensor puente (CA1, CA2, CA4, CA7)

`BridgeState.processed_check_run_ids`. `pr_record_from_github` incluye `head_sha`. `assimilate_failed_check_runs` en `github_bridge.rs`. `main.rs`: fetch Check Runs o fixture lab. No comentarios. No mezclar con `process_pr`.

Fixture versionada: `docs/features/kaizen-ci-telemetry-observability/_smoke-remote-ci-failure.json` (plantilla). Copia operativa lab: `.SddIA/.dev/remote_ci_failure_simulation.json`.

## T3 — Acumulador (CA3, CA6)

`event-telemetry-subscriptions.json` clave `CI_Job_Failed` → `radamanto-batch`. Rama en `radamanto_batch_core.rs`. `cumulo.paths.json` `radamanto.ci_failures`. Default en `load_radamanto_config`.

## T4 — Tests

`cargo test -p sddia-daemon-runtime` (parse/idempotencia/cancelled). Test `radamanto_batch_core` CI_Job_Failed. Verde local = gate de este plan (no `gh pr checks`).

## T5 — Evolution

`sddia-qa evolution-register --json` id PBI. Si toca evolution: `sddia-qa gate-evolution --json --range` exit 0 **antes** de push/DCC.

## T6 — Documental Tekton

`implementation.md` + `execution.md`.

## T7 — Argos + archive

`validacion.md` CA1–CA7. CA de CI GitHub → `PENDIENTE-CI` hasta `run_id` (norma v1.2.1). PBI → `docs/todos/done/` mismo `document_id`. `pbi_archived: true` solo tras el move. `global: APTO` permitido si ningún CA de Actions es gate (verificación = tests locales T4).

## T8 — DCC

`./sddia-run.sh --process delivery-close-cycle` con `source_process: feature`, `persist_ref`, `branch_name`. Prohibido `gh pr create` raw. DA-6: un finding CI → un parche → un push. Prohibido polling.
