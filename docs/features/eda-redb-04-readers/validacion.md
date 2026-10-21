---
feature_name: eda-redb-04-readers
created: "2026-10-21"
branch: feat/eda-redb-04-readers
branch_name: feat/eda-redb-04-readers
persist_ref: docs/features/eda-redb-04-readers
pbi_ref: docs/todos/pending/[ARQUITECTURA] EDA Redb 04 — Lectores, SSE y CLI de inspección.md
document_id: PBI-EDA-REDB-04-READERS
execution_id: "88fb5a6e-ed41-49ab-ab64-dcf602f60e4a"
global: APTO
pbi_archived: false
checks:
  AC-8: APTO
  AC-9: APTO
  CA-07: APTO
git_changes:
  - SddIA/sddia-event-store/src/lib.rs
  - SddIA/engine/execute-process/src/engine/event_store_query.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/main.rs
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs
  - SddIA/interfaces/kalma2-bridge/Cargo.toml
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/tools/event-bus-audit/Cargo.toml
  - SddIA/tools/event-bus-audit/src/main.rs
  - SddIA/tools/sddia-qa/src/lab_teardown.rs
  - SddIA/Cargo.lock
  - start-sddia.sh
  - docs/features/eda-redb-04-readers/
---

# Validación — eda-redb-04-readers

AC-8: `progress_stream_emits_store_frames_in_seq_order` lee `progress_traces` y emite `event: progress` con `seq` 0 antes que `seq` 1.

AC-9: en `filesystem` los tres flags devuelven `EVENT_STORE_DISABLED` y no crean la base. En `redb`, `--replay-dead-letter` vacía `dead_letter_index` y deja el testigo en `Pending`; `--query-events` e `--inspect-dead-letter` devuelven el envelope.

CA-07: `side_channel_heartbeat_is_audited_without_telemetry` audita `.SddIA/daemons/state/heartbeats/event-watcher.json` con `.events/telemetry/` ausente. `_ingest_telemetry_heartbeats` lista ese directorio.

El PBI sigue en pending. `pbi_archived` permanece false hasta el cierre documental.
