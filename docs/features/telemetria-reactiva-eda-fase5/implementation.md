---
feature_name: telemetria-reactiva-eda-fase5
created: "2026-05-28"
process: feature
items:
  - id: "5.A"
    touchpoint: "skills-contract.md, actions-contract.md, text-metrics.md, cumulo.paths.json v1.4.0"
    proposal: "telemetry_provided / telemetry_schema declarativos"
  - id: "5.E'"
    touchpoint: "SddIA/events/domain/telemetry-compliance-breached.md"
    proposal: "Clase ECST dominio breach"
  - id: "5.B"
    touchpoint: "execute_process_capsules.py, eda_bus_utils.py, raw-execution-finished.md"
    proposal: "Peaje fail-soft + telemetry_receipt + capsule_id"
  - id: "5.C"
    touchpoint: "telemetry_compliance_audit_core.py, telemetry-compliance-audit.md, event-telemetry-subscriptions.json"
    proposal: "Auditoría async dedicada; fan-out dual suscriptor"
  - id: "5.G"
    touchpoint: "stamp_fractal_delivery_state, maybe_purge_fractal_telemetry_when_terminal, route_fractal_event_core.py, radamanto_batch_core.py retrofix"
    proposal: "T5.6 Inmunidad Fan-Out — sellos delivery_state; purga infra"
  - id: "5.F"
    touchpoint: "test_telemetry_compliance.py, test_eda_fractal_bus.py"
    proposal: "QA AC5.x + regresión fan-out"
---

# Implementación — Fase 5

| Paso | Archivos | Cambio |
|------|----------|--------|
| 5.A | `skills-contract` v1.2.0, `actions-contract` v1.3.0, `text-metrics.md` | §6 termodinámica declarativa |
| 5.E′ | `telemetry-compliance-breached.md`, `domain/index.md` | Genoma dominio breach |
| 5.B | `eda_bus_utils`, `execute_process_capsules` | `extract_telemetry_receipt`, Peaje ampliado |
| 5.C | `telemetry_compliance_audit_core.py`, `telemetry-compliance-audit.md` | Proceso auditoría sin unlink |
| 5.G | `radamanto_batch_core.py`, `route_fractal_event_core.py` | Retrofix T5.6 + purga post-consenso |
| 5.F | `test_telemetry_compliance.py` | 10 tests nuevos |

Nuevos módulos: `telemetry_compliance_audit_core.py`.

Helpers SSOT: `resolve_ed_telemetry_contract`, `stamp_fractal_delivery_state`, `maybe_purge_fractal_telemetry_when_terminal`, `build_telemetry_compliance_breached_event`.
