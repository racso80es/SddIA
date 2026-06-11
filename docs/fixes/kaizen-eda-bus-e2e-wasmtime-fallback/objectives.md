---
feature_name: kaizen-eda-bus-e2e-wasmtime-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/eda-bus-e2e-wasmtime-fallback
persist_ref: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
bug_summary: CI eda-bus-e2e-smoke falla — wasmtime ausente tras migración WASI
---

# Objetivos — kaizen eda-bus-e2e-wasmtime-fallback

Restaurar `eda-bus-e2e-smoke` con fallback nativo de `cryptography-manager`, alineado al patrón `git-manager.py` ya existente.

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Fallback crypto | `crypto()` opera sin wasmtime |
| O2 | Sin regresión WASI | Con wasmtime+wasm sigue usando cápsula |
| O3 | CI verde | Job `eda-bus-e2e-smoke` SUCCESS |
