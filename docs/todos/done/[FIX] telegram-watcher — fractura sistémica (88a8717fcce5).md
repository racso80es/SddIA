---

document_id: PBI-FIX-FRACTURE-88a8717fcce5
title: "[FIX] telegram-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-07-16"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: telegram-watcher
fracture_hash: 88a8717fcce5
incident_ref: "System_Fracture_Detected — 88a8717fcce5"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md


# [FIX] telegram-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `telegram-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela telegram-watcher omitió 3971 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-15T06:28:57Z
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
