---
feature_name: event-watcher-redb-queue-logs-metrics
created: "2026-10-30"
process: feature
branch_name: feat/event-watcher-redb-queue-logs-metrics
persist_ref: docs/features/event-watcher-redb-queue-logs-metrics
pbi_ref: AUT-32
execution_id: "d126f8ae-c6c7-4507-8b11-cfc1199daa3a"
---

# Clarificación — AUT-32

Kaizen hijo de MAN-14. Ítem ya refinado en Linear (`refinement: refined`, sello Mayeuta 2026-10-30).

## Decisiones

- Alcance: observabilidad en `SddIA/daemons/event-watcher` (tick `claim_store_pending` y rutas sintéticas hacia `route-domain-event`). Sin store nuevo, sin lectura del ECST para drenar, sin enrutar JSON residual en modo redb.
- Evidencia de dolor: AUT-12 / PBI_Refined en store con watcher activo sin avance; enrutado manual ~66 s sí desbloqueó sello.
- Métrica observable: log por tick + log por ruta (elapsed_ms, exit) + línea explícita si `EventStore::open` falla (incl. Locked).
- Fuera de alcance de este ciclo: reintentos de lock en todo el árbol (AUT-55 / reconcile_unstamped); solo lo que el kaizen AUT-32 enumera en Propuesta 1–5.
