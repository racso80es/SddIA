---
uuid: "78e94d53-0445-4394-b399-3e594cabc511"
name: "iota-publish-relay"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "ecosystem-evolution"
hash_signature: "sha256:861904a9c1f290a9619832d907dd67d46625041237b1f583963c5c441d31234a"
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
