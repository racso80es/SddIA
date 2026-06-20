---
feature_name: event-sweeper-heartbeat-fracture-8b1ed140e48d
created: "2026-06-20"
process: bug-fix
branch: fix/event-sweeper-heartbeat-fracture-8b1ed140e48d
---

# Ejecución

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p event-sweeper
SddIA/target/debug/event-sweeper --once --json
SddIA/target/debug/execute-process --process daemon-heartbeat-audit --inputs '{"sweep":true}'
```
