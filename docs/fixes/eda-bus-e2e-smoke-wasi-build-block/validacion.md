---
feature_name: eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
process: bug-fix
branch: fix/eda-bus-e2e-smoke-wasi-build-block
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[FIX] eda-bus-e2e-smoke.md
checks:
  CA-1-eda-bus-e2e-smoke: pass-local
  CA-2-wasi-runtime-smoke: pass-local
  CA-3-no-email-watcher-wasi: pass
  CA-4-native-jobs: pending-ci
git_changes:
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - .github/workflows/sddia-index-qa.yml
  - docs/fixes/eda-bus-e2e-smoke-wasi-build-block/
  - docs/todos/done/[FIX] eda-bus-e2e-smoke.md
---

# Validación

**Veredicto global: APTO** (smokes locales; CI GHA pendiente de PR)

## CA-1 — `eda-bus-e2e-smoke`

`run-eda-e2e-lab --entity-class tool --json` → `success: true`, `cleaned: true`.

## CA-2 — `wasi-runtime-smoke` (fix unificado)

`run-wasi-ci-smoke --json` con `SDDIA_CI_REQUIRE_WASI=1` → `success: true`, `wasi_path_verified: true`.

## CA-3 — Grafo WASI sin daemons nativos

`build-wasi-capsules.sh` excluye `email-watcher` y centinelas sensoriales; no invoca `cargo build --workspace`.

## CA-4 — Regresión

Jobs nativos sin cambio en workflow. Verificación CI en PR.

## Nota

Fix compartido con `PBI-FIX-WASI-RUNTIME-SMOKE` (`docs/todos/pending/[Fix] wasi-runtime-somke.md`).
