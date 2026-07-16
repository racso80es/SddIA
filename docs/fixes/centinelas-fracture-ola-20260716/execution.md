---
feature_name: centinelas-fracture-ola-20260716
created: "2026-07-16"
process: bug-fix
branch: fix/centinelas-fracture-ola-20260716
---

# Execution

1. Rama `fix/centinelas-fracture-ola-20260716` desde `main`.
2. Inventario: 13 PBIs heartbeat (watcher 5 / sweeper 3 / telegram 3 / github-bridge 2).
3. Empírico: keepalive ya emite; `heartbeat-audit.json` missed=0 tras reinicio local.
4. Código: dedupe `materialize-fracture-pbi` por process_name abierto.
5. Tests: `cargo test -p execute-process materialize_` → 8 ok.
6. 13 satélites → `docs/todos/done/`.
