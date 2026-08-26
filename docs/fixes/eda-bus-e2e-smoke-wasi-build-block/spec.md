---
feature_name: eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
process: bug-fix
branch_name: fix/eda-bus-e2e-smoke-wasi-build-block
persist_ref: docs/fixes/eda-bus-e2e-smoke-wasi-build-block
pbi_ref: docs/todos/pending/[FIX] eda-bus-e2e-smoke.md
related_pbi: docs/todos/pending/[Fix] wasi-runtime-somke.md
---

# Especificación — build WASI selectivo en CI

## Problema

`cargo build --workspace --target wasm32-wasip1` arrastra `email-watcher` (IMAP/TLS → `openssl-sys`) al grafo WASI. Los jobs `wasi-runtime-smoke` y `eda-bus-e2e-smoke` fallan en exit 101 antes de ejecutar sus smokes.

## Solución

Script `SddIA/scripts/qa/build-wasi-capsules.sh` que compila solo cápsulas de dominio (`skills/`, `tools/`, `interfaces/`) con `src/main.rs`, excluyendo:

- `execute-process`, `sddia-qa` (nativos)
- Centinelas: `event-watcher`, `event-sweeper`, `email-watcher`, `telegram-watcher`, `github-bridge-watcher`

Contrato: **daemon nativo ≠ miembro del grafo WASI CI** (`DT-WASI-NATIVE-DAEMON-SPLIT`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | `eda-bus-e2e-smoke` SUCCESS — `run-eda-e2e-lab` + `event-sweeper --once` |
| CA-2 | `wasi-runtime-smoke` SUCCESS — `run-wasi-ci-smoke` + `gate-evolution` |
| CA-3 | Build CI no incluye `email-watcher` en target WASI |
| CA-4 | Sin regresión en jobs nativos (`verify-tools-index`, `eda-iota-smoke-simulate`) |
