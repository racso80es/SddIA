---
uuid: "a7b8c9d0-e1f2-4a3b-9c4d-5e6f7a8b9c0d"
name: "rust-warning-gate"
version: "1.0.0"
entity_type: "norm"
jurisdiction: "dedalo"
hash_signature: "sha256:6d3424528436c89a78d87b553b990e6058bd9ce88b19ee53c5c9ee43b0d1d489"
---

# Gate de warnings Rust (workspace SddIA)

## 1. Ley

Un cambio en el **workspace Rust** bajo `SddIA/` (crates listados en el workspace root `SddIA/Cargo.toml`) **no** se considera válido para merge ni para `validacion.md: global: APTO` si el compilador Rust (`rustc`) emite **warnings** en los perfiles y comandos que la aduana CI ejecuta (ver §2).

Queda **fuera de alcance**: avisos exclusivos de **Clippy** salvo que una norma o el propio proceso de la tarea los incorporen explícitamente.

Los prefijos `warn:` / fail-soft de **envelopes de proceso** (tracker, done-gate, etc.) **no** son warnings de compilador y no aplican a esta norma.

## 2. Comandos canónicos (aduana)

Toda PR hacia `main` debe mantener **cero warnings** de `rustc` cuando:

1. `RUSTFLAGS=-Dwarnings cargo build --workspace` (desde `SddIA/`).
2. Los pasos de compilación del workflow `.github/workflows/sddia-index-qa.yml` que invocan `cargo build` / `cargo test` (el `env` del workflow fija `RUSTFLAGS=-Dwarnings`).
3. Script `SddIA/scripts/qa/build-wasi-capsules.sh` (target `wasm32-wasip1`).

La **ignición local** (`start-sddia.sh`) usa los mismos flags vía `SddIA/.cargo/config.toml` (`[build] rustflags`).

## 3. Restricciones duras

- Prohibido `#![allow(warnings)]` a nivel de crate para ocultar deuda global.
- Un `#[allow(...)]` puntual debe ir acompañado de comentario que cite **issue Linear** (`AUT-xx`) o PBI documentado.
- Prohibido declarar `validacion.md` con `global: APTO` si el árbol relevante no compila sin warnings bajo §2.

## 4. Dependencias

- `SddIA/norms/external-ai-constraints.md` (vía orquestador / CI).
- `SddIA/core/cumulo.paths.json` → `directories.norms` (ubicación de esta norma).
