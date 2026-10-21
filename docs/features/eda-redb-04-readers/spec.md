---
feature_name: eda-redb-04-readers
created: "2026-10-21"
base: sddia-event-store
scope: lectores SSE CLI heartbeats
---

# Spec — eda-redb-04-readers

## Store

Métodos de lectura añadidos al crate: `list_deliveries`, `list_dead_letters`, `purge_event`. `replay` borra la clave de `dead_letter_index` y deja el testigo del suscriptor fallido en `Pending`.

## Lectores

| Superficie | Store |
| :--- | :--- |
| `handle_progress_stream` | `list_progress` por `correlation_id` |
| dominio / dead-letter | `get_event` + `get_dead_letter` |
| `Process_Execution_Completed` | `list_family(Orchestration)` filtrado por `payload.correlation_id` |
| telemetría cognitiva | `list_family(Telemetry)` con `telemetry_receipt` |
| `event-bus-audit` | familias, `list_dead_letters`, `list_deliveries` |
| `list_witnesses` / teardown | testigos sintéticos y `purge_event` |

## CLI

Envelope `success`, `status_code`, `exitCode`, `data`, `error`. Consulta ordenada por `created_at` descendente y truncada a `limit`.

## Gate

`_ingest_telemetry_heartbeats` lista `.SddIA/daemons/state/heartbeats/{daemon}.json` para los daemons obligatorios.
