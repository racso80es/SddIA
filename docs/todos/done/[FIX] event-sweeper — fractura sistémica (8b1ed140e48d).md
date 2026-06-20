---
document_id: PBI-FIX-FRACTURE-8b1ed140e48d
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 8b1ed140e48d"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/event-sweeper-heartbeat-fracture-8b1ed140e48d/validacion.md
---

# [FIX] event-sweeper — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-sweeper` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-sweeper omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-18T09:52:24Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Causa raíz:** `sweep_once` bloquea el hilo principal sin emitir `Daemon_Heartbeat` intermedio (intervalo 30s).

**Corrección:** hilo keepalive (`tick` cada 10s) en modo continuo. Distinto de `event-pending-sweeper` (lógica de purga).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
