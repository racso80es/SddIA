---
feature_name: route-domain-rust-port
created: "2026-07-11"
process: bug-fix
items:
  - id: RD1
    artifact: SddIA/engine/execute-process/src/engine/route_domain_core.rs
    nature: rust-module
    operation: create
  - id: RD2
    artifact: SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
    nature: rust-module
    operation: create
  - id: RD3
    artifact: SddIA/engine/execute-process/src/engine/handlers/route_domain.rs
    nature: rust-handler
    operation: update
  - id: RD4
    artifact: SddIA/engine/execute-process/src/engine/python_core.rs
    nature: rust-bridge-cores
    operation: update
  - id: RD5
    artifact: SddIA/engine/execute-process/src/engine/mod.rs
    nature: rust-module
    operation: update
---

# Implementación — Porte route-domain-event nativo

## Módulos

| Artefacto | Responsabilidad |
|-----------|-----------------|
| `route_domain_core.rs` | ECST gate, fan-out suscriptores, dispatch process/action/tool, sync/async |
| `eda_bus_topology.rs` | Topología V3+, testigos, sweep, PR lifecycle, persist_ref inference |
| `handlers/route_domain.rs` | Entry handler → `route_domain_core::route_domain_event` |

## Eliminado del path orquestador

- `python_core::invoke_route_domain_event`

## Residual Python (fuera de alcance)

- `route_domain_event_core.py` permanece para QA/scripts legacy (`execute_process_capsules.py` fan-out interno).
- `route_fractal_event_core.py`, `radamanto_batch_core.py`, `telemetry_compliance_audit_core.py` vía `python_core.rs`.
