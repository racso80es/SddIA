---
feature_name: route-domain-rust-port
created: "2026-07-11"
process: bug-fix
base: main
scope: route-domain-event-native
document_id: PBI-FIX-ROUTE-DOMAIN-RUST
version_spec: "1.0.0"
---

# Especificación — Porte route-domain-event core a Rust

## Diagnóstico

| Síntoma | Evidencia |
|---------|-----------|
| Core EDA `route_domain_event_core.py` invocado vía `python_core::invoke_route_domain_event` | Subprocess Python en path orquestador post-P17 |
| Deuda explícita §7.10 `migracion-execute-process-rust` | Gate post-P17 pendiente |

## Corrección

1. Portar `route_domain_event_core.py` → `engine/route_domain_core.rs`.
2. Extender bus EDA → `engine/eda_bus_topology.rs` (testigos, sweep, PR lifecycle).
3. `handlers/route_domain.rs` invoca core nativo; eliminar `invoke_route_domain_event` de `python_core.rs`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Sin `invoke_route_domain_event` ni `route_domain_event_core` en path orquestación Rust |
| CA2 | Golden `route-domain-event` verde |
| CA3 | Golden orchestrator 14/14 |
| CA4 | Smoke `eda-e2e-lab` verde |
| CA5 | `cargo test -p execute-process --lib` verde |
| CA6 | Paridad sync/async `SDDIA_LAB_ROUTE_SYNC` |
