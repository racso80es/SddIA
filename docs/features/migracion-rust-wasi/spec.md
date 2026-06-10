---
feature_name: migracion-rust-wasi
created: "2026-06-01"
process: feature
branch_name: feat/migracion-rust-wasi-12481127328253895075
persist_ref: docs/features/migracion-rust-wasi
---

# [ESPECIFICACIÓN] Migración de Cápsulas Rust a WASI

## 1. Naturaleza y Propósito
El objetivo de este PBI es transmutar las 12 cápsulas Rust existentes en el ecosistema (tools y skills) para que compilen y operen nativamente bajo el estándar WebAssembly System Interface (WASI). Esta migración materializa la Ley de Aislamiento Físico y asegura la portabilidad absoluta del motor de ejecución.

## 2. Fronteras del Dominio
- **Alcance Físico:** Modificación de las cadenas de construcción (scripts `build` y `run`), archivos `Cargo.toml` y configuraciones locales (`.cargo/config.toml`) para toda la topología bajo `skills/` y `tools/`.
- **Contrato I/O:** Todas las cápsulas deben seguir respetando estrictamente el protocolo `capsule-json-io.md` (comunicación exclusiva por stdin/stdout).
- **Aduana:** La migración debe superar los tests de integridad del proceso y compilar sin fallos fatales en el entorno unificado (`cargo build --workspace`).

## 3. Criterios de Aceptación (S+ Grade)
1. El target de compilación por defecto para todas las cápsulas muta a `wasm32-wasip1`.
2. Los scripts de invocación (`.sh` / `.bat`) levantan las cápsulas utilizando el runtime `wasmtime` en lugar de invocar binarios nativos.
3. El proceso de revisión de PR supera íntegramente los controles de la Aduana Universal (`pull-request-review`).