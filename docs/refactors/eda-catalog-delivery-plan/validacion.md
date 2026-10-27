---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
branch: refactor/eda-catalog-delivery-plan
global: APTO
tracker_ref: AUT-17
pr_url: https://github.com/racso80es/SddIA/pull/397
merged_pr: 397
merge_commit: 82afbce8ae9781f5cfa8601082d75578565d5ad3
pbi_archived: false
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/sddia-eda-catalog/
  - SddIA/sddia-event-store/src/gate.rs
  - SddIA/sddia-event-store/tests/gate.rs
  - SddIA/engine/execute-process/
  - SddIA/sddia-daemon-runtime/
  - SddIA/daemons/event-watcher/
  - SddIA/Cargo.toml
  - docs/refactors/eda-catalog-delivery-plan/
---

# Validación — AUT-17

AC-1: un `PBI_Refined` sin claves queda con `tekton.tracker-stamp: pending` si el catálogo lo declara. AC-2: dominio sin plan devuelve `EDA_DELIVERY_PLAN_MISSING`; el valor `success` no entra en `list_pending_deliveries`. AC-3: CI del PR #397 en verde (workflow `sddia-index-qa` run [40368366411](https://github.com/racso80es/SddIA/actions/runs/40368366411), merge `82afbce8`).

`global: APTO` con `done_gate: linear`; cierre de issue AUT-17 en estado Done en Linear (sin fichero en `docs/todos/done/`).
