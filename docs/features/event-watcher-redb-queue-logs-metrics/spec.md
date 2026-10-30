---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
pbi_ref: AUT-32
---

# Especificación

## Motor

- `SddIA/daemons/event-watcher/src/main.rs`: `claim_store_pending`, hilos que invocan `route-domain-event`.
- Sin nuevo store ni lectura del ECST para drenar.

## Observabilidad

1. **Tick `claim_store_pending`:** línea `claim_store_pending store=… pending_rows claimed skipped_file skipped_book skipped_inflight open_failed`.
2. **Ruta sintética:** `route spawn` y `route done` con `event_id`, `subscriber=route-domain-event`, `elapsed_ms`, `exit`.
3. **Contadores:** `METRIC_ROUTES_*` acumulados; volcado cada `CLAIM_METRICS_EVERY_N_TICKS` ticks de claim.
4. **`reconcile_unstamped`:** error logueado (no `let _` silencioso).
5. **`EventStore::open` fallido:** `open_failed=1` en línea de tick + `eprintln` con causa.

## Lab

Sin flags nuevos. Comportamiento visible en stderr/stdout del daemon.
