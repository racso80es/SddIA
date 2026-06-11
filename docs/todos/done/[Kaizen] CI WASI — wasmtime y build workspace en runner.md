---
document_id: PBI-KAIZEN-CI-WASI-RUNTIME-BUILD
title: "[Kaizen] CI WASI — wasmtime y build workspace en runner"
format: markdown
version: "1.0.0"
created: "2026-06-11"
status: done
priority: alta
process: feature
closed: "2026-06-11"
branch_name: feat/ci-wasi-runtime-validation
feature_ref: docs/features/ci-wasi-runtime-validation
merged_pr: 84
continues_from: PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK
origin: docs/todos/pending/[Kaizen] CI WASI — wasmtime y build workspace en runner.md
---

# [Kaizen] CI WASI — wasmtime y build workspace en runner

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-CI-WASI-RUNTIME-BUILD` |
| **Estatus** | ✅ Done |
| **Precedente** | `PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK` (PR #83) |
| **Feature** | [`docs/features/ci-wasi-runtime-validation/`](../../features/ci-wasi-runtime-validation/) |
| **Validación** | [`validacion.md`](../../features/ci-wasi-runtime-validation/validacion.md) — APTO |
| **PR** | #84 |

## Objetivo

Validar en GitHub Actions la ruta WASI completa (build workspace + `wasmtime` + cápsulas), complementando el fallback Python del Kaizen PR #83.

## Entregables

| Artefacto | Ruta |
|-----------|------|
| Job CI | `wasi-runtime-smoke` en `.github/workflows/sddia-index-qa.yml` |
| Smoke script | `SddIA/scripts/qa/run-wasi-ci-smoke.py` |
| Flag enforcement | `SDDIA_CI_REQUIRE_WASI` en `crypto()` / `_crypto()` |

## Criterios de aceptación

| ID | Criterio | Estado |
|----|----------|--------|
| CI-W1 | Job `wasi-runtime-smoke` | ✅ |
| CI-W2 | Build workspace WASI | ✅ (~28–32s PR #84) |
| CI-W3 | `wasmtime` en PATH | ✅ |
| CI-W4 | `wasi-poc` ejecuta | ✅ |
| CI-W5 | Crypto WASM sin fallback | ✅ |
| CI-W6 | `eda-bus-e2e-smoke` sin regresión | ✅ |
| CI-W8 | Paridad documental APTO | ✅ |
| CI-W9 | PBI en `done/` | ✅ |
