---

feature_name: eda-redb-03-watcher-vitality
created: "2026-10-21"
process: feature
document_id: PBI-EDA-REDB-03-WATCHER-VITALITY
uuid: "9b4e7c25-8a1d-4e63-b5f0-2d8c6a1e4f97"
execution_id: "d707b171-c91f-4933-a68b-0415c19a390d"
---
# Plan — eda-redb-03-watcher-vitality

1. `list_pending_deliveries` en `sddia-event-store`.
2. Sondas y reclamo en `event-watcher`. El sweeper no se toca.
3. Carga desde store en `route_domain_event` cuando el fichero falta y el modo escribe store.
4. Rama de tier en `dispatch_subscriber` y `authorize_dispatch_workspace` en Cerbero.
5. `daemon_declaration_patch` en la forja, invocado por `entity-manager`, para jurisdicción y versión sin UUID nuevo.
6. Pruebas AC-4, AC-7 y CA-06. Evolución antes del commit de código.
