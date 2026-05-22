---
feature_name: refactor-topologia-eventos-ola-c-v3
created: "2026-05-22"
process: refactorization
---

# Implementación — Topología simétrica bus EDA V3+

## K1 — SSOT y utilidades

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/core/cumulo.paths.json` | `eda_bus` simétrico: `processing`, `processed`, `dead_letter` (sin `subscribers.*` plano) |
| `SddIA/scripts/qa/eda_bus_utils.py` | Topología V3+; cabeceras; testigos anidados; alias legacy; `archive_event_after_sweep` |

## K2 — Proceso orquestador

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/scripts/qa/route_domain_event_core.py` | Núcleo ECST gate, fan-out, promoción, idempotencia |
| `SddIA/process/route-domain-event.md` | Contrato proceso v1.0.0 |
| `SddIA/process/index.md` | Fila `route-domain-event` |
| `SddIA/scripts/qa/execute_process_capsules.py` | Handler `run_process` → `route-domain-event` |

## K3 — Deprecación acción

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/actions/route-domain-event.md` | `status: deprecated`; puntero a proceso |
| `SddIA/scripts/qa/execute-action.py` | Shim `_run_route_domain_event_shim` |

## K4 — Watcher y sweeper

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/scripts/daemons/event-watcher.py` | Delgado; delega `execute-process --process route-domain-event` |
| `SddIA/scripts/daemons/event-sweeper.py` | Rutas `*/subscribers/`; in-flight guard; purga cabeceras |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | Asserts V3+ |

## K5 — Fan-out async

| Touchpoint | Cambio |
|------------|--------|
| `route_domain_event_core.py` | `ThreadPoolExecutor`; `dispatch_mode: async` (default) |
| Env | `SDDIA_LAB_ROUTE_SYNC=1` → modo secuencial |
| Decoración testigo | `result_status`, `delegation`, timestamps §5 spec |
| `SddIA/scripts/qa/test_eda_bus_v3plus.py` | Tests bootstrap, cabecera, purge, idempotencia |

## K6 — Verificación

| Touchpoint | Cambio |
|------------|--------|
| `validacion.md` | Matriz CA1–CA11 |
| `execution.md` | Comandos lab actualizados |
