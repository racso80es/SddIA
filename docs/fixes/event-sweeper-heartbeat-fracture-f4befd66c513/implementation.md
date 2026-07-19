---
feature_name: event-sweeper-heartbeat-fracture-f4befd66c513
created: "2026-07-19"
process: bug-fix
---

# Implementation

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` | `effective_heartbeat_baseline` = `max(last_heartbeat_at, lock.started_at)`; traza de fractura usa baseline efectiva; tests cold-start / steady-state |
| `docs/todos/pending/[FIX] event-sweeper — fractura sistémica (f4befd66c513).md` | → `done/` al cierre |

**Fuera de alcance:** keepalive de `event-sweeper` (ya correcto desde `8b1ed140e48d`).
