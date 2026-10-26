---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
branch: refactor/eda-catalog-delivery-plan
global: NO_APTO
tracker_ref: AUT-17
pr_url: https://github.com/racso80es/SddIA/pull/397
pbi_archived: false
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: PENDIENTE-CI
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

AC-1: un `PBI_Refined` sin claves queda con `tekton.tracker-stamp: pending` si el catálogo lo declara. AC-2: dominio sin plan devuelve `EDA_DELIVERY_PLAN_MISSING`; el valor `success` no entra en `list_pending_deliveries`. AC-3 espera el check de CI del PR. `global` queda `NO_APTO` hasta ese run.
