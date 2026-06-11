---
feature_name: wasi-poc-ignition
created: "2026-06-01"
process: feature
branch_name: feat/wasi-poc-ignition-9366362475876501103
persist_ref: docs/features/wasi-poc-ignition
agent_planificador: tekton
---

# Plan — WASI PoC Ignition

## 0. Estado de la entrega

| Bloque | Estado |
|--------|--------|
| Cápsula Rust envelope v2.0 | ✅ |
| Target `wasm32-wasip1` | ✅ |
| Scripts build/run | ✅ |
| Documentación feature | ✅ |
| Smoke Wasmtime | ⏳ (entorno local) |

## 1. Hito 1 — Cristalización

- [x] Alinear `main.rs` a `capsule-json-io.md` v2.0.
- [x] Fijar `edition = "2021"` y perfil release optimizado.
- [x] Configurar `.cargo/config.toml` con target WASI.

## 2. Hito 2 — Puente físico

- [x] `scripts/build-wasi.sh` — `cargo build --target wasm32-wasip1`.
- [x] `scripts/run-wasi.sh` — `wasmtime run` sin montajes de FS.

## 3. Hito 3 — Validación S+

- [ ] `build-wasi.sh` exit 0 con artefacto `.wasm`.
- [ ] `run-wasi.sh` devuelve JSON con `success: true` y `wasi_status: S+ Grade_Sealed`.
- [ ] Aduana `pull-request-review` sin bloqueo documental.

## 4. Comandos de verificación

```bash
chmod +x SddIA/tools/wasi-poc/scripts/*.sh
SddIA/tools/wasi-poc/scripts/build-wasi.sh
SddIA/tools/wasi-poc/scripts/run-wasi.sh
```
