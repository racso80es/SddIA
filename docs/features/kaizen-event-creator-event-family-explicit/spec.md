---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: feature
---

# Especificación — event_family obligatorio

## Contrato `event-creator` v1.2.0

- Input `event_family`: obligatorio, enum `{ telemetry, orchestration, domain }`.
- Fases: Validación de Arquitectura → Forja → Gobernanza de Índice (sin «Normalización de familia»).
- `phase_invocations`: sujeto canónico SHA256 incluye `event_family`.

## Runtime `run_event_forge`

```python
effective_family = resolve_effective_event_family(inputs)  # ValueError si ausente
artifact = SddIA/events/{effective_family}/{event_name}.md
index = SddIA/events/{effective_family}/index.md
```

Cabecera YAML forjada incluye `event_family: "{effective_family}"` y `events-contract v1.1.0` por defecto.

## `entity-manager`

Tabla `semantic_seed` exige `event_family`; `creator_inputs_from_entity` propaga el campo sin default.

## Documentación heredada

- `telemetria-reactiva-eda-fase1/spec.md` §6.1 — `required: true`, sin default.
- `clarify.md` D1.9 — referencia a esta feature.
