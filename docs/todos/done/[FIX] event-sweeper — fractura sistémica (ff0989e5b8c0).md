---

document_id: PBI-FIX-FRACTURE-ff0989e5b8c0
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: event-sweeper
fracture_hash: ff0989e5b8c0
incident_ref: "System_Fracture_Detected — ff0989e5b8c0"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/event-sweeper-heartbeat-fracture-8b1ed140e48d/validacion.md


# [FIX] event-sweeper — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-sweeper` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-sweeper omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T08:36:33Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Duplicado de incidente** — misma causa raíz que `8b1ed140e48d`. Resolución consolidada en `docs/fixes/event-sweeper-heartbeat-fracture-8b1ed140e48d/`.

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
