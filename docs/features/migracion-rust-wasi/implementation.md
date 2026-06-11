---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
branch_name: feat/migracion-rust-wasi-certificacion
items:
  - SddIA/Cargo.toml
  - SddIA/sddia-io/
  - SddIA/skills/bus-operator/
  - SddIA/skills/cryptography-manager/
  - SddIA/skills/git-manager/
  - SddIA/skills/shell-executor/
  - SddIA/tools/io-choke/
  - SddIA/tools/iota-immutable-publisher/
  - SddIA/tools/manage-event-receipt/
  - SddIA/tools/markdown-table-editor/
  - SddIA/tools/read-event-subscriptions/
  - SddIA/tools/sandbox-breacher/
  - SddIA/tools/schema-corruptor/
  - SddIA/tools/send-telegram-notification/
  - SddIA/tools/telegram-gateway/
  - SddIA/tools/transit-event-payload/
  - SddIA/tools/wasi-poc/
  - SddIA/scripts/qa/execute_process_capsules.py
  - scripts/skills/git-manager.py
  - scripts/skills/cryptography-manager.py
  - scripts/skills/bus-operator.py
  - scripts/skills/shell-executor.py
  - SddIA/skills/skills-contract.md
  - SddIA/tools/tools-contract.md
  - README.md
  - docs/todos/pending/OPERATIVO-PBI-Migracion-Rust-WASI.md
---

# Registro de materialización — migracion-rust-wasi

## Estado: EN CURSO (certificación)

Feature reiniciada en rama `feat/migracion-rust-wasi-certificacion` (2026-06-11).

### Completado (heredado de main)

| Touchpoint | Estado |
|------------|--------|
| Cargo workspace + `sddia-io` | ✅ |
| 4 skills + 12 tools en Rust | ✅ |
| Build `wasm32-wasip1` | ✅ |
| CI `wasi-runtime-smoke` (PR #84) | ✅ |
| PoC `wasi-poc-ignition` (PR #74) | ✅ |

### Ejecutado (esta rama)

| Touchpoint | Estado | Detalle |
|------------|--------|---------|
| Eliminar `scripts/skills/shell-executor.py` | ✅ | Sin referencias en ningún orquestador |
| Eliminar `scripts/skills/cryptography-manager.py` | ✅ | Migrada a WASM en los 3 puntos de uso |
| Migrar `scripts/qa/verify-process-integrity.py` | ✅ | wasmtime + cryptography-manager.wasm (parser `result` corregido) |
| Retirar `_invoke_crypto_native` de `execute_process_capsules.py` | ✅ | Ruta WASI obligatoria con error explícito |
| Retirar `_invoke_crypto_native` de `execute-action.py` | ✅ | Idem |
| `skills-contract.md` v1.3.0 | ✅ | §4 sustrato Rust/WASI + `execution_substrate: rust-wasi` |
| `tools-contract.md` v1.4.0 | ✅ | §7 sustrato Rust/WASI + historial |
| `README.md` | ✅ | Eliminar "Python permitido" de cápsulas skill |
| `cargo build --workspace --target wasm32-wasip1` | ✅ | Exit 0, warnings menores no críticos |
| `run-wasi-ci-smoke.py` | ✅ | `wasi_path_verified: true` |
| `verify-process-integrity.py` | ✅ | `OK` |

### Deuda técnica documentada (excepción D8)

| Touchpoint | Estado | Motivo |
|------------|--------|--------|
| `scripts/skills/git-manager.py` | Permanece | `git-manager.wasm` llama `git` subprocess; WASI no soporta sin flag experimental |
| `scripts/skills/bus-operator.py` | Permanece | `bus-operator.wasm` llama `wasmtime` subprocess; misma limitación |

Pendiente: `validacion.md` APTO + PBI → `docs/todos/done/`.