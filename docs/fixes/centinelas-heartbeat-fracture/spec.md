---
feature_name: centinelas-heartbeat-fracture
created: "2026-06-20"
process: bug-fix
base: main
scope: event-watcher, github-bridge-watcher
version_spec: "1.0.0"
---

# Especificación — Heartbeat keepalive centinelas restantes

## Diagnóstico

Mismo patrón que `telegram-watcher` (PR #98) y `event-sweeper` (PR #99):

| Centinela | Bloqueo sin latido | PBIs pending |
|-----------|-------------------|--------------|
| `event-watcher` | `invoke_route_process` + barrido multi-directorio | 5 |
| `github-bridge-watcher` | HTTP GitHub + `invoke_process_pr` | 2 |

## Corrección

Hilo keepalive: `tick()` cada 10s en modo continuo (`Arc<Mutex<DaemonRuntime>>`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Build `-p event-watcher -p github-bridge-watcher` OK |
| CA2 | `--once` exit 0 en ambos |
| CA3 | `daemon-heartbeat-audit` sweep OK |
| CA4 | 7 PBIs archivados en `done/` |
