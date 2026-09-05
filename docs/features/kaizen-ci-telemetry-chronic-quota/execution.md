---
feature_name: kaizen-ci-telemetry-chronic-quota
created: "2026-09-05"
process: feature
branch_name: feat/kaizen-ci-telemetry-chronic-quota
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
execution_id: "18aec32c-f457-4330-819c-2366b959cf57"
items_applied:
  - T1-threshold
  - T2-quota-engine
  - T3-forge-event-action
  - T4-handler-subs
  - T5-tests
  - T6-evolution
  - T7-tekton-docs
  - T8-cierre-documental
---

# Ejecución — kaizen-ci-telemetry-chronic-quota

## Init

`execution_id` `18aec32c-f457-4330-819c-2366b959cf57`. Rama `feat/kaizen-ci-telemetry-chronic-quota`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`, skip archive/DCC). Incorpora los 2 commits locales de `main` no empujados (`a7b3d0e`, `e158724`).

## T1–T2 motor

`radamanto.thresholds.json` v1.3.0. Default in-code en `load_radamanto_config`. `process_ci_job_failed`: conteo por `job_name`, lookup mapa, emisión, sello `alerts` solo post-OK, retry si duplicado `check_run_id` sin sello. Contrato `ok`/`kind`/`check_run_id` + `status`.

## T3 forja EM

`SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process entity-manager`.

| Entidad | UUID | Sello Domain_Entity_Created | execution_id EM |
|---------|------|-----------------------------|-----------------|
| `ci-chronic-failure-detected` | `c55ef8cc-41b8-42af-a524-c58b847039a8` | `0af54af4-b1db-4c4f-9941-43687d829f9a` | `070bb2d0-71cf-436d-8f6c-39ae27aed6d9` |
| `materialize-ci-chronic-failure-pbi` | `a6eb7f0c-8b2f-4c7d-ae5e-6c1b589f3c92` | `9d336aaa-d5f6-45a0-8b99-c37a85982b8e` | `5fb899c9-7c05-4a26-aaa7-1f8b4277c49c` |

Primer intento de forja falló: cápsula `cryptography-manager` ausente en `SddIA/target`. Compilada nativa debug; reintento OK. `radamanto.md` no mutado.

## T4–T5

Suscripción dominio → Cúmulo. Handler nativo + `CONSUMER_SKIP_FORGE_ACTIONS`.

```text
cd SddIA && cargo test -p execute-process --lib -- ci_job_failed ci_chronic thresholds_110
# 8 passed
```

## T6 evolution / cobertura

`sddia-qa evolution-register` → `166c91f9-7378-4766-b6fe-ff5e7eee382f` (`EVOL_OK`, `alta`).

`sddia-qa audit-eda-coverage --scan --json` → `orphan_count: 0`.

## T8 cierre documental

PBI → `docs/todos/done/` (`status: done`). `validacion.md` `global: APTO`, `pbi_archived: true`. DCC vía `./sddia-run.sh --process delivery-close-cycle`.

## Fuera de alcance del PR

CA9 positivo (mapa laudoado). `radamanto.md`. Compactación ledger / L-RESET.
