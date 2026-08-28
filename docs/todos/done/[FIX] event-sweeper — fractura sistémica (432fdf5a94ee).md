---

document_id: PBI-FIX-FRACTURE-432fdf5a94ee
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-08-19"
updated: "2026-08-26"
status: cerrado
closed: "2026-08-26"
priority: alta
process: bug-fix
fracture_process: event-sweeper
fracture_hash: 432fdf5a94ee
incident_ref: "System_Fracture_Detected — 432fdf5a94ee"
fix_ref: docs/fixes/centinelas-fracture-ola-20260819
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - docs/audits/centinelas-fracturas-eventos-pending-20260826.md


# [FIX] event-sweeper — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `event-sweeper` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela event-sweeper omitió 788 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-19T08:40:36Z
```

## Cierre (ola 20260819)

Laudo **(B) deuda documental**: downtime histórico (~6,6 h); runtime sano al 2026-08-26 (`missed_cycles=0`, PID 49944 vivo). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260819`.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
