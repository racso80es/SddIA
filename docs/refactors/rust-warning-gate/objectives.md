---

feature_name: rust-warning-gate
created: "2026-11-06"
process: refactorization
branch_name: refactor/rust-warning-gate
persist_ref: docs/refactors/rust-warning-gate
execution_id: "b0e1f2a3-4c5d-4e6f-8091-a2b3c4d5e6f7"
document_id: PBI-a1b2c3d4
---

# Objetivos — rust-warning-gate

## Misión

Errores de compilador (`rustc`) como warnings no pasan la aduana: norma kernel, CI con `RUSTFLAGS=-Dwarnings`, higiene del workspace y paridad local vía `SddIA/.cargo/config.toml`.

## Alcance

1. Norma `SddIA/norms/rust-warning-gate.md`.
2. `env: RUSTFLAGS: "-Dwarnings"` en `.github/workflows/sddia-index-qa.yml`.
3. `[build] rustflags` en `SddIA/.cargo/config.toml`.
4. Corrección de warnings hasta cubrir `cargo build --workspace`, release de ignición, WASI `build-wasi-capsules.sh` y tests de execute-process/Lance.

## Fuera de alcance

- Política Clippy estricta salvo decisión explícita posterior.
- Cambiar semántica de hooks Git.

## Criterios de aceptación

- `RUSTFLAGS=-Dwarnings` compila workspace release/debug y targets de CI sin error de warning.
- Workflow principal fija el flag a nivel job.
- `validacion.md` con `global: APTO`.
