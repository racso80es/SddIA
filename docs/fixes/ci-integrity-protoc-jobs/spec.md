---
feature_name: ci-integrity-protoc-jobs
created: "2026-10-25"
process: bug-fix
base: main
scope: artefacto native-debug-bins y protoc en eda-bus-e2e-smoke
tracker_ref: AUT-4
execution_id: "1d05f105-00ba-4ecd-b2ac-f295ff6ca047"
---
# Spec — cápsulas del artefacto y protoc en el job E2E

## Hecho

Run [40163380636](https://github.com/racso80es/SddIA/actions/runs/40163380636), head `9027675a`. `verify-compiled-capsules` reportó 34 binarios ausentes. `native-debug-bins` solo publicaba `sddia-qa`, `event-watcher`, `execute-process` e `iota-immutable-publisher`. La caché `native-debug` no restituye el ejecutable enlazado: el hit exacto en integridad dejó los 34 ausentes y la clave de GitHub no se sobrescribe.

`run-eda-e2e-lab` salió con `exitCode` 0. `run-linear-direct-cycle-e2e-lab` lanzó `cargo test -p execute-process linear_direct_cycle_e2e` y `lance-encoding` v10.0.0 abortó: `Could not find protoc`. El job `eda-bus-e2e-smoke` no instalaba `protobuf-compiler`. El runner ya tenía `cargo`: la traza llega al build script.

## Corte

1. Tras `cargo build --workspace` en `native-build`, `stage-native-debug-bins.sh` copia a `SddIA/target/native-debug-bins/` cada binario debug cuyo crate tenga `src/main.rs` en un nivel bajo `engine`, `skills`, `tools`, `daemons` o `interfaces`. El nombre es el `name` de `[package]`, el mismo que usa `verify-compiled-capsules`. El artefacto `native-debug-bins` publica ese directorio. Si falta un binario, el job de build falla.
2. `eda-bus-e2e-smoke` instala `protobuf-compiler` y `libprotobuf-dev` antes del laboratorio que recompila `lance-encoding`.

## Fuera de corte

El prebuild del installer. No se añade caché Rust al job E2E. No se recompila el workspace dentro de `sddia-index-integrity`.

## Criterios

| ID | Criterio |
|----|----------|
| AC-1 | `verify-compiled-capsules` encuentra los binarios del workspace, incluidos los 34 del run 40163380636. |
| AC-2 | `run-linear-direct-cycle-e2e-lab` no aborta por `protoc` ausente. |
| AC-3 | `native-build`, `wasi-build`, `sddia-installer-smoke`, `eda-iota-smoke-simulate`, `eda-iota-physical` y `wasi-runtime-smoke` siguen en verde. |
