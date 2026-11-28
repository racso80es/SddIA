---
uuid: "78e94d53-0445-4394-b399-3e594cabc511"
name: "iota-publish-relay"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "ecosystem-evolution"
hash_signature: "sha256:3ef6fe4cb25057d417d800aeba853a43107d9b6b7c5e7446db480a1470e47f2f"
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

## Propósito

Supervisor Rust de la aduana DLT: lock, Daemon_Heartbeat y spawn/reap del hijo Node (server.mjs). Ceguencia lógica absoluta.

## Ciclo de ejecución

Latido/tick vía entrypoint en frontmatter; binario nativo en `SddIA/target/release/<daemon>`.

Forja: `daemon-creator` (porte nativo `run_daemon_forge`). UUID vía `action:crypto-broker`.
