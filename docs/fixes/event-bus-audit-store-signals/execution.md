---
feature_name: event-bus-audit-store-signals
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/event-bus-audit-store-signals
tracker_ref: AUT-27
execution_id: "6cd0a99f-1d58-43ee-8e30-31e9e13a090b"
items_applied:
  - T1
  - T2
  - T3
  - T4
---

# Ejecución — AUT-27

1. **T1–T3** — Señales redb en `SddIA/tools/event-bus-audit/src/main.rs` (`audit_state_from_store`, helpers de catálogo, efecto y residuo).
2. **T4** — Tests `store_unavailable_*`, `redb_audit_marks_success_sin_efecto_*`, `redb_audit_catalog_mismatch_on_presented`, `redb_audit_residual_pending_with_fracture_trace`.
3. `cargo test -p event-bus-audit` → OK (13 tests).

4. **Live redb** — `SDDIA_EVENT_STORE=redb`, auditoría sobre `.SddIA/runtime/events.redb`: sin `store_unavailable`; señales `delivery_fuera_de_catalogo` y `residual_filesystem` activas. Ids históricos del PBI ausentes del store; cubiertos por fixtures. `validacion.md` → `global: APTO`.
