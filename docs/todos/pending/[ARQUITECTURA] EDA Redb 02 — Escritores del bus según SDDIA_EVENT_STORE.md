---
document_id: PBI-EDA-REDB-02-WRITERS
uuid: "6e2d9b84-1c5a-4f70-9d38-4a7b2e8c0f16"
title: "[ARQUITECTURA] EDA Redb 02 — Escritores del bus según SDDIA_EVENT_STORE"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/pending/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-02-writers
persist_ref_suggested: docs/features/eda-redb-02-writers
blocked_by:
  - PBI-EDA-REDB-01-STORE
unblocks:
  - PBI-EDA-REDB-03-WATCHER-VITALITY
  - PBI-EDA-REDB-04-READERS
  - PBI-EDA-REDB-05-DLT-FRONTIER
laudos:
  - L-REDB-02
  - L-REDB-04
  - L-REDB-05
related:
  - SddIA/engine/execute-process/src/engine/eda_bus.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/fractal.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/dlt_telemetry_anchor.rs
  - SddIA/sddia-daemon-runtime/src/eda_bus.rs
  - SddIA/sddia-daemon-runtime/src/eda_sweep.rs
  - SddIA/sddia-daemon-runtime/src/lib.rs
acceptance:
  - AC-1
  - AC-3
---

# Escritores del bus según `SDDIA_EVENT_STORE`

Historia madre: §3.1 cutover, §3.3 exclusiones, L-REDB-02, L-REDB-04, L-REDB-05. Depende del crate del PBI 01.

## 1. Entrega

Variable `SDDIA_EVENT_STORE`:

| Valor | Escritura | Lectura de despacho |
| :--- | :--- | :--- |
| `filesystem` (default) | Solo `.events/` como hoy | Ficheros |
| `dual` | Fichero y `events.redb` en la misma operación lógica. Si `redb` falla, la operación falla (fail-closed); no queda fichero sin fila. | Store si la clave existe; si no, fichero |
| `redb` | Solo `sddia-event-store` | Solo store |

Puntos de escritura a envolver, sin cambiar el contrato ECST:

- `execute-process`: `eda_bus.rs`, `eda_bus_topology.rs` (padre + testigo `{id}.{sub}.json`), `route_domain_core.rs` (`materialize_pending_domain_event`), `fractal.rs` / `fractal_bus.rs` (mapa `delivery_state`), emisión de frames de `.events/progress/{correlation_id}`.
- `sddia-daemon-runtime`: `eda_bus.rs`, `eda_sweep.rs`, emisores de telemetría/orquestación/dominio en `lib.rs`.

En `dual` y `redb`, el testigo de suscriptor va a `subscriber_delivery`, no a un segundo JSON. El frame de progreso va a `progress_traces`. `family` sale de la carpeta lógica actual (`telemetry`, `orchestration`, `domain`).

No se mueven:

- `.SddIA/daemons/state/heartbeats/`
- `.SddIA/proofs/` (incluido `dlt-telemetry`)
- `SddIA/events/` ni `.SddIA/events`

`dlt_telemetry_anchor.rs` sigue pudiendo escribir el proof local. Este PBI no quita la llamada IOTA (eso es el PBI 05). Si el modo es `redb` o `dual` y el ancla devuelve `tx_hash`, se copia a `dlt_audit_seals` sin dejar de hacer lo que el publisher hace hoy.

## 2. Fuera de alcance

- Cambiar el default a `redb` (PBI 07).
- Hacer que `event-watcher` deje de escanear `.events/pending/` (PBI 03). El watcher actual sigue viendo los ficheros mientras el default sea `filesystem`.
- Baja de suscriptores IOTA.
- `cumulo.paths.json`.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-1 | Con `SDDIA_EVENT_STORE=redb`, un ECST de dominio insertado por `route-domain-event` está en `events` y no aparece ningún `.json` nuevo bajo `.events/`. |
| AC-3 | Con `filesystem` (o variable ausente), `eda_e2e_lab` y la paridad de suscripciones siguen en verde. |
| CA-04 | `dual`: mismo `event_id` en fichero y en `events`; dos suscriptores no comparten una sola fila. |
| CA-05 | Un frame de progreso de correlación conocida se lee de `progress_traces` con `seq` monótono. El side-channel de heartbeats no gana filas en `events`. |
