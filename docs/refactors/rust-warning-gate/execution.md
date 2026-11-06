---

feature_name: rust-warning-gate
created: "2026-11-06"
process: refactorization
persist_ref: docs/refactors/rust-warning-gate
document_id: PBI-a1b2c3d4
---

# Execution — rust-warning-gate

| Paso | Comando / acción | Resultado |
| --- | --- | --- |
| E1 | `RUSTFLAGS=-Dwarnings cargo build --workspace` (SddIA/) | OK |
| E2 | Release lista `start-sddia.sh` + `-Dwarnings` | OK (~25m cold) |
| E3 | `RUSTFLAGS=-Dwarnings bash SddIA/scripts/qa/build-wasi-capsules.sh` | OK (24 paquetes wasm) |
| E4 | `cargo test -p execute-process --lib --no-run` con config | OK |
| E5 | `cargo test … memory_evolution_ingest --no-run` con config | OK |

Nota: Cargo emite aviso sobre `profiles` en `wasi-poc/Cargo.toml` (no es lint de `rustc`).
