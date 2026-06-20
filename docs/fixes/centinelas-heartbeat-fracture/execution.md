---
feature_name: centinelas-heartbeat-fracture
created: "2026-06-20"
process: bug-fix
branch: fix/centinelas-heartbeat-fracture-consolidado
---

# Ejecución

```bash
cd SddIA && cargo build -p event-watcher -p github-bridge-watcher
SddIA/target/debug/event-watcher --once
SddIA/target/debug/github-bridge-watcher --once
```
