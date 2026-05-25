---
feature_name: eda-coverage-ssot-bus-isolation
created: "2026-05-25"
process: feature
branch: feat/eda-coverage-ssot-bus-isolation
global: APTO
pbi_archived: true
checks:
  UNI-CA1: pass
  UNI-CA2: pass
  UNI-CA4: pass
  UNI-CA6: pass
  UNI-CA7: pass
  V1-scan: pass
  V2-watcher-scan: pass
  V4-e2e-lab: pass
  V6-integrity: pass
  unit-tests: pass
  UNI-CA3: pass
  UNI-CA5: pass
  V3-pre-commit: pass
git_changes:
  - SddIA/core/eda-coverage.json
  - SddIA/core/cumulo.paths.json
  - SddIA/scripts/qa/eda_coverage_utils.py
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/audit-entity-eda-coverage.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/scripts/qa/env_loader.py
  - SddIA/scripts/qa/test_eda_bus_v3plus.py
  - SddIA/library/norms/features-documentation-pattern.md
  - .dev/.env.example
  - .dev/.env.test.example
  - docs/features/eda-coverage-ssot-bus-isolation/
  - docs/todos/done/[Kaizen] EDA cobertura durable, aislamiento bus y smoke e2e — SSOT eda-coverage.md
  - docs/todos/done/[Kaizen] validación genómica EDA sin dependencia del bus — correlación durable.md
  - docs/todos/done/[Kaizen] eda-bus-e2e-smoke — topología local vs suscriptores core y sweep vacío.md
---

# Validación — SSOT eda-coverage y desacople bus EDA

**Veredicto global: APTO**

## Criterios UNI-CA

| ID | Criterio | Resultado |
|----|----------|-----------|
| UNI-CA1 | `--scan` vía `eda-coverage.json` | ✅ `scan_source: eda-coverage.json` |
| UNI-CA2 | Scan post-watcher sin retención cabeceras | ✅ V2 |
| UNI-CA3 | delivery-close sin manifiesto frágil | ✅ scan SSOT; excepción manifiesto no requerida |
| UNI-CA4 | E2E lab exit 0 | ✅ `success: true` |
| UNI-CA5 | CI `eda-bus-e2e-smoke` | ✅ local + CI en PR |
| UNI-CA6 | Lab `scope: local` + bus `.tmp/events_test` | ✅ |
| UNI-CA7 | Workaround `retain_processed` eliminado | ✅ tests sweep vacío |
| V3 | Pre-commit commit prueba `SddIA/` | ✅ sin BLOCKED |

## Comandos reproducibles

```powershell
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/daemons/event-watcher.py --once
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
cd SddIA/scripts/qa; python -m unittest test_eda_bus_v3plus -v
python SddIA/scripts/qa/verify-process-integrity.py
```

## Cierre documental

PBI `PBI-KAIZEN-EDA-COVERAGE-SSOT-BUS-ISOLATION` archivado en `docs/todos/done/` en la rama del PR.
