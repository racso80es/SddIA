---
feature_name: kaizen-ci-telemetry-observability
created: "2026-09-01"
process: feature
base: main
scope: mvp-a1-a2-a31
branch_name: feat/kaizen-ci-telemetry-observability
persist_ref: docs/features/kaizen-ci-telemetry-observability
pbi_ref: docs/todos/pending/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
execution_id: "88cff2d5-39c5-41e6-8ca8-2a68049c4344"
---

# Especificación — kaizen-ci-telemetry-observability

## 1. Contratos

| Pieza | Contrato |
|-------|----------|
| Clase | `ci-job-failed` / `CI_Job_Failed` / `event_family: telemetry` / `context: system-operations` |
| Emisor | `github-bridge-watcher` (paridad `Daemon_Heartbeat`, no CLI Peaje) |
| Destino | `eda_fractal.telemetry` → `./.events/telemetry/{event_id}.json` vía `write_fractal_event` |
| Fan-out | `event-telemetry-subscriptions.json` → `CI_Job_Failed` → `radamanto-batch` |
| Ledger | `cumulo.paths.json` `radamanto.ci_failures` = `.SddIA/radamanto/ci_failures.json` |
| Estado puente | `.SddIA/.dev/github_bridge_state.json` |

## 2. Payload Clase (REQUIRED)

`repository`, `head_sha`, `workflow_name`, `job_name`, `conclusion` (`failure`), `html_url`, `check_run_id`.

OPTIONAL: `pr_url`, `step_name` (solo si la API lo trae), `run_id`.

FORBIDDEN: `entity_id`, `asset_id`, `exit_code`, `process_name` (anti-contaminación Peaje).

## 3. Sensor

En `run_cycle` de `github-bridge-watcher`, **después** de listar PRs y **sin** mezclar envelope con `process_pr`:

1. Extraer `head_sha` del PR (`head.sha`). Añadir el campo al record local.
2. Lab (`SDDIA_LAB_SIMULATE_REMOTE_PR`): leer `.SddIA/.dev/remote_ci_failure_simulation.json` (array o `{ "check_runs": [...] }`). Sin HTTP. Sin `GITHUB_TOKEN`.
3. Remoto: `GET /repos/{owner}/{repo}/commits/{sha}/check-runs` (`Accept: application/vnd.github+json`).
4. Filtrar `conclusion == "failure"`. Descartar `cancelled`, `skipped`, `timed_out`, `null`.
5. Jobs conocidos de `sddia-index-qa`: `sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical` → `workflow_name=sddia-index-qa`. Resto → `workflow_name=github-actions`.
6. Si `check_run_id` ∈ `BridgeState.processed_check_run_ids` → skip.
7. Emitir ECST. Append id. `save_bridge_state`. `note_stimulus` si hubo emisión.

Lógica parse/emit/idempotencia en `sddia-daemon-runtime::github_bridge` (testeable). HTTP sigue en `main.rs`.

## 4. Radamanto A3.1

`process_telemetry_file_inner`: si `event_type == CI_Job_Failed`, rama propia **antes** de exigir `asset_id`:

- Dedup por `check_run_id` en el ledger.
- Append `{ check_run_id, job_name, workflow_name, head_sha, html_url, timestamp, event_id }`.
- Stamp `radamanto.radamanto-batch` = success.
- Return `{ ok: true, kind: "ci_job_failed" }`.
- Prohibido `entity_bucket`, `stats.json`, `Domain_Entity_Degraded`.

`load_radamanto_config` default `ci_failures`.

## 5. Trinidad (T-CONTRACT)

`events-contract.md` §6 fila telemetry: emisores = CLI (Peaje) **y** centinelas catalogados en la Clase (`Daemon_Heartbeat`, `CI_Job_Failed`). Destino inalterado. Regla de oro inalterada.

`telemetry/index.md` tabla emisores: mismo texto. Catálogo: fila que inserta `event-creator`.

## 6. Tests

| ID | Dónde | Qué |
|----|-------|-----|
| T-PARSE | `sddia-daemon-runtime` | failure emite; cancelled no; id duplicado no reemite |
| T-LAB | binario `--once` o unidad con fixture | CA7 sin token |
| T-BATCH | `radamanto_batch_core` | ledger sí; stats.json intacto |

## 7. Evolution

`sddia-qa evolution-register` id `f8661783-55b7-4419-b659-e96369c02410`. `gate-evolution --json --range` antes de DCC si el diff toca `directories.evolution`.
