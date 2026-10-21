---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
base: main
scope: exclude-event-bus-audit-wasi
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
pbi_uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
pbi_version: "1.1.0"
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
---

# Spec — kaizen-ci-chronic-eda-bus-e2e-smoke

## Slice único — EXCLUDE

`SddIA/scripts/qa/build-wasi-capsules.sh` descubre `skills/*/`, `tools/*/` e `interfaces/*/` con `src/main.rs` y los compila con:

```text
cargo build --target wasm32-wasip1 -p <paquete>
```

El workflow `sddia-index-qa` usa `dtolnay/rust-toolchain@stable` en el job `eda-bus-e2e-smoke` y en `wasi-runtime-smoke`.

`event-bus-audit` tiene `src/main.rs` y depende de `sddia-event-store` (`redb =2.6.4`). Es el único paquete descubierto con esa dependencia. `execute-process`, `event-watcher` y `kalma2-bridge` ya están excluidos. `sddia-daemon-runtime` no vive bajo esos tres árboles.

`redb` 2.6.4, línea 26:

```rust
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]
```

En `wasm32-wasip1` el predicado es verdadero. rustc stable emite `E0554` y el crate no compila (exit 101). Log de referencia: run `39562132513` (merge #356) y run `39565647758` (PR #358).

## Operación

Añadir `event-bus-audit` al array `EXCLUDE`, con comentario de host nativo junto al de `thought-graph-access`.

No se cambia `SddIA/sddia-event-store/Cargo.toml`, `.github/workflows/sddia-index-qa.yml`, `SddIA/tools/event-bus-audit/` ni `.SddIA/radamanto/ci_failures.json`.

## Gate local

`SddIA/scripts/qa/build-wasi-capsules.sh` exit 0. La salida no contiene `could not compile redb`.

Todo path bajo `SddIA/` fuera de `directories.evolution` es material para el veredicto. El correlato es el alta `4a6499e4-1a65-4aed-bd9f-e1be6b51b196`, emitida por `sddia-qa evolution-register`, con `relacionado` apuntando al script.
