---


feature_name: linear-hu-a-04-events
process: feature
execution_id: "f3cb7359-475c-401b-9dd4-4c05ad7710e9"
---
# Implementación

- Forja `pbi-refined`, `hu-refined`, `pbi-cancelled` vía `entity-manager` → `event-creator`.
- `Delivery_Committed` 1.1.0 en `SddIA/events/domain/` (optional `tracker_ref`, `pbi_ref`, `persist_ref`, `commit_sha`); retirado duplicado en códice.
- Motor: `resolve_event_contract` en `run_event_forge`; bootstrap de sello EDA en updates sin `hash_signature` previo.
- Tests: `linear_direct_cycle_events_ecst` en `ecst_validation.rs`.
