---

document_id: PBI-FIX-FRACTURE-00cc4fe00648
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: event-watcher
fracture_hash: 00cc4fe00648
incident_ref: "System_Fracture_Detected — 00cc4fe00648"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/centinelas-heartbeat-fracture/validacion.md


# [FIX] event-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-watcher omitió 5654 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T10:45:21Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Duplicado de incidente** — misma causa raíz que `a142ad1d25b3`: inanición de heartbeat durante `invoke_route_process` y barrido multi-directorio.

**Resolución:** consolidada en `docs/fixes/centinelas-heartbeat-fracture/` (PR consolidado).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
