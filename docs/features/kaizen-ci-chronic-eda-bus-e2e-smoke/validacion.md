---
feature_name: kaizen-ci-chronic-eda-bus-e2e-smoke
created: "2026-10-21"
process: feature
branch: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
branch_name: feat/kaizen-ci-chronic-eda-bus-e2e-smoke
persist_ref: docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke
pbi_ref: docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
document_id: PBI-KAIZEN-CI-CHRONIC-EDA-BUS-E2E-SMOKE
uuid: "f2bbed15-2cca-49ae-a439-aff1a935e43b"
global: PENDIENTE-CI
pbi_archived: false
execution_id: "0db56ca9-0fe4-4e81-96cd-88f5c52fcdd4"
checks:
  CA1: APTO
  CA2: APTO
  CA3: PENDIENTE-CI
  CA4: PENDIENTE-CI
  CA5: APTO
git_changes:
  - SddIA/scripts/qa/build-wasi-capsules.sh
  - docs/features/kaizen-ci-chronic-eda-bus-e2e-smoke/
  - docs/todos/pending/[KAIZEN] CI crónica — eda-bus-e2e-smoke.md
---

# Validación — kaizen-ci-chronic-eda-bus-e2e-smoke

CA1: `event-bus-audit` está en `EXCLUDE`. El descubrimiento devuelve 23 paquetes y no incluye ese nombre.

CA2: `build-wasi-capsules.sh` con `clang` 18.1.3 y `CARGO_TARGET_DIR=SddIA/target` terminó `exitCode: 0`, `package_count: 23`. La salida no contiene `could not compile redb`.

CA3: sin run `pull_request` verde de `eda-bus-e2e-smoke`. `global` no es `APTO`.

CA4: el mismo run aún no existe. `wasi-runtime-smoke` queda pendiente en ese run.

CA5: el working tree antes del cierre son el script, esta cascada y el PBI en `pending/`. Pin de `redb`, toolchain, fuente de `event-bus-audit` y ledger no entran en el diff.
