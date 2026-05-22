# Validación — Ola C V3 Coreografía

| Check | Estado | Evidencia |
|-------|:------:|-----------|
| `event_bus` en Cúmulo | ✅ | `"./.events"` |
| `.gitignore` `/.events/` | ✅ | raíz |
| Bootstrap 4 carpetas | ✅ | `ensure_event_bus_topology` |
| Testigo processing → processed | ✅ | smoke |
| Padre inmutable en pending | ✅ | smoke post-route |
| Sweeper purga con consenso | ✅ | smoke post-sweep |
| Alerta Kaizen dead-letter | ✅ | evento ECST inválido `99459a47-…` |
| `subscriber_id` compuesto | ✅ | `cumulo.sync-entity-index` |

| Grep: cero `docs/events/` en normativa SddIA activa | ✅ | grep 2026-05-22 |
| `verify-process-integrity.py` | ✅ | post-actualización normativa |
| `delivery-close-cycle` en PR de cierre | ⏳ | — |
