---
feature_name: inmunidad-caos-fase4
created: "2026-05-29"
process: feature
items:
  - id: "4.A"
    touchpoint: "events/domain, events/domain/index.md"
    proposal: "Clases ECST Suite_Execution_Requested y System_Immunity_Certified"
  - id: "4.B"
    touchpoint: "emit-suite-execution-requested, event-domain-subscriptions.json"
    proposal: "Estímulo indexado + suscripciones domain"
  - id: "4.C"
    touchpoint: "execute-suite, radamanto, execute_process_capsules"
    proposal: "Certificación post-manifiesto + DLT Radamanto"
  - id: "4.D"
    touchpoint: "test_chaos_immunity_eda.py, eda-coverage.json"
    proposal: "Regresión AC4.x"
  - id: "4.E"
    touchpoint: "dlt-immunity-acta.md"
    proposal: "Acta jurisdicción DLT"
---

# Implementación — Fase 4

| ID | Artefacto | Estado |
|----|-----------|--------|
| 4.A | Clases ECST domain | ✅ |
| 4.B | Acción emisora + suscripciones | ✅ |
| 4.C | Handler certificación + Radamanto §3 | ✅ |
| 4.D | Tests + EDA coverage + smoke fixture | ✅ |
| 4.E | Acta DLT | ✅ |

## Cambios runtime

- `emit_system_immunity_certified` + hook en `run_execute_suite` (solo si `all_pass`).
- `_run_emit_suite_execution_requested` en `execute-action.py` → `write_fractal_event` domain.
- `route_fractal_event_core`: fan-out `execute-suite` desde payload `suite_id`.
