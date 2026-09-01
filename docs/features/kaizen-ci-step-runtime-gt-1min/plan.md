---
feature_name: kaizen-ci-step-runtime-gt-1min
created: "2026-09-01"
process: feature
phase: planning
agents: dedalo
phases:
  - T1-cache
  - T2-workspace-gate
  - T3-ingest-itest
  - T4-evolution
  - T5-tekton-docs
  - T6-argos-archive
  - T7-delivery-close
  - T8-a32-wrapper
branch_name: feat/kaizen-ci-step-runtime-gt-1min
persist_ref: docs/features/kaizen-ci-step-runtime-gt-1min
pbi_ref: docs/todos/pending/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
document_id: PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN
uuid: "530039c9-100b-413a-b3d5-ca632d83acc6"
runtime_execution_id: "a13e2476-8474-49ef-ab2f-0d1fe915a21f"
---

# Plan — kaizen-ci-step-runtime-gt-1min

Blueprint Tekton. Contratos: `spec.md` v1.2.0. Este commit sella Diseño A3.2. Ejecución T8 en la misma sesión tras el commit. T1–T7 ya ejecutados (PR #246).

Init lab: `execution_id` `a13e2476-8474-49ef-ab2f-0d1fe915a21f` · relevo IDE.

## T1 — Cache (CA4 / L-CACHE-INTEGRITY / L-CACHE-IOTA-RO)

`.github/workflows/sddia-index-qa.yml`: integrity §4.1; `eda-iota-smoke-simulate` y `eda-iota-physical` §4.2. No tocar keys `wasi-*`.

## T2 — Workspace + gate (CA1 / L-ONE-WORKSPACE / L-GATE-IO)

Sustituir `Build QA aduana` por `Build native workspace`. Step `verify-compiled-capsules` sin `cargo build --workspace`.

## T3 — Ingest integración (CA2 / L-INGEST-ITEST / L-TEST-CMD)

`SddIA/engine/execute-process/tests/memory_evolution_ingest.rs`. Purgar `#[cfg(test)]` de `memory_evolution_ingest_core.rs`. YAML LanceDB = L-TEST-CMD.

```text
cd SddIA && cargo test -p execute-process --test memory_evolution_ingest
cd SddIA && cargo test -p sddia-core-memory -p sddia-infrastructure-lancedb-thought -p sddia-infrastructure-lancedb-evolution
```

## T4 — Evolution

`sddia-qa evolution-register` id `530039c9-100b-413a-b3d5-ca632d83acc6`. `relacionado`: PBI + YAML + tests + persist_ref. `gate-evolution --json --range` antes de DCC.

## T5 — Documental Tekton

`implementation.md` + `execution.md`.

## T6 — Argos + archive

`validacion.md`: checks CA1–CA7; CA de CI con `run_id` o `PENDIENTE-CI`. PBI → `docs/todos/done/` mismo `document_id`. `pbi_archived: true` solo tras el move.

## T7 — DCC

`delivery-close-cycle` · `source_process: feature` · `persist_ref` · `branch_name`. Tests locales T3 verdes **antes** de DCC. Prohibido polling CI (DA-6).

## T8 — A3.2 wrapper (CA4 / L-SCCACHE)

YAML jobs integrity + IOTA:

- `env` de job: `CARGO_INCREMENTAL=0`, `SCCACHE_GHA_ENABLED=true`
- Tras `sccache-action`: step `sccache rustc wrapper` → `RUSTC_WRAPPER` en `GITHUB_ENV`
- WASI intacto
- `validacion.md`: `PENDIENTE-CI` (SHA-1 calentamiento). CA1/CA5 no APTO en este SHA.

## Orden

```text
T1 → T2 → T3 → T4 → T5 → T6 → T7   # hecho
T8 → T7 (DCC SHA-1)
```

## Delegaciones

| Fase | Vía |
|------|-----|
| YAML CI | Tekton directo (`.github/` ∉ DA-2) |
| tests `engine/` | Tekton directo |
| Evolution | `sddia-qa evolution-register` |
| Git snapshot | `skill:git-manager` / commit pedido por Racso |
| PR | `delivery-close-cycle` |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Primer PR cache-miss `native-integrity-*` | restore-keys legado; CA1 techo / PENDIENTE-CI |
| Integración no ve el crate adapter | reexport o `use` del dep; no ciclo |
| `lookup-only` no restaura archivos | A3.1: `actions/cache/restore@v4` |
| Wrapper ausente (A3.1: 0 requests) | T8: `RUSTC_WRAPPER` post-action + `SCCACHE_GHA_ENABLED` en job |
| CA5 < 8 min en frío SHA-1 | SHA-1 calentamiento; medición SHA-2 |
