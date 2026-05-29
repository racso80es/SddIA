---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
---

# Implementación

## Código

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute_process_capsules.py` | `TRINITY_EVENT_FAMILIES`, `resolve_effective_event_family`, `run_event_forge` fractal |
| `SddIA/process/event-creator.md` | v1.2.0, input obligatorio, fases sin fallback |
| `SddIA/process/entity-manager.md` | `event_family` en tabla semilla |
| `SddIA/scripts/qa/run-eda-e2e-lab.py` | `event_family: domain` en forja event E2E |

## Documentación

| Archivo | Cambio |
|---------|--------|
| `docs/features/telemetria-reactiva-eda-fase1/spec.md` | §6.1 sin default |
| `docs/features/telemetria-reactiva-eda-fase1/clarify.md` | D1.9 cerrado |
| `docs/features/telemetria-reactiva-eda-fase1/validacion.md` | Kaizen cerrado |
| `docs/features/ola-c-event-entity/execution.md` | ejemplo con `event_family` |
| `docs/todos/done/[Kaizen] event-creator — …` | PBI archivado |
