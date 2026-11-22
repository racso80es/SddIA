---
feature_name: kaizen-sddia-qa-stdin-tty-guard
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Plan ejecutable — guarda Fail-Fast TTY y paridad capsule-json-io v2.0 (AUT-99)
branch_name: feat/kaizen-sddia-qa-stdin-tty-guard
persist_ref: docs/features/kaizen-sddia-qa-stdin-tty-guard
pbi_ref: AUT-99
document_id: PBI-KAIZEN-SDDIA-QA-EVOLUTION-REGISTER-TTY-HANG
agent: dedalo
phase: diseno-blueprint
version_plan: "1.0.0"
status: blueprint-ready
norm_ref: features-documentation-pattern@1.3.1
execution_id: "9d9d5478-f1d0-4396-a0a8-d2d0d47f71ff"
phases:
  - id: Ola 0
    name: Baseline y Oráculo PTY/Unit en Rojo
  - id: Ola 1
    name: Resolver sddia-io y tests unitarios (K-FILE, K-ENV, K-SDDIA-IO)
  - id: Ola 2
    name: sddia-qa y sddia-evolution-register + WASI (K-TTY, K-PIPE, K-HELP)
  - id: Ola 3
    name: Norma capsule-json-io.md y skill sddia-evolution-register.md (K-DOC)
  - id: Ola 4
    name: Verificación PTY real, workspace, WASI y evolution
---
# Plan de ejecución — kaizen-sddia-qa-stdin-tty-guard

## Ola 0 — Baseline y Oráculo PTY/Unit en Rojo
1. Verificar árbol limpio en rama `feat/kaizen-sddia-qa-stdin-tty-guard` y baseline de `cargo test -p sddia-io -p sddia-qa -p sddia-evolution-register`.

## Ola 1 — Resolver `sddia-io` (`K-FILE`, `K-ENV`, `K-SDDIA-IO`)
1. Implementar `resolve_capsule_json_input` y `read_capsule_json_from_args` en `SddIA/sddia-io/src/lib.rs`, actualizando `read_stdin_json()` y añadiendo tests unitarios.

## Ola 2 — `sddia-qa` y `sddia-evolution-register` (`K-TTY`, `K-PIPE`, `K-HELP`)
1. Actualizar `run_mutate` en `SddIA/tools/sddia-qa/src/gate_evolution.rs` y `usage()` en `SddIA/tools/sddia-qa/src/main.rs`.
2. Actualizar `SddIA/skills/sddia-evolution-register/src/main.rs`.
3. Añadir test en `gate_evolution.rs` verificando `--request-file` y error TTY.

## Ola 3 — Norma y Skill (`K-DOC`)
1. Actualizar `SddIA/norms/capsule-json-io.md` (regla Fail-Fast TTY).
2. Actualizar `SddIA/skills/sddia-evolution-register.md` vía `entity-manager` (ejemplo mínimo de invocación).

## Ola 4 — Verificación PTY real, workspace, WASI y evolution
1. Verificar en PTY real (`python3 -c 'import pty...'` con `sddia-qa evolution-register --json`) que termina en < 500 ms con `exitCode: 1`.
2. Verificar `--request-file`, `SDDIA_CAPSULE_REQUEST`, `SDDIA_SKIP_STDIN=1`, `cargo test -p sddia-io -p sddia-qa -p sddia-evolution-register`, `RUSTFLAGS="-Dwarnings" cargo check --workspace`, compilación WASI (`wasm32-wasip1`) y `cargo build --release -p sddia-qa`.
3. Registrar evolución (`evolution-register`) y consolidar `implementation.md` y `execution.md`.
