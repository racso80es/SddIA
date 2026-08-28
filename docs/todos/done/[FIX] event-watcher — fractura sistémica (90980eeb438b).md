---

document_id: PBI-FIX-FRACTURE-90980eeb438b
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: event-watcher
fracture_hash: 90980eeb438b
incident_ref: "System_Fracture_Detected — 90980eeb438b"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md


# [FIX] event-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-watcher omitió 37 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T16:08:11Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

_Pendiente de síntesis Mayeuta (Kintsugi async)._



## Resolución (ola 2026-07-16)

**Duplicado / satélite** — consolidado en `docs/fixes/centinelas-fracture-ola-20260716/` (PBI-CENTINELAS-FRACTURE-OLA-20260716).
Causa de spam: materialize-fracture-pbi hasheaba traza variable; idempotencia por `process_name` corregida en este fix.

## Criterio de cierre

- [x] Causa raíz resuelta (consolidada)
- [x] Argos APTO en `validacion.md` del fix (ola)
- [x] Este TODO movido a `docs/todos/done/`
