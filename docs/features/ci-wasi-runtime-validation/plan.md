---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
---

# Plan — ci-wasi-runtime-validation

## Fase 1 — Infraestructura CI

- [x] Añadir job `wasi-runtime-smoke` en `sddia-index-qa.yml`
- [x] Rust toolchain + target `wasm32-wasip1`
- [x] Instalación `wasmtime` + exposición en `GITHUB_PATH`
- [x] Cache Cargo (`SddIA/Cargo.lock` como clave)

## Fase 2 — Smoke script

- [x] Crear `SddIA/scripts/qa/run-wasi-ci-smoke.py`
- [x] Verificar artefactos mínimos post-build
- [x] Invocar `wasi-poc` vía wasmtime (workspace debug)

## Fase 3 — Enforcement WASI

- [x] Implementar `SDDIA_CI_REQUIRE_WASI` en `crypto()` / `_crypto()`
- [x] Integrar `run-eda-e2e-lab.py` bajo flag en smoke script

## Fase 4 — Cierre

- [x] `validacion.md` APTO + `pbi_archived: true`
- [x] Mover PBI pending → done en mismo PR
