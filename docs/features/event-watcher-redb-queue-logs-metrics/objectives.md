---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
branch_name: feat/event-watcher-redb-queue-logs-metrics
persist_ref: docs/features/event-watcher-redb-queue-logs-metrics
pbi_ref: AUT-32
execution_id: "d126f8ae-c6c7-4507-8b11-cfc1199daa3a"
---

# Objetivos — event-watcher-redb-queue-logs-metrics

## Objetivo

Hacer visible en logs (y contadores en proceso) el comportamiento del `event-watcher` en `StoreMode::Redb`: tamaño de cola, ids reclamados o saltados, duración de rutas sintéticas y causa cuando `events.redb` no abre.

## Alcance

- `SddIA/daemons/event-watcher/src/main.rs`: `claim_store_pending`, spawn/cierre de `route-domain-event`, manejo de `EventStore::open` en el tick.
- Mantener drenado por id sin ECST y sin reinyectar JSON residual en redb (F-EDA-RESIDUAL-FILESYSTEM sigue siendo guardrail, no objetivo de borrado en este PBI salvo que el diseño lo exija).
- Documentación de ciclo bajo `docs/features/event-watcher-redb-queue-logs-metrics/`.

## Criterios de aceptación

1. Cada tick de `claim_store_pending`: una línea con modo de store, `pending_rows`, `claimed`, `skipped_file`, `skipped_book`, `skipped_inflight`, `open_failed`.
2. Al spawn y al cierre de cada ruta sintética: `event_id`, suscriptor, `elapsed_ms`, exit code.
3. Contadores en proceso (reclamados, éxito, fallo, cola observada); volcado periódico en log o heartbeat del daemon.
4. Si `EventStore::open` falla en el tick, log con causa (Locked u otro); no tick mudo.
5. Caso de regresión documentado: fila Pending de dominio + watcher vivo → log muestra `pending_rows >= 1` y, en el tick que reclama, `claimed >= 1` con `event_id`; tras `route-domain-event` exit 0, `event-bus-audit` refleja avance en esa fila.

Referencia canónica: [AUT-32](https://linear.app/oscar-perez/issue/AUT-32/event-watcher-en-redb-logs-y-metricas-de-la-cola-de-entregas).
