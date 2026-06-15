---
uuid: "456a3d9b-70b6-4113-9283-16ba6d142793"
name: "github-bridge-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "source-control"
hash_signature: "sha256:b067cbfd8eb07d662a2b9eb59098856a38b728dec3f7c170f506855e07b63699"
capabilities:
  - "github-pr-bridge"
  - "dlt-oracle"
execution:
  entrypoint: "SddIA/scripts/daemons/github_bridge_watcher.py"
  runtime: "python3"
  heartbeat_interval_seconds: 60
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# github-bridge-watcher

Oráculo sensor DLT: detecta PRs remotos, ancla IOTA, materializa `PullRequest_Presented`. Emite `Daemon_Heartbeat` cada 60s.
