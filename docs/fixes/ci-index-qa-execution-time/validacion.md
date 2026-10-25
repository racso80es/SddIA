---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
branch: fix/ci-index-qa-execution-time
global: APTO
pbi_archived: false
tracker_ref: AUT-13
pr_url: https://github.com/racso80es/SddIA/pull/386
merged_pr: https://github.com/racso80es/SddIA/pull/386
merge_commit: 15b2945b
ci_run: https://github.com/racso80es/SddIA/actions/runs/40217143680
ci_installer_run: https://github.com/racso80es/SddIA/actions/runs/40217146356
ci_head: 1b6802ac6029031466685cac92ca0c1f689835da
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

`done_gate` linear: issue **AUT-13** en Done; descripción Linear actualizada con laudo instalador (`paths` en PR y push `main`).

## Local

- Labs Linear: `cargo test -p execute-process --lib linear_direct_cycle_e2e` — 4 passed (~11s CPU).
- `gate-evolution --json --range` → `EVOL_OK`.

## CI — `sddia-index-qa`

Run [40217143680](https://github.com/racso80es/SddIA/actions/runs/40217143680), `headSha` `1b6802ac`. Jobs en **success**: `changes`, `native-build`, `wasi-build`, `sddia-index-integrity` (~56s), `eda-iota-smoke-simulate`, `eda-iota-physical`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke` (~1m13s).

En este PR el diff activa `lance_memory` y `linear_e2e` (suite completa de índice); la poda estrecha (AC-5 sin tocar motor/memoria) queda para un PR documental futuro.

## CI — instalador

Run [40217146356](https://github.com/racso80es/SddIA/actions/runs/40217146356): job **`sddia-installer-smoke` success** (~19m23s) tras prep acotado (`1b6802ac`). AC-2 sellado.

`sddia-installer-qa.yml`: mismos `paths` en `pull_request` y `push` a `main` (no smoke en cada push sin tocar superficie instalador).
