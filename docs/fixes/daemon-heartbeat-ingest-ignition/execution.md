---
feature_name: daemon-heartbeat-ingest-ignition
created: "2026-07-23"
process: bug-fix
execution_id: 21d465b1-940f-45e2-879d-9cb8f773e230
---

# Execution — daemon-heartbeat-ingest-ignition

1. Auditoría empírica: `event-bus-audit` (cápsula) + `daemon-heartbeat-audit` + ignición fallida (gate 45s).
2. `bug-fix` init parcial (DNS host NXDOMAIN → agentes/git fetch fallan); materialización Tekton local en rama `fix/daemon-heartbeat-ingest-ignition`.
3. Parche `start-sddia.sh` + schema `di.binding` + rebuild `execute-process`.
4. Revalidación: DI `event-bus-audit` + smoke ignición.
