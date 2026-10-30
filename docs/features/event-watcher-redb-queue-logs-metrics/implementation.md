---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
pbi_ref: AUT-32
---

# Implementación

## Cambios

- `event-watcher/src/main.rs`: observabilidad del tick de claim y rutas sintéticas (AUT-32).
- Contadores `METRIC_ROUTES_CLAIMED|OK|FAIL` y volcado periódico en log de claim.

## Fuera de alcance

- Reintento de lock global en `sddia-event-store` (AUT-55).
- Cambios en `route-domain-event` o enrutado filesystem residual.
