---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
branch: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
pbi_ref: docs/todos/done/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/360
merged_pr: 360
merge_commit: 3f2c27b97cb32cd4dcaa1b0ff2f0211edc1a501d
ci_run_id: "39570890303"
ci_run_url: https://github.com/racso80es/SddIA/actions/runs/39570890303
ci_head_sha: "47c04d515c9bd8d25aac3be09c20b19e9b9e9e9a"
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
checks:
  CA1: APTO
  CA2: APTO
  CA3: APTO
  CA4: APTO
  CA5: APTO
git_changes:
  - docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke/validacion.md
  - docs/todos/done/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
  - docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
---

# Validación — kaizen-ci-chronic-eda-bus-e2e-smoke

CA1: `event-bus-audit` está en `EXCLUDE`. El descubrimiento devuelve 23 paquetes y no incluye ese nombre.

CA2: `build-wasi-capsules.sh` con `clang` 18.1.3 y `CARGO_TARGET_DIR=SddIA/target` terminó `exitCode: 0`, `package_count: 23`. La salida no contiene `could not compile redb`.

CA3: run `pull_request` [39570890303](https://github.com/racso80es/SddIA/actions/runs/39570890303), head `47c04d515c9bd8d25aac3be09c20b19e9b9e9e9a`. Job `eda-bus-e2e-smoke` en success. Merge de ese head: [#360](https://github.com/racso80es/SddIA/pull/360), `3f2c27b97cb32cd4dcaa1b0ff2f0211edc1a501d`.

CA4: en el mismo run, `wasi-runtime-smoke` en success. `eda-iota-smoke-simulate` falla con `cumulo.iota-immutable-publisher` `delivery_status` failed. Ese fallo ya está en `main` antes de este diff: run [39568902126](https://github.com/racso80es/SddIA/actions/runs/39568902126), merge #359, head `37d499f5d40084580586d842bafaa8977df74661`, el mismo mensaje. El job no ejecuta `build-wasi-capsules.sh`.

CA5: este cierre documental mueve el PBI a `docs/todos/done/` y sella `validacion.md`. No toca el pin de `redb`, el toolchain, el fuente de `event-bus-audit` ni el ledger.
