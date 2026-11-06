---

feature_name: rust-warning-gate
created: "2026-11-06"
process: refactorization
persist_ref: docs/refactors/rust-warning-gate
document_id: PBI-a1b2c3d4
branch: refactor/rust-warning-gate
global: APTO
pbi_archived: false
---

# Validación — rust-warning-gate

## Checks

| ID | Criterio | Estado |
| --- | --- | --- |
| C1 | `cargo build --workspace` con `RUSTFLAGS=-Dwarnings` | PASS |
| C2 | Release ignición (paquetes `start-sddia.sh`) con `-Dwarnings` | PASS |
| C3 | `build-wasi-capsules.sh` con `-Dwarnings` | PASS |
| C4 | Tests `execute-process` (lib + `memory_evolution_ingest`) compilan con `-Dwarnings` | PASS |
| C5 | Norma `rust-warning-gate.md` + CI + `.cargo/config.toml` | PASS |

## Git / tracker

- `tracker.provider: linear` — cierre en issue Linear (Done), sin ficheros nuevos en `docs/todos/pending` ni `done/`.
