---
uuid: "78e94d53-0445-4394-b399-3e594cabc511"
name: "iota-publish-relay"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "ecosystem-evolution"
hash_signature: "sha256:c042faef06782190299ac111d104db0ec6548afda1773287e346d5b462e02b2d"
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

## Propósito

Supervisor Rust de la aduana DLT: lock, Daemon_Heartbeat y spawn/reap del hijo Node (server.mjs). Ceguencia lógica absoluta.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
