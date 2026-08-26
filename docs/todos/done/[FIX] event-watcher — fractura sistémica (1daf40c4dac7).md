---
document_id: PBI-FIX-FRACTURE-1daf40c4dac7
title: "[FIX] event-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-20"
updated: "2026-08-26"
status: cerrado
closed: "2026-08-26"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — 1daf40c4dac7"
fix_ref: docs/fixes/centinelas-fracture-ola-20260819
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/audits/centinelas-fracturas-eventos-pending-20260826.md
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
Centinela event-watcher omitió 237 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-20T12:06:43Z
```

## Cierre (ola 20260819)

Laudo **(B) deuda documental**: downtime histórico (~2 h); runtime sano al 2026-08-26 (`missed_cycles=0`, PID 57131 vivo). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260819`.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
