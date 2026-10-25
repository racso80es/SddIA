---
feature_name: ci-index-qa-execution-time
created: "2026-10-25"
process: bug-fix
branch: fix/ci-index-qa-execution-time
global: PENDIENTE-CI
pbi_archived: false
tracker_ref: AUT-13
pr_url: https://github.com/racso80es/SddIA/pull/386
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: PENDIENTE-CI
  AC-5: PENDIENTE-CI
  AC-6: PENDIENTE-CI
  AC-7: PENDIENTE-CI
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

## Local

- Labs Linear invocan `cargo test -p execute-process --lib` (AC-1).
- `prep-installer-lab-release.sh` limita release a `execute-process` (AC-2).
- Clasificador bash expone tres flags; en `push` `main` las fuerza a `true` (AC-6 parcial).

## CI

Pendiente run verde de `sddia-index-qa` y `sddia-installer-qa` en el PR de esta rama.
