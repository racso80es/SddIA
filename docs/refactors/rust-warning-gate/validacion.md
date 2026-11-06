---
feature_name: rust-warning-gate
created: 2026-11-06
process: refactorization
persist_ref: docs/refactors/rust-warning-gate
document_id: PBI-a1b2c3d4
branch: refactor/rust-warning-gate
global: APTO
pbi_archived: false
validated_base_sha: 9f6360a14e296e1a8805ecc54b97ea93ecd0d1a3
validated_head_sha: feb2c1e0f010c0c2f8da8a97e4125aac82b1caa7
validated_tree_sha: d4a7e1d34b67367fbe4ca946a651bf91fdca8158
diff_digest: sha256:16553e90c70f061d0a6e58deb7b4dd5ce246db0e3c2c3e16742a8233a0cdc99e
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