---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
items:
  - id: H1-spec
    touchpoint: docs/features/kaizen-event-creator-event-family-explicit/spec.md
    status: done
  - id: H2-event-creator
    touchpoint: SddIA/process/event-creator.md
    status: done
  - id: H3-runtime
    touchpoint: SddIA/scripts/qa/execute_process_capsules.py
    status: done
  - id: H4-seeds
    touchpoint: run-eda-e2e-lab.py, ola-c-event-entity/execution.md
    status: done
  - id: H5-fase1-docs
    touchpoint: docs/features/telemetria-reactiva-eda-fase1/
    status: done
  - id: H6-tests
    touchpoint: SddIA/scripts/qa/test_event_forge_fractal.py
    status: done
---

# Implementación — Kaizen event-creator event_family explícito

## Touchpoints

| ID | Archivo | Cambio |
|----|---------|--------|
| H2 | `event-creator.md` v1.2.0 | Input `event_family` obligatorio; fase 0 validación estricta |
| H3 | `execute_process_capsules.py` | `resolve_event_family_required`, `run_event_forge` fractal, `creator_inputs_from_entity` |
| H3 | `entity-manager.md` | Tabla seed → `event_family` obligatorio |
| H4 | `run-eda-e2e-lab.py` | `event_family: domain` en seed event |
| H4 | `ola-c-event-entity/execution.md` | Smoke JSON con `event_family` |
| H5 | `telemetria-reactiva-eda-fase1` | D1.9 cerrado; §6.1 histórico + v1.2.0 |
| H6 | `test_event_forge_fractal.py` | Regresión domain/telemetry + error sin familia |

## Pendiente cierre

- `execution.md` smokes en rama
- `validacion.md` APTO + PBI → `done/`
