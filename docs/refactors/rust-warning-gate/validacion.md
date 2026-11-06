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
validated_head_sha: e8e4aff29f436a47091118305428f3cece9684cb
validated_tree_sha: 82f00b0699e463d711be6dc2dbde3399f6480f10
diff_digest: sha256:9a9a72b4edc45cdd9a1e65b69d748c8c5f19580b17e26eb6782e8a1cc4ae9c9d
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