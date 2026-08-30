---
capabilities:
- eda-bus-watch
- route-domain-event-delegate
context: ecosystem-evolution
contract: daemons-contract v1.0.0
execution:
  entrypoint: SddIA/daemons/event-watcher.sh
  heartbeat_interval_seconds: 30
  runtime: native-rust
hash_signature: sha256:8fffedc6e00bc0f9d70c3723ee7c98ba0c5e083e19a2cdaa400e30ad6aa42fa6
jurisdiction: Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus
name: event-watcher
source_sha256: sha256:0641700028cde139bcff669282a059f8a2f5184f6ddad5a40ec5c13e2cf98f7e
telemetry_provided: true
telemetry_schema:
- uptime_seconds
- pid
- status
uuid: f995cc89-22a7-488d-9b25-ddb1e5e3a4a4
version: 1.1.0
---

# event-watcher

Centinela EDA: monitoriza `.events/pending/` y delega en `route-domain-event` vía `execute-process`. Binario Rust en `SddIA/target/{release|debug}/event-watcher`; launcher `SddIA/daemons/event-watcher.sh`. Emite `Daemon_Heartbeat` cada 30s.
