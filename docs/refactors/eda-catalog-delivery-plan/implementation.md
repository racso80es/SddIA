---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
items:
  - sddia-eda-catalog
  - record_ecst
  - escritores de dominio
  - event-watcher reconcile
---

# Implementación

El plan vive en `SddIA/sddia-eda-catalog`. `execute-process` (`plan_route`, `subscriber_id`) delega ahí. Sellan antes de `record_ecst`: `write_pending_domain_event_file`, `route_domain_event`, `actions`, `domain_mutation`, los `write_fractal_event`, `linear_bridge`, `github_bridge` y la fractura residual del watcher.
