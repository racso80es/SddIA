---
feature_name: refactor-topologia-eventos-ola-c-v3
created: "2026-05-22"
process: refactorization
branch: feat/refactor-topologia-eventos-ola-c-v3
global: APTO
checks:
  CA1-bootstrap: pass
  CA2-route-topology: pass
  CA3-processed-witness: pass
  CA4-dead-letter-sweeper: pass
  CA5-purge-processing: pass
  CA6-sweeper-consensus: pass
  CA7-watcher-process: pass
  CA8-process-artifact: pass
  CA9-e2e-lab: pass
  CA10-idempotency: pass
  CA11-async-fanout: pass
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/process/route-domain-event.md
  - SddIA/scripts/daemons/event-watcher.py
  - SddIA/scripts/daemons/event-sweeper.py
---

# Validación — Topología simétrica bus EDA V3+

## Matriz CA

| ID | Criterio | Resultado | Evidencia |
|----|----------|-----------|-----------|
| CA1 | Bootstrap 7 rutas | ✅ | `test_eda_bus_v3plus.test_bootstrap_creates_symmetric_tree` |
| CA2 | Route → processing header + testigos; pending intacto | ✅ | E2E `run-eda-e2e-lab.py` |
| CA3 | Testigos en `processed/subscribers/` | ✅ | E2E `witnesses_processed` |
| CA4 | Dead-letter → alerta sweeper | ✅ | Diseño `event-sweeper.py` + testigo ECST histórico |
| CA5 | Purge `processing/[UUID].json` al cerrar suscriptores | ✅ | `test_purge_processing_when_all_terminal` |
| CA6 | Sweeper purga pending con consenso | ✅ | Sweeper `--once` purgó evento E2E |
| CA7 | Watcher invoca proceso | ✅ | `event-watcher.py` → `execute-process` |
| CA8 | Proceso en genoma | ✅ | `SddIA/process/route-domain-event.md` + índice |
| CA9 | E2E lab verde | ✅ | `run-eda-e2e-lab.py --entity-class tool` |
| CA10 | Idempotencia testigos terminales | ✅ | `test_terminal_witness_idempotent` |
| CA11 | Fan-out async (`dispatch_mode: async`) | ✅ | E2E `dispatch_mode: async` |

## Comandos ejecutados

```bash
python -m unittest test_eda_bus_v3plus -v
SDDIA_LAB_SIMULATE_IOTA=1 SDDIA_LAB_SIMULATE_SYNC_INDEX=1 python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
python SddIA/scripts/daemons/event-sweeper.py --once --json
```

## Veredicto

**APTO** — Hitos K1–K5 implementados; documentación K6 cerrada. Pendiente: forja formal `entity-manager` del proceso (hash `pending-forge`) y cierre PR vía `delivery-close-cycle`.
