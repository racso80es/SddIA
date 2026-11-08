---





feature_name: eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
execution_id: "976c68ea-7630-4f70-86b8-be44bcb4a8d5"
---
# Implementación — MAN-33 cutover

## T0 (contrato 1.1.0)

- Schema, puerto, adapters, mock y suite en `SddIA/sddia-core` y `SddIA/infrastructure/adapters/*_event_adapter/`.

## T1–T5 (esta rama)

- **Composition root:** `SddIA/sddia-event-manager-host/` (`resolve_productive_port`, open budget `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS`).
- **Puente runtime:** `SddIA/sddia-core/src/ports/event_manager/runtime_bridge.rs`.
- **execute-process:** `engine/eda_port.rs` (thread-local + install en `run_process`); eliminada dependencia `sddia-event-store`.
- **event-watcher:** `claim_pending` / `record_delivery` vía puerto inyectado en `run_watcher`.
- **sddia-daemon-runtime / sddia-eda-catalog:** publicación y sweep vía host + puerto; `reconcile_unstamped` no-op.
- **Genoma:** `eda:event-manager` → 1.1.0 en taxonomía y bindings.

## Datos (L-SCHEMA)

**Cold-cut:** sin migración legacy `events*` → `em_*`. Pendiente operativo en `events.redb` previo al merge se considera descartado.

## Gaps conocidos (T5 parcial)

- `SddIA/tools/{event-bus-audit,repo-retention-purger}` y `kalma2-bridge`, `lancedb_thought_repo` siguen referenciando `sddia-event-store`.
- **`sddia-qa`:** smoke IOTA (`write_pending`) publica vía `resolve_productive_port` + `publish_ecst`; sin `SDDIA_EVENT_STORE=filesystem` en lab.
- Crate `sddia-event-store` aún miembro del workspace hasta cierre del inventario.
- Tests de integración `execute-process` / watcher con matriz legacy pendientes de reescritura sobre mock/Redb temporal.
