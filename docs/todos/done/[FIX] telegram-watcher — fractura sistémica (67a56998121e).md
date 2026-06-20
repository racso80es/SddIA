---
document_id: PBI-FIX-FRACTURE-67a56998121e
title: "[FIX] telegram-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-06-18"
status: "cerrado"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 67a56998121e"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
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
Centinela telegram-watcher omitió 5653 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-06-16T10:45:34Z
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

**Causa raíz:** bloqueo síncrono de `getUpdates` (timeout=30s) sin emitir `Daemon_Heartbeat` intermedio; intervalo heartbeat=30s → `missed_cycles ≥ 3` con PID vivo. Agravante: HTTP 409 por instancia duplicada/webhook.

**Corrección:** hilo keepalive (`tick` cada 10s), `POLL_TIMEOUT=25`, `deleteWebhook` en bootstrap, backoff ante 409.

## Criterio de cierre

- [x] Causa raíz resuelta
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
