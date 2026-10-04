---


feature_name: linear-hu-a-08-stamp
---
# Especificación

| Artefacto | Cambio |
|-----------|--------|
| `tracker_stamp.rs` | Eventos HU/PBI refined, cancelled, delivery committed; roll-up HU con cancelled |
| `tracker_sync_replay.rs` | Orden con `todo`; `cancelled` terminal |
| `delivery_close.rs` | Payload `Delivery_Committed` con tracker_ref/pbi_ref/persist_ref/commit_sha |
| `event-domain-subscriptions.json` | Suscriptores tracker-stamp ×4 eventos |
| `tracker-stamp.md` / `tracker-sync-replay.md` | v1.1.0 |
