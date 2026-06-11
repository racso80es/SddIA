---
feature_name: kaizen-eda-bus-e2e-wasmtime-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/eda-bus-e2e-wasmtime-fallback
persist_ref: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
---

# Especificación — fallback cryptography-manager sin wasmtime

## Problema

`eda-bus-e2e-smoke` ejecuta `run-eda-e2e-lab.py` → `entity-manager` → `emit_domain_mutation` → `crypto(GENERATE_UUID)`. Tras migración WASI, `crypto()` exige `wasmtime` y `cryptography-manager.wasm`; CI no los provee.

## Cambio

| Archivo | Modificación |
|---------|--------------|
| `execute_process_capsules.py` | `_invoke_crypto_native`, `_crypto_wasm_ready`, `crypto()` con fallback |
| `execute-action.py` | Misma paridad en `_crypto()` |

## Reglas de enrutamiento

1. Si `cryptography-manager.wasm` existe **y** `wasmtime` en PATH → WASI.
2. En caso contrario → `scripts/skills/cryptography-manager.py` vía stdin/stdout JSON.
3. Parseo unificado de envelope (`result` plano o `data.result`).

## No objetivos (Fase 1 — cerrada)

- Instalar Rust/wasmtime en CI → **delegado a Fase 2:** [`PBI-KAIZEN-CI-WASI-RUNTIME-BUILD`](../../../todos/pending/[Kaizen]%20CI%20WASI%20—%20wasmtime%20y%20build%20workspace%20en%20runner.md)
- Alterar contrato ECST ni flujo EDA del lab.

## Fase 2 — continuación

Ver PBI pendiente `PBI-KAIZEN-CI-WASI-RUNTIME-BUILD` y feature objetivo `docs/features/ci-wasi-runtime-validation/`.
