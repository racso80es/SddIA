---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
pbi_ref: docs/todos/pending/[Kaizen] CI WASI — wasmtime y build workspace en runner.md
continues_from: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
---

# Objetivos — ci-wasi-runtime-validation

Validar empíricamente en GitHub Actions la ruta WASI completa post-migración Rust (PR #77), sin eliminar el fallback Python del Kaizen PR #83.

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Toolchain CI | Rust + `wasm32-wasip1` + `wasmtime` en runner |
| O2 | Build workspace | `cargo build --workspace --target wasm32-wasip1` exit 0 |
| O3 | Smoke WASI | Job `wasi-runtime-smoke` SUCCESS |
| O4 | Sin regresión | `eda-bus-e2e-smoke` sigue verde con fallback |
| O5 | Trazabilidad | `SDDIA_CI_REQUIRE_WASI=1` falla si se usaría fallback |
