---
feature_name: daemon-heartbeat-ingest-ignition
created: "2026-07-23"
process: bug-fix
base: main
scope: heartbeat-ingest-ignition
---

# Especificación — ingest de Daemon_Heartbeat en ignición

## Causa raíz

1. `Daemon_Heartbeat` se escribe en `.events/telemetry/` pero el fan-out `route-telemetry` → `daemon-heartbeat-audit` no sella `delivery_state` a tiempo (inanición: `event-watcher` síncrono saturado por `pending/`).
2. `start-sddia.sh` invocaba `daemon-heartbeat-audit` con `{}` → solo `audit_staleness`; **no** `record_heartbeat`.
3. Residual H9: `di.binding.schema.json` rechazaba `tool:*` → `event-bus-audit` fallaba en envelope Cerbero.

Los PBI FIX `event-sweeper` / `github-bridge-watcher` / `telegram-watcher` son el mismo síntoma (`missed_cycles ≥ 3`), no defectos aislados de cada binario.

## Cambios requeridos

1. Gate ignición: ingerir el último `Daemon_Heartbeat` por Centinela obligatorio vía `event_file_path` antes del sweep.
2. Ampliar pattern provider a `^(skill|action|tool):[a-z0-9-]+$`.
3. Rebuild `execute-process` con `CARGO_TARGET_DIR=SddIA/target`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `_ingest_telemetry_heartbeats` actualiza `heartbeat-audit.json` para obligatorios sin depender del fan-out. |
| CA2 | `start-sddia.sh` alcanza «Ecosistema S+ Grade» con heartbeats frescos post-ignición (missed_cycles < 3). |
| CA3 | `./sddia-run.sh --process event-bus-audit` fase DI/envelope APTO (`tool:event-bus-audit`). |
| CA4 | Schema `di.binding` acepta `tool:`. |
| CA5 | Los 3 PBI FIX fractura archivados en `docs/todos/done/` con `validacion.md` APTO. |
