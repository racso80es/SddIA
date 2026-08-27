---
uuid: "78e94d53-0445-4394-b399-3e594cabc511"
name: "iota-publish-relay"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "ecosystem-evolution"
hash_signature: "sha256:e31c41ba05313347493d04caa8fe5dca1487b8cd118b8b7651da926a7de7d320"
capabilities:
  - "iota-relay-supervise"
  - "dlt-publish-http"
execution:
  entrypoint: "SddIA/daemons/iota-publish-relay.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguencia Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# iota-publish-relay

Supervisor Rust de la aduana DLT: lock, Daemon_Heartbeat y spawn/reap del hijo Node (server.mjs). Ceguencia lógica absoluta.

Forja: `daemon-creator` (porte nativo `run_daemon_forge`). UUID vía `action:crypto-broker`.
