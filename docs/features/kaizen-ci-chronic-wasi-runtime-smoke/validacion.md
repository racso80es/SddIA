---
feature_name: kaizen-ci-chronic-wasi-runtime-smoke
created: "2026-09-25"
process: feature
branch: feat/kaizen-ci-chronic-wasi-runtime-smoke
branch_name: feat/kaizen-ci-chronic-wasi-runtime-smoke
persist_ref: docs/features/kaizen-ci-chronic-wasi-runtime-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-WASI-RUNTIME-SMOKE
uuid: "30a026cf-8227-4315-a521-53ec34a8cc0a"
global: PENDIENTE-CI
pbi_archived: false
checks:
  CA1: APTO
  CA2: APTO
  CA3: PENDIENTE-CI
  CA4: PENDIENTE-CI
  CA5: APTO
git_changes:
  - SddIA/evolution/83d6eb73-0936-4acb-9f1e-5d519987f1ab.md
  - docs/features/kaizen-ci-chronic-wasi-runtime-smoke/
  - docs/todos/pending/[KAIZEN] CI crónica — wasi-runtime-smoke.md
---

# Validación — kaizen-ci-chronic-wasi-runtime-smoke

CA1: `hash_integrity` es `sha256:7a76a140e8f111bed85904c5fcbf38f6eb88ad260db3d866b35edcf4b7f06dd8`. El literal `pending-pr-292` no está en el fichero.

CA2: `gate-evolution --json --all` sobre `1992513` → `exitCode: 0`, `universo conforme`. Rango `--sync-base --require-synced-base` → `L-SELF / sin material`.

CA3 y CA4: sin run `pull_request` todavía. `global` no es `APTO`.

CA5: el único path bajo `SddIA/` en el diff de código es el registro evolution. Sin cambios en `build-wasi-capsules.sh` ni en el ledger.
