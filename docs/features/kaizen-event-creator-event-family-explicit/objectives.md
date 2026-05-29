---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
branch_name: feat/kaizen-event-creator-event-family-explicit
persist_ref: docs/features/kaizen-event-creator-event-family-explicit
pbi_ref: docs/todos/done/[Kaizen] event-creator — eliminar default event_family domain.md
document_id: PBI-KAIZEN-EVENT-CREATOR-EVENT-FAMILY-EXPLICIT
---

# Objetivos — Kaizen event-creator: event_family explícito

## Misión

Retirar el fallback D1.9 (`event_family` ausente → `domain`) del proceso `event-creator` y del runtime que normaliza `process_inputs`, exigiendo familia Trinidad explícita y error de validación si falta.

## Objetivos

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Input explícito | `event-creator.md` v1.2.0; sin fase de default |
| O2 | Inventario migrado | Labs y ejemplos con `event_family` en semilla |
| O3 | Documentación Fase 1 | spec/clarify/validacion Fase 1 sin deuda Kaizen pendiente |
| O4 | Regresión | `run_event_forge` fractal + tests bus verdes |

## Touchpoints

- `SddIA/process/event-creator.md`
- `SddIA/process/entity-manager.md`
- `SddIA/scripts/qa/execute_process_capsules.py`
- `SddIA/scripts/qa/run-eda-e2e-lab.py`
- `docs/features/telemetria-reactiva-eda-fase1/`
- `docs/features/ola-c-event-entity/execution.md`

## No objetivos

- `event_family` en envelope de instancia ECST (deuda Fase 3.C).
- Cambiar taxonomía Trinidad ni rutas del bus fractal.
