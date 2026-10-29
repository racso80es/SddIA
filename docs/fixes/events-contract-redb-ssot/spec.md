---
feature_name: events-contract-redb-ssot
created: "2026-10-29"
process: bug-fix
branch_name: fix/events-contract-redb-ssot
pbi_ref: AUT-45
execution_id: "a629489e-2c6c-41fb-89db-26484a96fbd9"
---

# Especificación — AUT-45

## Cambios

1. `SddIA/events/events-contract.md`: §2 tabla instancia ECST; §4 ciclo con modos `filesystem` | `dual` | `redb` (default), SSOT `subscriber_delivery`.
2. `sddia-event-store`: `delivery_terminal_in_store`.
3. `eda_bus_topology::terminal_witness_exists`: consulta store antes que FS en modos con store.

## Regresión

Test `terminal_witness_exists_redb_without_fs_witness` en `event_store_writers.rs`.
