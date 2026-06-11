---
feature_name: kaizen-eda-bus-e2e-wasmtime-fallback
created: "2026-06-11"
process: bug-fix
branch_name: fix/eda-bus-e2e-wasmtime-fallback
persist_ref: docs/fixes/kaizen-eda-bus-e2e-wasmtime-fallback
---

# Implementación — kaizen eda-bus-e2e-wasmtime-fallback

## Touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute_process_capsules.py` | `_parse_crypto_envelope`, `_invoke_crypto_native`, `_crypto_wasm_ready`, `crypto()` |
| `SddIA/scripts/qa/execute-action.py` | Paridad en `_crypto()` + `import shutil` |

## Lógica

```text
crypto(payload)
  ├─ wasm + wasmtime disponibles → wasmtime run cryptography-manager.wasm
  └─ else → python scripts/skills/cryptography-manager.py (stdin JSON)
```

## Verificación local

```bash
PATH="/usr/bin:/bin" \
  SDDIA_LAB_SIMULATE_IOTA=1 SDDIA_LAB_SIMULATE_SYNC_INDEX=1 SDDIA_LAB_ROUTE_SYNC=1 \
  python3 SddIA/scripts/qa/run-eda-e2e-lab.py --entity-class tool --json
# → "success": true
```
