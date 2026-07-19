---
feature_name: event-sweeper-heartbeat-fracture-f4befd66c513
created: "2026-07-19"
process: bug-fix
base: main
branch: fix/event-sweeper-heartbeat-fracture-f4befd66c513
scope: daemon-heartbeat-audit
incident_ref: "System_Fracture_Detected — f4befd66c513"
version_spec: "1.0.0"
---

# Spec — Carrera post-arranque en daemon-heartbeat-audit

## Diagnóstico (causa raíz)

| Hecho | Evidencia |
|-------|-----------|
| Síntoma | `missed_cycles=39`, `last_heartbeat=2026-07-19T17:13:18Z` |
| Reinicio ecosistema | `start-sddia` ~`17:32:56Z`; lock sweeper `started_at=17:32:58Z` |
| PBI materializado | ~`17:33:01Z` (~3s tras arranque) |
| Keepalive sweeper | Presente y sano (`spawn_heartbeat_worker`, `missed_cycles=0` ahora) |

**Clasificación:** carrera del auditor tras cold-start — usa `last_heartbeat_at` obsoleto de sesión previa aunque el PID/lock sea nuevo. **No** es regresión del keepalive (`8b1ed140e48d`).

## Corrección

En `audit_running_daemon`: baseline efectiva = `max(last_heartbeat_at, lock.started_at)`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Cold-start: `last_hb` antiguo + `started_at` reciente + PID vivo → `missed_cycles < 3`, sin `System_Fracture_Detected` |
| CA2 | Caso normal: `last_hb` reciente → comportamiento invariante |
| CA3 | `cargo test -p execute-process daemon_heartbeat` OK |
| CA4 | `cargo build -p execute-process` OK |
| CA5 | PBI `f4befd66c513` en `docs/todos/done/` + `validacion.md` APTO |
