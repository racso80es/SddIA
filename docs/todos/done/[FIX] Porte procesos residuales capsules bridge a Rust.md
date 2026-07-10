---
document_id: PBI-FIX-CAPSULES-BRIDGE-RUST
title: "[FIX] Porte procesos residuales capsules bridge a Rust"
format: markdown
version: "1.0.0"
created: "2026-06-18"
closed: "2026-07-10"
status: "cerrado"
pr_url: https://github.com/racso80es/SddIA/pull/102
priority: media
process: bug-fix
related:
  - docs/features/migracion-execute-process-rust/implementation.md
  - docs/fixes/capsules-bridge-rust-port/execution.md
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
---

# [FIX] Porte procesos residuales capsules bridge a Rust

## Contexto

Tras P17, `delegate_python` delegaba únicamente a `_execute_process_capsules_bridge.py` para procesos no portados: creators complejos, radamanto, telemetry, accept-pr, etc.

`entity-manager` ya es nativo; el bridge residual mantenía vivo `execute_process_capsules.py` y consumo PyYAML en subprocess interno.

## Objetivo

Portar procesos restantes a handlers nativos Rust o invocación binaria recursiva sin bridge Python.

## Criterio de cierre

- `delegate_python` eliminado o vacío (sin subprocess Python).
- Eliminar `_execute_process_capsules_bridge.py`.
- Golden 14/14 + smokes E2E verdes.
- Inventario documentado de procesos portados vs. retirados.

## Resolución

Motor `residual_runner.rs` + `accept_pr.rs` reemplazan bridge. Golden 14/14, cargo test 45/45. Cores EDA puntuales en `python_core.rs` (deuda fan-out). `execute_process_capsules.py` permanece solo para fan-out interno EDA.
