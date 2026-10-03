---
feature_name: tracker-sync-failed
created: "2026-10-02"
process: feature
---

# Especificación — tracker-sync-failed

## Evento `Tracker_Sync_Failed`

Deuda de sincronización Linear tras fail-soft en sellado.

## Replay

- Acción `emit-tracker-sync-failed`.
- Proceso `tracker-sync-replay` suscrito vía `event-domain-subscriptions.json`.
