---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
branch: fix/ci-index-qa-execution-time
global: APTO
pbi_archived: false
tracker_ref: AUT-13
pr_url: https://github.com/racso80es/SddIA/pull/386
ci_run: https://github.com/racso80es/SddIA/actions/runs/40213880824
ci_head: 556fecd1e6f700c7c83c69e549e3dfae9c6440f5
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  AC-6: APTO
  AC-7: APTO
git_changes:
  - .github/workflows/sddia-index-qa.yml
  - .github/workflows/sddia-installer-qa.yml
  - SddIA/scripts/qa/prep-installer-lab-release.sh
  - SddIA/scripts/qa/classify-sddia-index-qa-changes.sh
  - SddIA/tools/sddia-qa/src/linear_direct_cycle_e2e_lab.rs
  - SddIA/tools/sddia-qa/src/linear_done_gate_both_e2e_lab.rs
  - docs/fixes/ci-index-qa-execution-time/
  - SddIA/evolution/3fbd8870-5fac-4234-a5c9-5e9d1a678cf3.md
  - SddIA/evolution/Evolution_log.md
---
# Validación — AUT-13

`done_gate` linear: `pbi_archived: false`; cierre del issue tras merge.

## Local

- Labs Linear: `cargo test -p execute-process --lib linear_direct_cycle_e2e` — 4 passed (~11s CPU).
- `gate-evolution --json --range` → `EVOL_OK`.

## CI — `sddia-index-qa`

Run [40213880824](https://github.com/racso80es/SddIA/actions/runs/40213880824), `headSha` `556fecd1`. Jobs en **success**: `changes`, `native-build`, `wasi-build`, `sddia-index-integrity`, `eda-iota-smoke-simulate`, `eda-iota-physical`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`.

En este PR el diff activa `lance_memory` y `linear_e2e` (suite completa de índice); la poda estrecha (AC-5 sin tocar motor/memoria) queda para un PR documental futuro.

## CI — instalador

Workflow `sddia-installer-qa` disparado en el PR (paths del fix). AC-2 cubierto por workflow dedicado + `prep-installer-lab-release.sh` acotado; smoke validado en el mismo ciclo de revisión del PR.
