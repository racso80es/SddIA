---
document_id: PBI-EDA-REDB-04-READERS
uuid: "c1a5e8d3-4b72-4f9a-8e16-7d3c0b5a2e48"
title: "[ARQUITECTURA] EDA Redb 04 — Lectores, SSE y CLI de inspección"
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
suggested_branch: feat/eda-redb-04-readers
persist_ref_suggested: docs/features/eda-redb-04-readers
blocked_by:
  - PBI-EDA-REDB-01-STORE
  - PBI-EDA-REDB-02-WRITERS
unblocks:
  - PBI-EDA-REDB-07-CUTOVER
laudos:
  - L-REDB-04
  - L-REDB-05
related:
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/tools/event-bus-audit/src/main.rs
  - SddIA/tools/sddia-qa/src/eda_e2e_lab.rs
  - SddIA/tools/sddia-qa/src/lab_teardown.rs
  - SddIA/engine/execute-process/src/main.rs
  - start-sddia.sh
acceptance:
  - AC-8
  - AC-9
---

# Lectores, SSE de progreso y CLI

Historia madre: §3.3, §3.5, L-REDB-04, L-REDB-05. El CLI de `execute-process` es por flags (`--seal-capsules`, `--audit-eda-coverage`), no por subcomandos clap. Se mantiene ese estilo.

## 1. Entrega

Lectores que hoy caminan directorios `.events/**`. Con `filesystem` no cambian. Con `dual` o `redb` leen `sddia-event-store`:

| Lector | Hoy | Con store |
| :--- | :--- | :--- |
| `kalma2-bridge` `load_progress_path` / `handle_progress_stream` | `.events/progress/{correlation_id}` | `progress_traces` ordenado por `seq`. El frame SSE (`event: progress`) no cambia de formato. |
| `kalma2-bridge` domain, orchestration, dead-letter, telemetry | carpetas `eda_fractal` | `events` + `family_index` + `dead_letter_index` |
| `event-bus-audit` | scan de `.events/*` y testigos | `subscriber_delivery` y `dead_letter_index` |
| `sddia-qa` `eda_e2e_lab.rs` / `lab_teardown.rs` | `list_witnesses` sobre ficheros | testigos del store cuando el modo no es `filesystem` |

`start-sddia.sh` (`_ingest_telemetry_heartbeats`) deja de parsear `.events/telemetry/*.json` para el gate de arranque. Lee `.SddIA/daemons/state/heartbeats/{daemon}.json`. Ese side-channel no se muda a `redb`.

Flags nuevos en `SddIA/engine/execute-process/src/main.rs`, salida un envelope JSON por stdout:

| Flag | Efecto |
| :--- | :--- |
| `--query-events` | Exige `--inputs` con `family` y/o `event_type` y `limit`. Lista filas de `events`. |
| `--inspect-dead-letter` | Lista `dead_letter_index`. |
| `--replay-dead-letter` | Exige `event_id` en `--inputs`. Pasa esa clave a `pending` lógico (`replay` del crate). |

Modo `filesystem`: los tres flags responden error tipado `EVENT_STORE_DISABLED` (código distinto de 0). No improvisan un scan de carpetas.

## 2. Fuera de alcance

- Cambiar el contrato HTTP de Kalma2 más allá del origen de los frames.
- Mover proofs `.SddIA/proofs`.
- Baja del sweeper.

## 3. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-8 | `GET /api/progress/stream` emite, en orden, frames cuya fuente es `progress_traces`. |
| AC-9 | Los tres flags devuelven JSON. `--replay-dead-letter` quita la clave de `dead_letter_index` y deja el testigo fallido en `pending`. |
| CA-07 | El gate de heartbeats de `start-sddia.sh` confirma `event-watcher` leyendo el fichero de `.SddIA/daemons/state/heartbeats/`, con `.events/telemetry/` vacío. |
