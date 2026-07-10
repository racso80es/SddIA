---
feature_name: capsules-bridge-rust-port
created: "2026-07-10"
process: bug-fix
items_applied:
  - residual-runner-native
  - accept-pr-native
  - capsules-bridge-deleted
  - delegate-python-deleted
---

# Ejecución — Porte capsules bridge

## Cambios aplicados

| Artefacto | Acción |
|-----------|--------|
| `engine/residual_runner.rs` | Motor residual nativo (catch-all post-handlers) |
| `engine/accept_pr.rs` | Proceso `accept-pr` nativo |
| `engine/python_core.rs` | Invocación puntual cores EDA (route fractal, radamanto, telemetry compliance) |
| `engine/mod.rs` | Ruta final → `residual_runner::run` |
| `delegate_python.rs` | Eliminado |
| `_execute_process_capsules_bridge.py` | Eliminado |
| `invoke_capsules_bridge` | Eliminado de `invoke_orchestrator.rs` |

## Verificación

| Prueba | Resultado |
|--------|-----------|
| `golden_orchestrator_parity.py` | ✅ 14/14 |
| `cargo test -p execute-process --lib` | ✅ 45/45 |
| `orchestrator_touchpoint_e2e_smoke.py` | 7/8 — `kalma2-bridge` falla por `.SddIA/client/sddia-client-bridge.py` ausente (preexistente) |

## Deuda residual

- `execute_process_capsules.py` permanece para fan-out interno EDA (`route_fractal_event_core` import directo).
- Cores Python en `python_core.rs` para radamanto/telemetry/route-fractal hasta porte full nativo.
- `daemon-creator`: forja simulada hasta handler dedicado.

## Impacto en P16 (PyYAML)

El orquestador **ya no** invoca subprocess Python vía capsules bridge ni route bridge wrapper. Gate P16 cerrado (ver `docs/fixes/p16-pyyaml-poda/execution.md`).
