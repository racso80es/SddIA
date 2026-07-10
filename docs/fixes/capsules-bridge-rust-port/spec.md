---
feature_name: capsules-bridge-rust-port
created: "2026-07-10"
process: bug-fix
base: main
scope: capsules-bridge-elimination
document_id: PBI-FIX-CAPSULES-BRIDGE-RUST
version_spec: "1.0.0"
---

# Especificación — Porte capsules bridge a Rust

## Diagnóstico

| Síntoma | Evidencia |
|---------|-----------|
| `delegate_python` delegaba a `_execute_process_capsules_bridge.py` | Subprocess Python + PyYAML en orquestación interna |
| Procesos residuales sin handler nativo | creators, accept-pr, execute-suite, route-fractal, radamanto, telemetry |

## Corrección

1. Motor `residual_runner.rs` nativo reemplaza `delegate_python`.
2. Handlers dedicados: `accept_pr.rs`, `python_core.rs` (cores EDA puntuales).
3. Eliminar `_execute_process_capsules_bridge.py` e `invoke_capsules_bridge`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Sin `delegate_python` ni `_execute_process_capsules_bridge.py` |
| CA2 | Golden 14/14 verde |
| CA3 | Smokes E2E verdes (salvo env preexistente) |
| CA4 | `cargo test -p execute-process --lib` verde |
| CA5 | Inventario portado documentado en `execution.md` |
