---
document_id: PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK
title: "[Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime"
format: markdown
version: "1.0.0"
created: "2026-06-11"
status: done
priority: alta
process: bug-fix
closed: "2026-06-11"
branch_name: fix/eda-bus-e2e-wasmtime-fallback
feature_ref: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
merged_pr: 83
origin: docs/todos/pending/[Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime.md
---

# [Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK` |
| **Estatus** | ✅ Done |
| **Fix** | [`docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback/`](../../fixes/kaizen-eda-bus-e2e-wasmtime-fallback/) |
| **Validación** | [`validacion.md`](../../fixes/kaizen-eda-bus-e2e-wasmtime-fallback/validacion.md) — APTO |
| **PR** | #83 |

## 1. Incidente

Job CI `eda-bus-e2e-smoke` fallaba con `RuntimeError: [Errno 2] No such file or directory: 'wasmtime'` tras migración WASI (PR #77).

## 2. Causa raíz

`entity-manager` → `crypto()` exige `wasmtime` + `cryptography-manager.wasm`; el runner GitHub Actions no instala runtime ni compila artefactos.

## 3. Solución aplicada

Fallback nativo a `scripts/skills/cryptography-manager.py` en `execute_process_capsules.crypto()` y `execute-action._crypto()`, siguiendo el patrón de `git-manager.py`.

## 4. Criterios de aceptación

| ID | Criterio | Estado |
|----|----------|--------|
| KZ-CA1 | Smoke sin wasmtime en PATH | ✅ |
| KZ-CA2 | Sin regresión con wasmtime | ✅ |
| KZ-CA3 | CI `eda-bus-e2e-smoke` SUCCESS | ✅ |
| KZ-CA4 | Paridad documental | ✅ |
