---
document_id: PBI-FIX-FRACTURE-a81b72a312d5
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — a81b72a312d5"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
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
Centinela event-sweeper omitió 3971 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-15T06:28:43Z
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
