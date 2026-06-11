---
document_id: PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK
title: "[Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime"
format: markdown
version: "1.0.0"
created: "2026-06-11"
status: in_progress
priority: alta
process: bug-fix
branch_name: fix/eda-bus-e2e-wasmtime-fallback
feature_ref: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
related:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/scripts/qa/run-eda-e2e-lab.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/execute-action.py
  - scripts/skills/cryptography-manager.py
  - docs/features/migracion-rust-wasi/implementation.md
blocks: "CI job eda-bus-e2e-smoke en verde sostenido tras migración WASI"
---

# [Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK` |
| **Estatus** | En ejecución |
| **Feature** | [`docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback`](../../fixes/kaizen-eda-bus-e2e-wasmtime-fallback/) |
| **Prioridad** | Alta — CI `main` en rojo sostenido |

## 1. Incidente

| Campo | Valor |
|-------|-------|
| **Síntoma** | Job CI `eda-bus-e2e-smoke` falla con `RuntimeError: [Errno 2] No such file or directory: 'wasmtime'` |
| **Workflow** | `.github/workflows/sddia-index-qa.yml` |
| **Comando** | `python SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json` |
| **Checks que pasan** | `verify-tools-index`, `eda-iota-smoke-simulate`, `eda-iota-physical` |

## 2. Causa raíz

Tras la migración Rust/WASI (PR #77), `entity-manager` invoca `crypto()` → `wasmtime run cryptography-manager.wasm`. El job CI no instala `wasmtime` ni compila artefactos WASM; el runner Ubuntu carece del runtime.

Paridad existente: `invoke_git_manager` ya dispone de fallback nativo (`git-manager.py`) cuando WASI no puede ejecutar git.

## 3. Solución

Añadir fallback laboratorio en `crypto()` / `_crypto()` hacia `scripts/skills/cryptography-manager.py` cuando:

- `wasmtime` no está en `PATH`, o
- el artefacto `.wasm` no existe.

Preferir WASI cuando ambos estén disponibles (sin regresión local).

## 4. Criterios de aceptación

| ID | Criterio | Estado |
|----|----------|--------|
| KZ-CA1 | Smoke local sin wasmtime en PATH → `success: true` | ⏳ |
| KZ-CA2 | Smoke local con wasmtime → sin regresión | ⏳ |
| KZ-CA3 | CI `eda-bus-e2e-smoke` SUCCESS en PR | ⏳ |
| KZ-CA4 | Paridad documental fix + `validacion.md` APTO | ⏳ |
