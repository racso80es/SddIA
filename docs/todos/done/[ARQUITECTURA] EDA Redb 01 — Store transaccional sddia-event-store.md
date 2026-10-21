---
document_id: PBI-EDA-REDB-01-STORE
uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
title: "[ARQUITECTURA] EDA Redb 01 — Store transaccional sddia-event-store"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/done/historias/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-01-store
persist_ref_suggested: docs/features/eda-redb-01-store
blocked_by: []
unblocks:
  - PBI-EDA-REDB-02-WRITERS
  - PBI-EDA-REDB-03-WATCHER-VITALITY
  - PBI-EDA-REDB-04-READERS
laudos:
  - L-REDB-01
  - L-REDB-03
related:
  - SddIA/Cargo.toml
  - SddIA/sddia-io/
acceptance:
  - AC-2
---

# Store transaccional `sddia-event-store`

Historia madre: `HU-SDDIA-EDA-REDB-001` §3.1, §3.2, L-REDB-01, L-REDB-03. Este PBI no cablea orquestador, daemons ni genoma.

## 1. Entrega

Crate `SddIA/sddia-event-store` (`crate_name`: `sddia-event-store`), miembro nuevo de `SddIA/Cargo.toml`. Dependencia `redb` con pin explícito de la línea 2.x publicada. Prohibido el directorio `SddIA/crates/`.

API síncrona, sin tokio. Un `Database` abierto sobre una ruta inyectada (el llamador resuelve `.SddIA/runtime/events.redb`; este crate no lee `cumulo.paths.json`).

| Tabla | Clave | Valor |
| :--- | :--- | :--- |
| `events` | `event_id` | ECST + `family` (`telemetry`\|`orchestration`\|`domain`) + `event_type` + `created_at` |
| `family_index` | `(family, created_at, event_id)` | vacío |
| `subscriber_delivery` | `(event_id, subscriber_id)` | `pending`\|`success`\|`failed`\|`skipped` + witness JSON |
| `dead_letter_index` | `event_id` | causa + `failed_subscriber_id` |
| `dlt_audit_seals` | `event_id` | `tx_hash`, digest, `scope` |
| `progress_traces` | `(correlation_id, seq)` | frame JSON |

Operaciones mínimas: `open`, `insert_event` (evento + N filas `pending` en la misma write-tx), `set_delivery`, `seal_dlt`, `append_progress`, `list_family`, `get_event`, `dead_letter`, `replay` (borra `dead_letter_index` y deja el testigo del suscriptor fallido en `pending`).

Concurrencia: N `begin_read`. Escritura solo dentro de una write-tx. Dos `set_delivery` del mismo `event_id` y distinto `subscriber_id` no se pisan.

## 2. Fuera de alcance

- Flag `SDDIA_EVENT_STORE` y cualquier llamada desde `execute-process` o `sddia-daemon-runtime` (PBI 02).
- Borrar `.events/` o `event-sweeper`.
- Elegir suscriptores. El crate persiste los `subscriber_id` que el llamador ya resolvió.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-2 | Dos suscriptores del mismo `event_id` conservan ambos testigos tras reapertura del fichero. |
| CA-01 | `cargo test -p sddia-event-store` verde. Dimensión de clave distinta no corrompe la tabla vecina. |
| CA-02 | `SddIA/Cargo.toml` `members` incluye `sddia-event-store`. Cero ruta `SddIA/crates/`. |
