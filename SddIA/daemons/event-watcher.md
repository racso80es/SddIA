---
uuid: "f995cc89-22a7-488d-9b25-ddb1e5e3a4a4"
name: "event-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "ecosystem-evolution"
hash_signature: "sha256:130f4181b99335e7a9f359c9bdfff19198aae5456549e0953ac544868201d062"
capabilities:
  - "eda-bus-watch"
  - "route-domain-event-delegate"
execution:
  entrypoint: "SddIA/scripts/daemons/event-watcher.py"
  runtime: "python3"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# event-watcher

Centinela EDA: monitoriza `.events/pending/` y delega en `route-domain-event`. Emite `Daemon_Heartbeat` cada 30s.
