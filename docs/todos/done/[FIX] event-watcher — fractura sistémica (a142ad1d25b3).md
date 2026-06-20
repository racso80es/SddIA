---
document_id: PBI-FIX-FRACTURE-a142ad1d25b3
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — a142ad1d25b3"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/centinelas-heartbeat-fracture/validacion.md
---

# [FIX] event-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-18T09:52:24Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Causa raíz:** inanición de heartbeat durante `invoke_route_process` y barrido multi-directorio — el hilo principal queda bloqueado sin emitir `Daemon_Heartbeat` intermedio (intervalo 30s) → `missed_cycles ≥ 3` con PID vivo.

**Corrección:** hilo keepalive (`tick` cada 10s) en modo continuo (`Arc<Mutex<DaemonRuntime>>`).

**Resolución:** consolidada en `docs/fixes/centinelas-heartbeat-fracture/` (PR consolidado).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
