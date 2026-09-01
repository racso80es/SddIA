---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
items:
  - T1-cache
  - T2-workspace-gate
  - T3-ingest-itest
  - T4-evolution
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Implementation — kaizen-ci-step-runtime-gt-1min

## Touchpoints

| Path | Cambio |
|------|--------|
| `.github/workflows/sddia-index-qa.yml` | Key `native-integrity-*`; IOTA `lookup-only`; un `cargo build --workspace`; gate I/O; LanceDB multi-`-p` + `--test memory_evolution_ingest` |
| `SddIA/engine/execute-process/tests/memory_evolution_ingest.rs` | 3 tests de ingesta (nombres estables) |
| `SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs` | Purga `#[cfg(test)]` |
| `SddIA/evolution/530039c9-100b-413a-b3d5-ca632d83acc6.md` | Alta vía `sddia-qa evolution-register` |

## Cache (CA4 — decisión)

Save de `SddIA/target` **solo** en `sddia-index-integrity` (`native-integrity-${os}-${lock}`). Jobs IOTA restauran la misma key con `lookup-only: true` (no sellan `target/` parcial). `restore-keys` incluye legado `native-${os}-`. Jobs WASI sin cambio.

Primer PR: posible miss exacto + restore parcial legado. Números de run en `validacion.md`.

## CA1 anti-maquillaje

Presupuesto verify = `Build native workspace` + `verify-compiled-capsules` (I/O). LanceDB = un `cargo test` de 3 crates + `--test memory_evolution_ingest` (sin `cfg(test)` de 366 tests del orquestador).
