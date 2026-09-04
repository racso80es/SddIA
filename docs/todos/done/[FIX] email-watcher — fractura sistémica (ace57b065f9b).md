---
document_id: PBI-FIX-FRACTURE-ace57b065f9b
title: "[FIX] email-watcher — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: cerrado
closed: "2026-09-04"
priority: alta
process: bug-fix
fracture_process: email-watcher
fracture_hash: ace57b065f9b
incident_ref: "System_Fracture_Detected — ace57b065f9b"
fix_ref: docs/fixes/centinelas-fracture-ola-20260901
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] email-watcher — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `email-watcher` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela email-watcher lock huérfano: PID 638582 muerto. last_heartbeat=2026-09-01T14:30:52Z
```

## Cierre (ola 20260901)

Laudo **(B) deuda documental**: snapshot lock huérfano 2026-09-01T14:30Z; runtime sano al 2026-09-04 (`missed_cycles=0`, PID 75943 vivo). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260901`. Diagnóstico Mayeuta (backfill EDA) descartado: colisión token `huérfan`.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
