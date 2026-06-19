---
document_id: PBI-FIX-FRACTURE-49b0f53ea572
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 49b0f53ea572"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
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

_Pendiente de síntesis Mayeuta (Kintsugi async)._

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
