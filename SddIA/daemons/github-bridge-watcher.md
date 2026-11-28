---
uuid: "456a3d9b-70b6-4113-9283-16ba6d142793"
name: "github-bridge-watcher"
version: "1.1.0"
contract: "daemons-contract v1.0.0"
context: "source-control"
hash_signature: "sha256:30c7aa3357e63408ba45b1a6ea8ca8bf220376fd388ff12f8840b86082154f21"
capabilities:
  - "github-pr-bridge"
  - "dlt-oracle"
execution:
  entrypoint: "SddIA/daemons/github-bridge-watcher.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 60
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---

# github-bridge-watcher

## Propósito

Oráculo sensor DLT: detecta PRs remotos (GitHub API o fixture lab), delega anclaje IOTA en handler nativo `github_bridge::process_pr`, materializa `PullRequest_Presented`. Binario Rust en `SddIA/target/{release|debug}/github-bridge-watcher`; launcher `SddIA/daemons/github-bridge-watcher.sh`. Emite `Daemon_Heartbeat` cada 60s.

## Ciclo de ejecución

Latido/tick vía entrypoint en frontmatter; binario nativo en `SddIA/target/release/<daemon>`.
