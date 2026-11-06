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
validated_head_sha: c46019e6099fe78fcd9a6fb98ab93be6437fbb97
validated_tree_sha: 87a886e9e94cea44dee7aa64c5fd365d61ab3f0a
diff_digest: sha256:01700f8bacbddc8cc668cd72f0cc4a65db9d79d64075f5ed0aecf5e111545d68
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