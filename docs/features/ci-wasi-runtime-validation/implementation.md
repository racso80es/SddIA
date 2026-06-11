---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
---

# Implementación — ci-wasi-runtime-validation

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `.github/workflows/sddia-index-qa.yml` | Job `wasi-runtime-smoke` |
| `SddIA/scripts/qa/run-wasi-ci-smoke.py` | Orquestador CI WASI |
| `SddIA/scripts/qa/execute_process_capsules.py` | `SDDIA_CI_REQUIRE_WASI`, `_raise_if_wasi_required` |
| `SddIA/scripts/qa/execute-action.py` | Paridad `_crypto()` |

## Flujo del smoke

1. Verifica `wasmtime` en PATH.
2. Exige `cryptography-manager.wasm` y `wasi-poc.wasm` en `SddIA/target/wasm32-wasip1/debug/`.
3. Ejecuta ambas cápsulas vía `wasmtime run`.
4. Ejecuta `run-eda-e2e-lab.py` con `SDDIA_CI_REQUIRE_WASI=1`.

## Verificación local

```bash
# Tras cargo build --workspace --target wasm32-wasip1 en SddIA/
SDDIA_CI_REQUIRE_WASI=1 \
  SDDIA_LAB_SIMULATE_IOTA=1 SDDIA_LAB_SIMULATE_SYNC_INDEX=1 SDDIA_LAB_ROUTE_SYNC=1 \
  python3 SddIA/scripts/qa/run-wasi-ci-smoke.py --json
```
