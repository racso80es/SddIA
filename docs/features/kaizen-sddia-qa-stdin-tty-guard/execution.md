---

feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
phase: ejecucion
status: executed
verdict: executed
norm_ref: features-documentation-pattern@1.3.1
agent: tekton
items_applied:
  - Ola 0
  - Ola 1
  - Ola 2
  - Ola 3
  - Ola 4
phases:
  Ola 0:
    status: closed
  Ola 1:
    status: closed
  Ola 2:
    status: closed
  Ola 3:
    status: closed
  Ola 4:
    status: closed
execution_id: "c1910c23-3ebf-456b-a60b-9c3aa30884a5"
---
# Registro de Ejecución — kaizen-sddia-qa-stdin-tty-guard

## Evidencia de Verificación

| Check / Métrica | Resultado |
|-----------------|-----------|
| `cargo test -p sddia-io` | **OK** (`6 passed; 0 failed`) |
| `cargo test -p sddia-evolution-register` | **OK** (`26 passed; 0 failed`) |
| `cargo test -p sddia-qa gate_` | **OK** (`13 passed; 0 failed`) |
| `RUSTFLAGS="-Dwarnings" cargo check --workspace` | **OK** |
| `cargo check --target wasm32-wasip1 -p sddia-evolution-register` | **OK** |
| `cargo build --release -p sddia-qa` | **OK** |
| **K-TTY** (PTY real `python3 pty.fork` sobre `sddia-qa evolution-register --json`) | **OK** (`PTY_EXIT=1`, `ELAPSED_MS=6.7`, JSON `success: false`) |
| **K-HELP** (`sddia-qa --help`) | **OK** (declara `--request-file <path>`, `SDDIA_CAPSULE_REQUEST` y `stdin no-TTY`) |
| `verify-process-integrity` | **OK** |
| `gate-event-manager-port --json` | **OK** (`265` ficheros, `0` violaciones) |

## Matriz de Criterios de Aceptación

| ID | Estado | Evidencia |
|----|--------|-----------|
| **K-TTY** | **PASS** | PTY real `6.7 ms < 500 ms`, `exitCode: 1`, JSON accionable. |
| **K-FILE** | **PASS** | `--request-file` validado en unitarios `sddia-io` y en registro de evolución. |
| **K-ENV** | **PASS** | `SDDIA_CAPSULE_REQUEST` y `SDDIA_SKIP_STDIN=1` cubiertos en unitarios y release. |
| **K-PIPE** | **PASS** | Pipe/heredoc preservados; compilación `wasm32-wasip1` verde. |
| **K-SDDIA-IO** | **PASS** | `read_stdin_json()` con guarda `is_terminal()` y 6/6 tests en verde. |
| **K-HELP** | **PASS** | `usage()` actualizado en `SddIA/tools/sddia-qa/src/main.rs`. |
| **K-DOC** | **PASS** | `capsule-json-io.md` y `sddia-evolution-register.md` actualizados; `gate-evolution` `EVOL_OK`. |
