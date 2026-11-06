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
validated_head_sha: 0a0b91df73ef1af9b5e7d61de7317243a9dfd27f
validated_tree_sha: 296573cc2059004019bf301b9f318612814ca285
diff_digest: sha256:73992f2a40e8f81bed1febdb60a194a29c6ce0b219f1b46abbc50a7f8b3fffdb
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