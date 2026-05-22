---
feature_name: remove-route-domain-event-action
created: "2026-05-22"
process: refactorization
---

# Especificación — Retirada acción route-domain-event

## CA1 — Artefacto eliminado

| Antes | Después |
|-------|---------|
| `SddIA/actions/route-domain-event.md` | **Eliminado** |
| Fila en `actions/index.md` | **Eliminada** (8 acciones) |

## CA2 — Runtime

| Check | Esperado |
|-------|----------|
| `execute-action --action route-domain-event` | Error: acción no encontrada / sin handler |
| `execute-process --process route-domain-event` | Sigue operativo |
| `event-watcher.py` | Delega en proceso (sin shim de acción) |

## CA3 — Normativa activa

Referencias `action:route-domain-event` sustituidas por `process:route-domain-event` en:

- `SddIA/norms/execution-contexts.md` §2.7
- `SddIA/events/events-contract.md` §4
- Plantilla `SddIA/templates/eda-instance-events/README.md`

## CA4 — README

Sección **Eventos** documenta topología V3+, pipeline y comando manual de laboratorio.
