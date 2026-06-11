---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
branch: feat/migracion-rust-wasi-certificacion
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/OPERATIVO-PBI-Migracion-Rust-WASI.md
checks:
  CA-1: pass
  CA-2: pass
  CA-3: pass
  CA-4: pass
  CA-5: pass
  CA-6: pass
  CA-7: pass
git_changes:
  - README.md
  - SddIA/core/eda-coverage.json
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/skills/skills-contract.md
  - SddIA/tools/tools-contract.md
  - SddIA/skills/skills-contract.md
  - docs/features/migracion-rust-wasi/
  - scripts/qa/verify-process-integrity.py
  - scripts/skills/cryptography-manager.py (eliminado)
  - scripts/skills/shell-executor.py (eliminado)
---

# Validación — migracion-rust-wasi

**Veredicto global: APTO**

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| CA-1 | Sin skill/tool requiere intérprete Python | ✅ | `cryptography-manager.py` + `shell-executor.py` eliminados; fallbacks retirados de orquestadores |
| CA-2 | `cargo build --workspace --target wasm32-wasip1` exit 0 | ✅ | Local + CI (`wasi-runtime-smoke` PR #84) |
| CA-3 | Contratos declaran Rust/WASI único | ✅ | `skills-contract.md` v1.3.0 §4; `tools-contract.md` v1.4.0 §7; `README.md` |
| CA-4 | Envelope JSON en error (no panic stdout) | ✅ | `sddia-io` centraliza emit_error / emit_success en todas las cápsulas |
| CA-5 | CI `wasi-runtime-smoke` + `eda-bus-e2e-smoke` verdes | ✅ | PR #84 — ambos jobs SUCCESS |
| CA-6 | Ciclo documental completo | ✅ | `clarify.md`, `spec.md`, `plan.md`, `implementation.md`, `validacion.md` |
| CA-7 | PBI archivado en `docs/todos/done/` | ✅ | `OPERATIVO-PBI-Migracion-Rust-WASI.md` movido en esta rama |

## Notas

- **Excepción D8 (documentada):** `scripts/skills/git-manager.py` y `bus-operator.py` permanecen como fallback funcional hasta que WASI soporte subprocess spawning sin flag experimental. Ver `clarify.md §D8`.
- `verify-process-integrity.py` migrado a `wasmtime run cryptography-manager.wasm` — OK.
- Warnings de Rust en `telegram-gateway`, `iota-immutable-publisher`, etc. son preexistentes y no críticos (no errores).
