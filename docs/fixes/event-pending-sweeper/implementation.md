---
feature_name: event-pending-sweeper
created: "2026-05-22"
process: bug-fix
branch: fix/event-pending-sweeper
version_implementation: "1.0.0"
---

# Implementación — event-pending-sweeper

## Cambios de código

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/eda_bus_utils.py` | Nuevas funciones `processed_subscriber_names`, `required_subscriber_ids_for_event`, `try_sweep_event` |
| `SddIA/scripts/qa/route_domain_event_core.py` | Invoca `try_sweep_event` al cierre de `route_domain_event`; expone `data.sweep` |
| `SddIA/scripts/daemons/event-sweeper.py` | Refactor: delega en `try_sweep_event` por UUID |
| `SddIA/scripts/daemons/event-watcher.py` | Logs según `sweep.status`; elimina mensaje ambiguo «padre permanece en pending» |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | Criterio éxito: `parent_purged` + `sweep.status == purged` |
| `SddIA/events/events-contract.md` | §4: purga inmediata vía route + sweeper como recolector stale |
| `README.md` | Pipeline pasos 3 y 5 actualizados |

## Helper `try_sweep_event`

Estados retornados:

| status | purged | Comportamiento |
|--------|--------|----------------|
| `absent` | false | Padre ya no está en pending |
| `kaizen` | false | Dead-letter presente — no purgar |
| `in-flight` | false | Suscriptores en processing/subscribers |
| `awaiting` | false | Faltan testigos processed |
| `purged` | true | `archive_event_after_sweep` ejecutado |
| `no-subscribers` | false | Sin suscriptores aplicables (sin cambio) |

## Idempotencia

Llamadas repetidas a `try_sweep_event` tras purga retornan `status: absent` sin efecto secundario.
