---

feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
execution_id: "d707b171-c91f-4933-a68b-0415c19a390d"
---
# Implementación — eda-redb-03-watcher-vitality

## Piezas

- `SddIA/sddia-event-store/src/lib.rs` — `list_pending_deliveries`.
- `SddIA/daemons/event-watcher/` — tick de sondas y reclamo por id. Dependencia de path al store.
- `SddIA/engine/execute-process/src/engine/route_domain_core.rs` — carga del ECST y `AGENT_PHASE`.
- `SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs` — workspace revocable.
- `SddIA/engine/execute-process/src/forges/common.rs` y `factory.rs` — parche de declaración del daemon.
- `SddIA/engine/execute-process/src/engine/entity_manager.rs` — propaga `daemon_declaration_patch`.

## Conservado

`event-sweeper` sigue invocando las mismas sondas. `probe_bus_topology` no se reescribe. El modo por defecto sigue siendo `filesystem`.
