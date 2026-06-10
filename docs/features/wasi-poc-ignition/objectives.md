---
feature_name: wasi-poc-ignition
created: "2026-06-01"
process: feature
branch_name: feat/wasi-poc-ignition
persist_ref: docs/features/wasi-poc-ignition
---

# Objetivos — wasi-poc-ignition

## Misión

Proof of concept for WASI tool compilation and IO sandbox validation

## Alcance (manifiesto)

- Cápsula Rust `wasi-poc` compilable a `wasm32-wasip1`.
- Ejecución vía Wasmtime con envelope `capsule-json-io` v2.0.
- Scripts `build-wasi.sh` / `run-wasi.sh` como puente mínimo.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
