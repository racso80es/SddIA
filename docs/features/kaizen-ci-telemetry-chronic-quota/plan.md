---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
phase: planning
agents: dedalo
phases:
  - T0-docs
  - T1-threshold
  - T2-quota-engine
  - T3-forge-event-action
  - T4-handler-subs
  - T5-tests
  - T6-evolution
  - T7-tekton-docs
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
pbi_ref: docs/todos/done/[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8-CA9).md
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
runtime_execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
---

# Plan — kaizen-ci-telemetry-chronic-quota

Blueprint Tekton. Contratos: `spec.md`. Init `18aec32c-f457-4330-819c-2366b959cf57`.

## T0 — Documental de ciclo

`objectives.md` / `clarify.md` / `spec.md` / `plan.md`.

## T1 — Umbral (DA-4 JSON)

`radamanto.thresholds.json` v1.3.0 bloque `ci_failures`. Default in-code en `load_radamanto_config`. Test versión `1.3.0`.

## T2 — Motor cuota

`process_ci_job_failed`: conteo, lookup, emisión, sello post-OK, retry en duplicado sin sello. Conservar contrato `ok`/`kind`/`check_run_id`.

## T3 — Forja EM

`entity-manager` create evento + acción. Prohibido Write IDE en `SddIA/events/` y `SddIA/actions/`. No tocar `radamanto.md`.

## T4 — Handler + suscripción

Módulo `materialize_ci_chronic_failure_pbi`. Registro `try_run_native`. `CONSUMER_SKIP_FORGE_ACTIONS`. Clave en `event-domain-subscriptions.json`.

## T5 — Tests

`cargo test -p execute-process` filtros `ci_job_failed` / `ci_chronic` / `thresholds_110`. Verde local = gate. No `gh pr checks`.

## T6 — Evolution

`sddia-qa evolution-register --json` id PBI.

## T7 — `implementation.md` + `execution.md`
