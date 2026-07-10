---
document_id: PBI-FIX-ROUTE-DOMAIN-RUST
title: "[FIX] Porte route-domain-event core a Rust (eliminar route bridge)"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "abierto"
priority: alta
process: bug-fix
related:
  - docs/features/migracion-execute-process-rust/implementation.md
  - SddIA/scripts/qa/_execute_process_route_bridge.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/engine/execute-process/src/engine/handlers/route_domain.rs
---

# [FIX] Porte `route-domain-event` core a Rust (eliminar route bridge)

## Contexto

P17 cerró el entrypoint orquestador binario-only. La lógica ECST/fan-out de enrutamiento EDA sigue en Python vía `python_core::invoke_route_domain_event` → `route_domain_event_core.py` (wrapper `_execute_process_route_bridge.py` eliminado en P16).

## Objetivo

Portar `route_domain_event_core.py` a `engine::handlers::route_domain` (o módulo dedicado) con paridad de envelope y comportamiento sync/async (`SDDIA_LAB_ROUTE_SYNC`).

## Criterio de cierre

- Golden `route-domain-event` verde sin bridge wrapper Python.
- Smoke `eda-e2e-lab` 8/8.
- ~~Eliminar `_execute_process_route_bridge.py` y referencias en Rust.~~ ✅ (P16)
- Porte full nativo de `route_domain_event_core.py` a Rust.
- `grep` limpio de `route_domain_event_core` en path de orquestación.

## Gate

Post-P17 deuda explícita (§7.10 implementation.md).
