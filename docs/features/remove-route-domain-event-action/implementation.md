---
feature_name: remove-route-domain-event-action
created: "2026-05-22"
process: refactorization
---

# Implementación

## Archivos tocados

| Archivo | Cambio |
|---------|--------|
| `SddIA/actions/route-domain-event.md` | Eliminado |
| `SddIA/actions/index.md` | −1 fila; conteo 8 acciones |
| `SddIA/scripts/qa/execute-action.py` | −shim `_run_route_domain_event_shim` |
| `SddIA/norms/execution-contexts.md` | `process:route-domain-event` |
| `SddIA/events/events-contract.md` | Ciclo V3+ simétrico |
| `SddIA/templates/eda-instance-events/README.md` | Ref proceso |
| `SddIA/process/entity-manager.md` | Ref proceso |
| `SddIA/process/pull-request-review.md` | Ref proceso |
| `SddIA/actions/sync-entity-index.md` | Ref proceso |
| `SddIA/actions/emit-domain-mutation.md` | Ref proceso |
| `SddIA/actions/materialize-fracture-pbi.md` | Ref proceso |
| `README.md` | Pipeline EDA V3+ |

## Intencionalmente fuera de alcance

Manifiestos históricos (`backfill-manifest.json`, features cerrados) conservan rutas legacy como evidencia forense.
