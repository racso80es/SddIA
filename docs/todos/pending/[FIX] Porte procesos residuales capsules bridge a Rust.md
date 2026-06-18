---
document_id: PBI-FIX-CAPSULES-BRIDGE-RUST
title: "[FIX] Porte procesos residuales capsules bridge a Rust"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "abierto"
priority: media
process: bug-fix
related:
  - docs/features/migracion-execute-process-rust/implementation.md
  - SddIA/scripts/qa/_execute_process_capsules_bridge.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/engine/execute-process/src/engine/delegate_python.rs
---

# [FIX] Porte procesos residuales capsules bridge a Rust

## Contexto

Tras P17, `delegate_python` delega únicamente a `_execute_process_capsules_bridge.py` para procesos no portados: creators complejos, radamanto, telemetry, accept-pr, etc.

`entity-manager` ya es nativo; el bridge residual mantiene vivo `execute_process_capsules.py` y consumo PyYAML en subprocess interno.

## Objetivo

Portar procesos restantes a handlers nativos Rust o invocación binaria recursiva sin bridge Python.

## Criterio de cierre

- `delegate_python` eliminado o vacío (sin subprocess Python).
- Eliminar `_execute_process_capsules_bridge.py`.
- Golden 14/14 + smokes E2E verdes.
- Inventario documentado de procesos portados vs. retirados.

## Gate

Post-P17 deuda explícita (§7.10 implementation.md).
