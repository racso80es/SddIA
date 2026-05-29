---
feature_name: inmunidad-caos-fase3
created: "2026-05-29"
process: feature
items_applied:
  - "3.A — directories.suites, suite-creator, entity-manager enum suite, sync-entity-index"
  - "3.B — suites-contract v1.0.0, suites/index.md"
  - "3.C — execute-suite.md, run_execute_suite, invoke_subprocess_process_full, bootstrap workspace_path inyectado"
  - "3.D — compile_survival_manifest en workspace orquestador"
  - "3.E — core-full-stress.md (3 nodos audit Fase 2)"
  - "3.F — test_execute_suite.py (5 tests), fail-fast-lab suite, eda-coverage upsert, smoke fixture"
---

# Ejecución — Fase 3

## Registro Tekton

| Paso | Evidencia |
|------|-----------|
| Genoma | `SddIA/suites/`, `process/suite-creator.md`, `entity-manager` +9ª clase |
| Orquestador | `run_execute_suite` — estrategias `fail_fast` / `run_all` (lab secuencial) |
| Aislamiento | `nodes/{idx}-{process}/{execution_id}/` bajo workspace orquestador |
| Manifiesto | `{workspace_path}/survival-manifest.md` tras nodos |
| Regresión | `test_execute_suite` 4/4 + `test_chaos_audit_processes` 5/5 |
| EDA | `orphan_count: 0` (`audit-entity-eda-coverage --scan`) |

## Rama

`feat/inmunidad-caos-fase3`
