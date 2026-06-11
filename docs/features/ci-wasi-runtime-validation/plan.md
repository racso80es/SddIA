---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
---

# Plan — ci-wasi-runtime-validation

## Fase 1 — Infraestructura CI

- [ ] Añadir job `wasi-runtime-smoke` en `sddia-index-qa.yml`
- [ ] Rust toolchain + target `wasm32-wasip1`
- [ ] Instalación `wasmtime` + exposición en `GITHUB_PATH`
- [ ] Cache Cargo (`SddIA/Cargo.lock` como clave)

## Fase 2 — Smoke script

- [ ] Crear `SddIA/scripts/qa/run-wasi-ci-smoke.py`
- [ ] Verificar artefactos mínimos post-build
- [ ] Invocar `wasi-poc` vía `run-wasi.sh`

## Fase 3 — Enforcement WASI

- [ ] Implementar `SDDIA_CI_REQUIRE_WASI` en `crypto()` / `_crypto()`
- [ ] Integrar `run-eda-e2e-lab.py` bajo flag en smoke script

## Fase 4 — Cierre

- [ ] `validacion.md` APTO + `pbi_archived: true`
- [ ] Mover PBI pending → done en mismo PR
