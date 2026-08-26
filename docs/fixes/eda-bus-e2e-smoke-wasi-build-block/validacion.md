---
feature_name: eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
process: bug-fix
branch: fix/eda-bus-e2e-smoke-wasi-build-block
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] eda-bus-e2e-smoke.md
checks:
  CA-1-eda-bus-e2e-smoke: pass
  CA-2-wasi-runtime-smoke: pass-ci-pending
  CA-3-no-email-watcher-wasi: pass
  CA-4-native-jobs: pass
  CA-5-evolution-gate: pass
evolution_id: 6b600e3f-2f4a-4f1f-b0f9-087d2f954c1f
pr_url: https://github.com/racso80es/SddIA/pull/195
git_changes:
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - .github/workflows/sddia-index-qa.yml
  - SddIA/evolution/6b600e3f-2f4a-4f1f-b0f9-087d2f954c1f.md
  - SddIA/evolution/Evolution_log.md
  - docs/fixes/eda-bus-e2e-smoke-wasi-build-block/
  - docs/todos/done/[FIX] eda-bus-e2e-smoke.md
---

# Validación

**Veredicto global: APTO**

## CA-1 — `eda-bus-e2e-smoke`

CI PR #195: **SUCCESS**. `run-eda-e2e-lab` + `event-sweeper --once` ejecutados.

## CA-2 — `wasi-runtime-smoke` (fix unificado)

Build WASI + smoke OK; fallo inicial en `gate-evolution` por `EVOL_MATERIAL_UNREGISTERED` en `build-wasi-capsules.sh` — resuelto con alta evolution `6b600e3f-2f4a-4f1f-b0f9-087d2f954c1f`.

## CA-3 — Grafo WASI sin daemons nativos

`build-wasi-capsules.sh` excluye `email-watcher` y centinelas sensoriales; no invoca `cargo build --workspace`.

## CA-4 — Regresión

`verify-tools-index`, `eda-iota-smoke-simulate`, `eda-iota-physical`: SUCCESS en PR #195.

## CA-5 — Evolution gate

Alta `6b600e3f-2f4a-4f1f-b0f9-087d2f954c1f` cubre material SddIA del diff.
