---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
items_applied:
  - T1-cache
  - T2-workspace-gate
  - T3-ingest-itest
  - T4-evolution
  - T1b-a31-sccache
  - T8-a32-wrapper
---

# Ejecución — kaizen-ci-step-runtime-gt-1min

## Init

`execution_id`: `a13e2476-8474-49ef-ab2f-0d1fe915a21f`. Relé IDE. Commit Diseño: `534a9f8`.

## T1–T2 YAML

`.github/workflows/sddia-index-qa.yml`: `native-integrity-*` + rustc hash; registry/git; sccache; IOTA `cache/restore@v4`; `Build native workspace`; gate I/O; LanceDB L-TEST-CMD.

## T1b — A3.1 (iteracion)

`lookup-only` sustituido. `SddIA/target` fuera del blob. `mozilla-actions/sccache-action@v0.0.9`. `permissions.actions: write`.

## T8 — A3.2 wrapper

Job env `SCCACHE_GHA_ENABLED=true`. Step `sccache rustc wrapper` → `RUSTC_WRAPPER` en `GITHUB_ENV` tras el action. WASI intacto. Commit Diseño A3.2: `9eacf10`.

## T3 tests locales

```text
PROTOC=.tmp/protoc/bin/protoc
cargo test -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution
# 3+5+5 passed
cargo test -p execute-process --test memory_evolution_ingest
# 3 passed (0.06 s runtime; compile lib 6m43s en frío local)
```

## T4 evolution

`sddia-qa evolution-register` → `530039c9-100b-413a-b3d5-ca632d83acc6` (`EVOL_OK`).
