---
document_id: PBI-FIX-EVENT-PENDING-SWEEPER
title: "[FIX] event-pending-sweeper — padre permanece en pending tras enrutamiento"
format: markdown
version: "1.0.0"
created: "2026-05-22"
status: "completado"
closed: "2026-05-22"
priority: alta
process: bug-fix
incident_ref: "Bus EDA — testigos en processed/ pero JSON padre persiste en pending/"
feature_ref: docs/fixes/event-pending-sweeper
validacion_ref: docs/fixes/event-pending-sweeper/validacion.md
branch: fix/event-pending-sweeper
pr_ref: "https://github.com/racso80es/SddIA/pull/29"
merge_commit: "0ba2ac7e608db36321a51aefe4e9c1550a3d22c6"
---

# [FIX] event-pending-sweeper — padre permanece en pending tras enrutamiento

**Estado:** ✅ Completado — PR [#29](https://github.com/racso80es/SddIA/pull/29), merge `0ba2ac7`.

## Entregables

| Ítem | Resultado |
|------|-----------|
| `try_sweep_event` | Helper compartido en `eda_bus_utils.py` |
| Cierre post-route | `route_domain_event_core.py` purga `pending/` al consenso |
| Watcher | Logs reflejan purga real |
| Sweeper | Refactor delegando en helper (recolector stale) |
| E2E lab | `run-eda-e2e-lab.py` valida `parent_purged: true` |

## Invocación canónica

```bash
# Automático (watcher enruta + purga al consenso)
python SddIA/scripts/daemons/event-watcher.py --once

# Recolector stale (opcional)
python SddIA/scripts/daemons/event-sweeper.py --once
```
