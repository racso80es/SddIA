---
feature_name: telemetria-reactiva-eda-fase3
created: "2026-05-27"
process: feature
items:
  - id: "3.B"
    touchpoint: "SddIA/core/cumulo.paths.json, eda_bus_utils.py"
    proposal: "eda_fractal SSOT v1.2.0 + ensure_fractal_bus_topology + write_fractal_event"
  - id: "3.D'"
    touchpoint: "SddIA/events/orchestration/process-execution-completed.md"
    proposal: "Clase ECST Process_Execution_Completed"
  - id: "3.A"
    touchpoint: "execute_process_capsules.py"
    proposal: "Peaje Termodinámico fail-soft D3.13: log emergencia + veredicto negocio inmutable ante E/S"
  - id: "3.C"
    touchpoint: "event-*-subscriptions.json, route-*.md, route_fractal_event_core.py"
    proposal: "Split suscripciones + enrutadores fractales"
  - id: "3.C.1"
    touchpoint: "event-watcher.py"
    proposal: "Watcher multi-ruta con SDDIA_LAB_WATCH_FRACTAL"
  - id: "3.E"
    touchpoint: "telemetry-batch-stub.md, test_eda_fractal_bus.py"
    proposal: "Stub Radamanto + smoke AC3.1–AC3.4"
  - id: "3.F"
    touchpoint: "touchpoints-ia.md, eda-instance-events/README.md"
    proposal: "Documentación persistencia ECST + runtime fractal"
---

# Implementación — Fase 3

| Paso | Archivos | Cambio |
|------|----------|--------|
| 3.B | `cumulo.paths.json` v1.2.0, `eda_bus_utils.py` | Bloque `eda_fractal`; helpers fractales |
| 3.D′ | `orchestration/process-execution-completed.md`, `orchestration/index.md` | Clase orquestación |
| 3.A | `execute_process_capsules.py` | `run_thermodynamic_toll`, exempt enrutadores |
| 3.C | `event-domain/telemetry/orchestration-subscriptions.json`, `route-*.md`, `route_fractal_event_core.py` | Split + enrutadores |
| 3.C.1 | `event-watcher.py` | Poll `pending/` + rutas fractales |
| 3.E | `telemetry-batch-stub.md`, `test_eda_fractal_bus.py` | Stub + tests |
| 3.F | `touchpoints-ia.md`, plantilla instancia | ECST `workspace_path` |

Nuevo módulo: `SddIA/scripts/qa/route_fractal_event_core.py`.
