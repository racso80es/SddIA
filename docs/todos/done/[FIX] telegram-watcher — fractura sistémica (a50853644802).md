---
document_id: PBI-FIX-FRACTURE-a50853644802
title: "[FIX] telegram-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — a50853644802"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/validacion.md
---

# [FIX] telegram-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `telegram-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela telegram-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T08:38:22Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Duplicado de incidente** — misma causa raíz que `67a56998121e` y `871991ff1ed3`: bloqueo síncrono de `getUpdates` sin heartbeat intermedio.

**Resolución:** consolidada en `docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/` (PR #98).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
