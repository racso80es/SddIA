---
document_id: PBI-FIX-FRACTURE-49b0f53ea572
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 49b0f53ea572"
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
Centinela event-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T08:36:33Z
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
