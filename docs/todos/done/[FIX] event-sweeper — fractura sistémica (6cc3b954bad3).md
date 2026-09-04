---
document_id: PBI-FIX-FRACTURE-6cc3b954bad3
title: "[FIX] event-sweeper — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: cerrado
closed: "2026-09-04"
priority: alta
process: bug-fix
fracture_process: event-sweeper
fracture_hash: 6cc3b954bad3
incident_ref: "System_Fracture_Detected — 6cc3b954bad3"
fix_ref: docs/fixes/centinelas-fracture-ola-20260901
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
Centinela event-sweeper lock huérfano: PID 7007 muerto. last_heartbeat=2026-09-01T14:30:52Z
```

## Cierre (ola 20260901)

Laudo **(B) deuda documental**: snapshot lock huérfano 2026-09-01T14:30Z; runtime sano al 2026-09-04 (`missed_cycles=0`, PID 67914 vivo). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260901`.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
