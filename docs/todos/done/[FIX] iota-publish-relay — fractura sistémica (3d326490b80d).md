---
document_id: PBI-FIX-FRACTURE-3d326490b80d
title: "[FIX] iota-publish-relay — fractura sistémica"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: cerrado
closed: "2026-09-04"
priority: alta
process: bug-fix
fracture_process: iota-publish-relay
fracture_hash: 3d326490b80d
incident_ref: "System_Fracture_Detected — 3d326490b80d"
fix_ref: docs/fixes/centinelas-fracture-ola-20260901
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
---

# [FIX] iota-publish-relay — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `iota-publish-relay` |
| Emisor | `argos` |
| Acción intentada | `daemon-heartbeat-audit` |

## Traza de error

```
Centinela iota-publish-relay lock huérfano: PID 653392 muerto. last_heartbeat=2026-09-01T14:30:36Z
```

## Cierre (ola 20260901)

Laudo **(B) deuda documental**: snapshot lock huérfano 2026-09-01T14:30Z; runtime sano al 2026-09-04 (`missed_cycles=0`, PID 75932 vivo). Archivado bajo `docs/fixes/centinelas-fracture-ola-20260901`. Diagnóstico Mayeuta (backfill EDA) descartado: colisión token `huérfan`.

## Criterio de cierre

- [x] Causa raíz resuelta (laudo B — snapshot histórico)
- [x] Archivado en ola documental
- [x] Este TODO movido a `docs/todos/done/`
