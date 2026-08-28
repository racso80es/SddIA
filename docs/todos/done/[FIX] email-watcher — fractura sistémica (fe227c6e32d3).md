---

document_id: PBI-FIX-FRACTURE-fe227c6e32d3
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-20"
updated: "2026-08-26"
status: cerrado
closed: "2026-08-26"
priority: alta
process: bug-fix
fracture_process: email-watcher
fracture_hash: fe227c6e32d3
incident_ref: "System_Fracture_Detected — fe227c6e32d3"
fix_ref: docs/fixes/centinelas-fracture-ola-20260819
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/audits/centinelas-fracturas-eventos-pending-20260826.md


# [FIX] email-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `email-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela email-watcher omitió 1532 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-19T16:26:27Z
```

## Cierre (ola 20260819)

Laudo **(B) deuda documental**: downtime histórico (~12,8 h); runtime sano al 2026-08-26 (`missed_cycles=0`, sweep `fractures_emitted: []`). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260819`. Causa probable: indisponibilidad de proceso (host/colisión multi-instancia), no bug de latido en genoma.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
