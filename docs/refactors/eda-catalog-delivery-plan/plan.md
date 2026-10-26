---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
phases:
  - name: Catálogo y store
    status: forjada
  - name: Emisores y reconciliación
    status: forjada
---

# Plan

1. Crate `sddia-eda-catalog` con `subscriber_id`, `plan_route` y `stamp_published_event`. El motor delega en él.
2. `record_ecst` exige plan en dominio y respeta el estado de cada clave.
3. Los escritores de dominio sellan antes de persistir. El watcher reconcilia lo ya guardado.
