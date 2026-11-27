---
uuid: "5171f316-d417-4e4d-99ee-884b8008bb01"
name: "linear-bridge-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "tracker-operations"
hash_signature: "sha256:fc466f60e1d59b1449fe9f568a543d3d27a803ab0ffdd61f6eb8d6551704bfaa"
capabilities:
  - "linear-tracker-poll"
  - "eda-bus-inject"
execution:
  entrypoint: "SddIA/daemons/linear-bridge-watcher.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 60
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# linear-bridge-watcher

Sensor Linear: polling list_issues y POST webhook lab (bind local, firma HMAC); materializa Tracker_Issue_Changed (`source`: poll | webhook); anti-eco vía tracker-outbound; sin apply markdown.

Variables lab (no en manifiesto): `SDDIA_LINEAR_WEBHOOK_PORT`, `SDDIA_LINEAR_WEBHOOK_BIND` (default `127.0.0.1`), `LINEAR_WEBHOOK_SECRET`.

Forja: `daemon-creator` (porte nativo `run_daemon_forge`). UUID vía `action:crypto-broker`.
