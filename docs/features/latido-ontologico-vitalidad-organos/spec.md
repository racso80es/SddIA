---
feature_name: latido-ontologico-vitalidad-organos
created: "2026-08-31"
process: feature
base: main
scope: core
pbi_ref: docs/todos/done/[OPERATIVO] Latido Ontológico (System Heartbeat).md
execution_id: "cb141830-b5e3-4b9e-904d-014922254734"
---

# Spec — latido-ontologico-vitalidad-organos

## Contratos

### `Daemon_Heartbeat` (update)

Emisores autorizados = stems de `SddIA/daemons/index.md` que emiten latido:

`event-watcher`, `event-sweeper`, `telegram-watcher`, `github-bridge-watcher`, `email-watcher`, `iota-publish-relay`, y tras Fase 2 `kalma2-bridge`.

UUID de clase inmutable (`9c5190ac-ac8a-46b6-b61d-67d45ff7caf1`). Update vía `entity-manager` + `markdown_body_replacements` (no regenerar uuid).

### `kalma2-bridge` (daemon create)

- `execution.entrypoint`: `SddIA/scripts/daemons/kalma2-bridge.sh` (existente)
- `runtime`: `native-rust`
- `heartbeat_interval_seconds`: 30
- `context`: `system-operations`
- Crate: `sddia-daemon-runtime`; hilo keepalive `tick`; `ctrlc` → `shutdown` (quita lock). Crash/`kill -9` deja lock huérfano.

### Auditor

`audit_running_daemon`:

1. Sin lock → silencio (no arrancó / stop limpio).
2. Lock + `!pid_alive` + sin `fracture_event_id` → `System_Fracture_Detected` (causa: lock huérfano / PID muerto).
3. Lock + PID vivo + `missed_cycles >= umbral` → fractura de cuelgue (regresión prohibida).
4. `host_suspend` no aplica al caso 2.

### `System_Vitality_Probed` (create, telemetry)

Payload REQUIRED: `probes` (array), `verdict` (`ok` \| `degraded`). OPTIONAL: `red_probe_ids`. Emisor: `system-vitality-probe` (CLI).

### `system-vitality-probe` (process, Argos)

Sondas: `bus.topology`, `cumulo.tools_index`, `cerbero.config`, `kalma2.http`. Rojo → `System_Fracture_Detected` idempotente por `probe_id` (estado bajo `.SddIA/daemons/state/vitality-probe.json`).

### Sweeper

`SDDIA_VITALITY_PROBE_SECONDS` default 300, piso 30. Invoca el proceso como ya invoca `daemon-heartbeat-audit`. Capability `vitality-probe-sweep` en genoma `event-sweeper`.
