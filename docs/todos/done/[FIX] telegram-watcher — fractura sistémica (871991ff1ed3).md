---

document_id: PBI-FIX-FRACTURE-871991ff1ed3
title: "[FIX] telegram-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-16"
status: "cerrado"
priority: alta
process: bug-fix
fracture_process: telegram-watcher
fracture_hash: 871991ff1ed3
incident_ref: "System_Fracture_Detected — 871991ff1ed3"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/validacion.md


# [FIX] telegram-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `telegram-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela telegram-watcher omitió 320 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T05:56:02Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Duplicado de incidente** — misma causa raíz que `a50853644802` y `67a56998121e`: bloqueo síncrono de `getUpdates` sin heartbeat intermedio; agravante HTTP 409 (instancia duplicada/webhook).

**Resolución:** consolidada en `docs/fixes/telegram-watcher-heartbeat-fracture-67a56998121e/` (PR #98).

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
