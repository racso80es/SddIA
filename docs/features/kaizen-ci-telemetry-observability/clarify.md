---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY v1.1.0
version_clarify: "1.0.0"
execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
pbi_ref: docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
---

# Clarificación — kaizen-ci-telemetry-observability

Transcript Mayeuta. Semilla: PBI v1.1.0 (Filtro A ya aplicado). Init lab `execution_id` `88cff2d5-39c5-41e6-8ca8-2a68049c4344`. Relé IDE.

## D0 — Apertura

| Pregunta | Decisión |
|----------|----------|
| Vehículo | `--process feature`. Relé `SDDIA_AGENT_RELAY_IDE=1`. Init con skip archive/DCC; DCC al cierre. |
| Rama | `feat/kaizen-ci-telemetry-observability` |
| `persist_ref` | `docs/features/kaizen-ci-telemetry-observability` |
| MVP | A1 + A2 + A3.1. CA1–CA7. CA8/CA9 fuera. |
| Stop planning | Commit de Diseño (clarify/objectives/spec/plan). Ejecución en la misma sesión tras ese commit. |

## D1 — Sensor (L-API)

Check Runs `GET /repos/{owner}/{repo}/commits/{sha}/check-runs`. `conclusion == failure` únicamente. `cancelled` / `skipped` / `timed_out` no emiten. `job_name` = `check_run.name`. `workflow_name` = `sddia-index-qa` si el nombre es uno de los cinco jobs del workflow; si no, `github-actions`. Prohibido inferir `entity_id`.

## D2 — Forja (DA-2 / DA-4)

Clase `ci-job-failed` vía `entity-manager` (`entity_class: event`, `lifecycle_operation: create`). Emisor: `github-bridge-watcher`. Contexto: `system-operations` (paridad `Daemon_Heartbeat`).

Prosa Trinidad (`events-contract.md` §6 y bloque emisores de `telemetry/index.md`): no hay creator de contrato. T-CONTRACT = parche en ejecución **después** de topología feature (DA-4) y **después** del sello `Domain_Entity_Created` de la Clase. `radamanto.thresholds.json` no se toca.

## D3 — Acumulador (A3.1)

`.SddIA/radamanto/ci_failures.json` (clave nueva `radamanto.ci_failures` en `cumulo.paths.json`). Consumidor: `radamanto-batch` rama distinta a `Raw_Execution_Finished`. Sella `delivery_state`. No escribe `stats.json`. No emite `Domain_Entity_Degraded`.

## D4 — Lab (CA7)

Misma bandera `SDDIA_LAB_SIMULATE_REMOTE_PR`. Fixture `.SddIA/.dev/remote_ci_failure_simulation.json`. `github-bridge-watcher --once` sin `GITHUB_TOKEN`.

## D5 — Fuera

Comentarios PR. `gh pr checks`. `max_ci_failures_per_entity`. Kintsugi. `push` a `main` sin PR (L-MAIN). Mapa job→entidad (L-MAP). Reabrir PBI CI step/workflow.
