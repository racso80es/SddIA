---
uuid: a7b3c4d5-e6f7-4890-a1b2-c3d4e5f67890
entity_ref: PBI-FIX-CAPSULES-BRIDGE-RUST
type: bug-fix
version: "1.0.0"
created: "2026-07-10"
---

# Evolución — Porte capsules bridge a Rust

Eliminados `delegate_python` y `_execute_process_capsules_bridge.py`. Motor residual nativo (`residual_runner`, `accept_pr`, `python_core` para cores EDA puntuales). Golden 14/14; cargo test 45/45.

**persist_ref:** `docs/fixes/capsules-bridge-rust-port/`
