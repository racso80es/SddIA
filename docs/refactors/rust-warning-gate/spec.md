---

feature_name: rust-warning-gate
created: "2026-11-06"
process: refactorization
persist_ref: docs/refactors/rust-warning-gate
document_id: PBI-a1b2c3d4
---

# Spec — rust-warning-gate

## Norma

| Archivo | Contenido |
| --- | --- |
| `SddIA/norms/rust-warning-gate.md` | Ley: cero warnings `rustc` en merges; comandos canónicos; fuera Clippy y fuera `warn:` de proceso |

## CI

| Cambio | Efecto |
| --- | --- |
| `RUSTFLAGS: "-Dwarnings"` en `env` de `sddia-index-qa.yml` | Propaga a `native-build`, `wasi-build`, `sddia-index-integrity` (tests Lance) |

## Local

| Archivo | Contenido |
| --- | --- |
| `SddIA/.cargo/config.toml` | `[build] rustflags = ["-Dwarnings"]` alineado a ignición y desarrollo |

## Higiene

- Eliminación de `dead_code`, imports no usados y `mut` innecesarios.
- Helpers de test solo usados en `cfg` condicional ajustados (WASI: imports bajo `#[cfg(not(target_arch = "wasm32"))]`).
- URI `URI_PATH_MISSING` vía `SandboxError::InvalidArgument` en workspace-server.
