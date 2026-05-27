---
feature_name: telemetria-reactiva-eda-fase3
created: "2026-05-27"
process: feature
items_applied:
  - "3.B eda_fractal SSOT"
  - "3.D' Process_Execution_Completed"
  - "3.A Peaje Termodinámico + D3.13 fail-soft E/S"
  - "3.C split suscripciones + enrutadores"
  - "3.C.1 watcher multi-ruta"
  - "3.E stub + test_eda_fractal_bus"
  - "3.F touchpoints + plantilla instancia"
---

# Ejecución — Fase 3

## Smoke manual

```text
python SddIA/scripts/qa/execute-process.py --process workspace-smoke --inputs {}
```

Resultado: `thermodynamic_toll.telemetry.target_path` → `.events/telemetry/{uuid}.json`; `orchestration` en `.events/orchestration/`; `workspace_path` inyectado.

## Tests QA

| Suite | Resultado |
|-------|-----------|
| `test_eda_fractal_bus.py` | 6/6 OK (incl. fail-soft E/S D3.13) |
| `test_eda_bus_v3plus.py` | 14/14 OK (sin regresión V3+) |

## Evidencia AC3.x

| AC | Evidencia |
|----|-----------|
| AC3.1 | Peaje emite telemetría en toda ejecución `workspace-smoke` |
| D3.13 | `test_thermodynamic_toll_io_fail_soft_preserves_business_success` — negocio OK con E/S rota |
| AC3.2 | 3 suscripciones + 4 procesos (`route-*` + stub) en `process/index.md` |
| AC3.3 | Test `test_no_telemetry_in_orchestration_path` |
| AC3.4 | Suscripción `radamanto` → `telemetry-batch-stub`; test purge |

## Pendiente cierre

- Argos → `validacion.md` APTO
- `delivery-close-cycle` → PR
