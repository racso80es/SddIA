# Implementación — Ola C V3 Coreografía

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/core/cumulo.paths.json` | `event_bus`, `eda_bus.subscribers.*` |
| `.gitignore` | `/.events/` |
| `SddIA/scripts/qa/eda_bus_utils.py` | Topología V3, testigos, bootstrap |
| `SddIA/scripts/daemons/event-watcher.py` | Padre inmutable; fan-out vía testigos |
| `SddIA/scripts/daemons/event-sweeper.py` | **Nuevo** recolector |
| `SddIA/scripts/qa/execute-action.py` | Emisión a `.events/pending/` |
| `SddIA/scripts/qa/execute_process_capsules.py` | Idem + scan Presented |
| `SddIA/scripts/tools/transit-event-payload/` | Resolución V3 |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | Assert testigos processed |
| `README.md` | Mapa `/.events/` |

## Identificador de suscriptor

Cuando `agent` se repite en una suscripción, el testigo usa `{agent}.{process|action|tool}` (ej. `cumulo.sync-entity-index`).

## Deuda residual

- Carpeta legacy `docs/events/` en clones antiguos — migración manual one-shot si aplica.
