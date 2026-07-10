---
feature_name: capsules-bridge-rust-port
created: "2026-07-10"
process: bug-fix
items:
  - id: R1
    artifact: SddIA/engine/execute-process/src/engine/residual_runner.rs
    nature: rust-module
    operation: create
  - id: R2
    artifact: SddIA/engine/execute-process/src/engine/accept_pr.rs
    nature: rust-handler
    operation: create
  - id: R3
    artifact: SddIA/engine/execute-process/src/engine/python_core.rs
    nature: rust-bridge-cores
    operation: create
  - id: R4
    artifact: SddIA/engine/execute-process/src/engine/delegate_python.rs
    nature: rust-module
    operation: delete
  - id: R5
    artifact: SddIA/scripts/qa/_execute_process_capsules_bridge.py
    nature: python-bridge
    operation: delete
---

# Implementación — Porte capsules bridge

## Inventario procesos portados vs. retirados

| Proceso / familia | Handler Rust | Notas |
|-------------------|--------------|-------|
| `accept-pr` | `accept_pr.rs` | Fases merge/sello nativas |
| `*-creator` | `residual_runner` + `materialize_by_inputs` | Forja nativa; `daemon-creator` simulado |
| `execute-suite` | `residual_runner` | Hijos vía `invoke_process_full` recursivo |
| `telemetry-batch-stub` | `python_core` + Rust | Purga evento nativa |
| `radamanto-batch` | `python_core` | Core Python puntual (deuda fan-out) |
| `telemetry-compliance-audit` | `python_core` | Core Python puntual |
| `route-telemetry/orchestration/domain` | `python_core` | Fan-out fractal vía core Python |
| `pull-request-review` | `residual_runner` genérico | Fases agente simuladas; cápsulas vía `phase_capsules` |
| `feature/bug-fix/refactorization` | `executor.rs` | Sin cambio (ya nativo) |
| Handlers satélite | `handlers/*` | Sin cambio |

## Eliminados

- `delegate_python.rs`
- `invoke_capsules_bridge()` en `invoke_orchestrator.rs`
- `_execute_process_capsules_bridge.py`
