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
  - T8-a32-wrapper
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Implementation — kaizen-ci-step-runtime-gt-1min

## Touchpoints

| Path | Cambio |
|------|--------|
| `.github/workflows/sddia-index-qa.yml` | A3.2: job `SCCACHE_GHA_ENABLED=true`; step `RUSTC_WRAPPER` post-action; A3.1 registry/git + restore@v4 |
| `SddIA/engine/execute-process/tests/memory_evolution_ingest.rs` | 3 tests de ingesta |
| `SddIA/engine/execute-process/src/engine/memory_evolution_ingest_core.rs` | Purga `#[cfg(test)]` |
| `SddIA/evolution/530039c9-100b-413a-b3d5-ca632d83acc6.md` | Alta vía `sddia-qa evolution-register` |

## Cache (CA4)

A3.2: wrapper Cableado. `CARGO_INCREMENTAL=0` se sostiene. Stats SHA-1/SHA-2 en `validacion.md`.
