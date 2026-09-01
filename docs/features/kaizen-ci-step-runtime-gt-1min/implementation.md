---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
items:
  - T1-cache
  - T2-workspace-gate
  - T3-ingest-itest
  - T4-evolution
  - T1b-a31-sccache
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Implementation — kaizen-ci-step-runtime-gt-1min

## Touchpoints

| Path | Cambio |
|------|--------|
| `.github/workflows/sddia-index-qa.yml` | A3.1: sccache + `restore@v4`; no `SddIA/target` en cache; key + rustc hash |
| `SddIA/engine/execute-process/tests/memory_evolution_ingest.rs` | 3 tests de ingesta (nombres estables) |
| `SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs` | Purga `#[cfg(test)]` |
| `SddIA/evolution/530039c9-100b-413a-b3d5-ca632d83acc6.md` | Alta vía `sddia-qa evolution-register` |

## Cache (CA4)

A3.0 (NO_APTO): save de `target/` en integrity no calienta Cargo (hit `754c575`: restore 43 s, Build 429 s). `lookup-only` no descarga (IOTA 8 min).

A3.1: cache solo registry/git; compile vía sccache GHA; IOTA `actions/cache/restore@v4`; `CARGO_INCREMENTAL=0`; key incluye hash de `rustc -vV`. WASI sin cambio.

## CA1 anti-maquillaje

Presupuesto verify = `Build native workspace` + `verify-compiled-capsules` (I/O). LanceDB = L-TEST-CMD.
