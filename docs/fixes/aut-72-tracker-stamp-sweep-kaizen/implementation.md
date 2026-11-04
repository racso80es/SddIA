---
feature_name: aut-72-tracker-stamp-sweep-kaizen
created: "2026-11-04"
process: bug-fix
branch_name: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-72-tracker-stamp-sweep-kaizen
pbi_ref: AUT-72
execution_id: "523a4752-c526-488d-aaa0-d9a12b1275c6"
---

# Implementación — AUT-72

## T1 — Barrido EDA

Archivo: `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs`

- Constante `TRACKER_STAMP_SUBSCRIBER` = `tekton.tracker-stamp`.
- Helpers: `payload_tracker_ref`, `event_requires_tracker_done_seal`, `dead_letter_includes_tracker_stamp`, `pending_tracker_sync_failed_for_ref`, `tracker_stamp_done_reconciled`.
- Guard en `try_sweep_event` (rama dead-letter + terminales): emite `tracker-stamp-unreconciled` con `purged: false` antes de `kaizen-finalized`.

## T2 — Idempotencia Done

Archivo: `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs`

- `stamp_done_with_hu_roll_up`: si `fetch` ya está en estado Done (canónico), escribe proof local si falta y retorna sin re-mutar Linear ni emitir `Tracker_Sync_Failed`.

## T4 — Tests

- `aut72_tracker_stamp_dead_letter_not_kaizen_finalized`
- `aut72_tracker_stamp_dead_letter_kaizen_finalized_with_proof`
