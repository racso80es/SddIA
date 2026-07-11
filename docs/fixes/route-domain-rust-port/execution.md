---
feature_name: route-domain-rust-port
created: "2026-07-11"
process: bug-fix
items_applied:
  - route-domain-core-rust
  - eda-bus-topology-rust
  - invoke-route-domain-removed
---

# Ejecución — Porte route-domain-event core

## Cambios aplicados

| Artefacto | Acción |
|-----------|--------|
| `engine/route_domain_core.rs` | Núcleo ECST/fan-out nativo (~900 líneas paridad Python) |
| `engine/eda_bus_topology.rs` | Bus V3+, testigos, sweep, PR lifecycle |
| `engine/handlers/route_domain.rs` | Handler → core Rust |
| `engine/python_core.rs` | Eliminado `invoke_route_domain_event` |
| `engine/mod.rs` | Registro módulos nuevos |

## Verificación

| Prueba | Resultado |
|--------|-----------|
| `golden_orchestrator_parity.py` (route-domain-event) | ✅ |
| `golden_orchestrator_parity.py` (full) | ✅ 14/14 |
| `run-eda-e2e-lab.py` | ✅ |
| `cargo test -p execute-process --lib` | ✅ 45/45 |
| `grep route_domain_event_core` en `SddIA/engine/` | ✅ limpio |

## Deuda residual

- `route_domain_event_core.py` consumido por `execute_process_capsules.py` (fan-out interno legacy).
- Cores fractal/radamanto/telemetry siguen en `python_core.rs`.
