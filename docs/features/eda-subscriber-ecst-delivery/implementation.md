---
feature_name: eda-subscriber-ecst-delivery
pbi_ref: AUT-46
---

# Implementación

- `SddIA/engine/execute-process/src/engine/ecst_input_loader.rs`
- Handlers email/preferencias, `memory_evolution_ingest_core`, `tracker_markdown_apply`, `tracker_stamp`
- `route_domain_core.rs`: `delivery_status`, `process_dispatch_terminal`
- `eda_bus_topology.rs`: `promote_witness` → `result_status` / `skip_reason`
- Tests: `tests/eda_subscriber_ecst_redb.rs`, ampliación `event_store_writers.rs`
