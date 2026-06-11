---
feature_name: migracion-rust-wasi
created: "2026-06-11"
process: feature
branch_name: feat/migracion-rust-wasi-certificacion
persist_ref: docs/features/migracion-rust-wasi
---

# Especificación — Certificación migración Rust/WASI

## 1. Propósito

Completar la migración de cápsulas ejecutables (skills y tools) declarada en el PBI `OPERATIVO-PBI-Migracion-Rust-WASI`. La forja Rust ya está en `main`; esta feature certifica paridad funcional, elimina legado Python y actualiza la normativa.

## 2. Fronteras

| In scope | Out of scope |
|----------|--------------|
| Poda `scripts/skills/*.py` | Reescritura de cápsulas ya migradas |
| Fallback removal en `execute_process_capsules.py` | Migración de `execute-process.py` a Rust |
| Contratos + README | Adapters LanceDB (`infrastructure/adapters/`) |
| `validacion.md` + cierre PBI | Transpilador de Intenciones (PBI Snapshot) |

## 3. Arquitectura de ejecución

```text
Orquestador (execute-process.py)
    └── wasmtime run --dir=. SddIA/target/wasm32-wasip1/debug/{capsule}.wasm
            └── stdin/stdout JSON (capsule-json-io v2)
```

**Skills:** `bus-operator`, `cryptography-manager`, `git-manager`, `shell-executor`  
**Tools:** 12 crates bajo `SddIA/tools/*` + `wasi-poc`

## 4. Criterios de aceptación (S+ Grade)

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA-1 | Ninguna skill/tool requiere intérprete Python | Grep + smoke sin fallback |
| CA-2 | `cargo build --workspace --target wasm32-wasip1` exit 0 | CI + local |
| CA-3 | Contratos declaran Rust/WASI único | Diff en `*-contract.md`, README |
| CA-4 | Envelope JSON en error (no panic stdout) | Revisión `sddia-io` + smokes |
| CA-5 | CI `wasi-runtime-smoke` + `eda-bus-e2e-smoke` verdes | GitHub Actions |
| CA-6 | Ciclo documental completo bajo `persist_ref` | Argos / validacion.md APTO |
| CA-7 | PBI archivado en `docs/todos/done/` | Mismo PR |

## 5. Dependencias

- PoC: `docs/features/wasi-poc-ignition/` (APTO, PR #74)
- CI: `docs/features/ci-wasi-runtime-validation/` (APTO, PR #84)
- Norma I/O: `SddIA/norms/capsule-json-io.md`
- Workspace: `SddIA/Cargo.toml`, `SddIA/sddia-io/`
