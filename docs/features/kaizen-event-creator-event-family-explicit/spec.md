---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
base: main
scope: kaizen-event-creator-event-family-explicit
version_spec: "1.0.0"
---

# Especificación — Kaizen event-creator event_family explícito

## H1 — Contrato `event-creator.md` (v1.2.0)

| Campo | Antes (v1.1.0 / D1.9) | Después |
|-------|----------------------|---------|
| Input `event_family` | Opcional; default `domain` | **Obligatorio** |
| Fase 0 | Normalización con fallback | Validación estricta; ausente/vacío → error |
| Fase 1 | Sobre `effective_event_family` | Igual; sin rama default |

```yaml
- "event_family":
    description: "Familia Trinidad: telemetry | orchestration | domain"
    required: true
```

**Error canónico (runtime lab):** `ValueError: event_family es obligatorio (telemetry | orchestration | domain)`

Enum rechazado: cualquier valor fuera de `{ telemetry, orchestration, domain }`.

## H2 — Runtime `run_event_forge`

| Aspecto | Comportamiento |
|---------|----------------|
| Resolución familia | `resolve_event_family_required(inputs)` — sin fallback |
| Ruta artefacto | `SddIA/events/{event_family}/{event_name}.md` |
| Cabecera YAML | Incluir `event_family: "{family}"` (coherente con `events-contract` v1.1.0) |
| Índice | Actualizar `SddIA/events/{event_family}/index.md` (no raíz plana) |
| `events_contract_version` default | `1.1.0` |
| Output `artifact_events_index` | Path relativo del índice de familia |

### Inserción en Códice de familia

Fila canónica (alineada con `domain/index.md`):

```text
| `{event_name}.md` | `{uuid}` | {name} | {event_type} | {version} | events-contract v{ver} | {context} | `{cap}` |
```

Idempotencia create: si `{family}/{name}.md` existe → `FileExistsError`.

## H3 — Puente `entity-manager`

| Campo `semantic_seed` | Input `event-creator` | Obligatorio |
|-----------------------|-------------------------|-------------|
| `event_family` | `event_family` | **Sí** |

Propagación en `creator_inputs_from_entity` (`entity_class == "event"`).

Documentar fila en tabla `entity-manager.md` § Fase 1.

## H4 — Inventario migración (O2)

| Touchpoint | Acción |
|------------|--------|
| `run-eda-e2e-lab.py` | `semantic_seed.event_family: "domain"` cuando `entity_class == "event"` |
| `ola-c-event-entity/execution.md` smoke JSON | Añadir `"event_family": "domain"` |
| Docs Fase 1 `spec.md` §6.1 / `clarify.md` D1.9 | Nota: default retirado por este Kaizen |
| Emisores instancia (`write_fractal_event`) | Sin cambio (fuera alcance forja Clase) |

Barrido grep (criterio cierre O2):

```text
semantic_seed / process_inputs de forja event sin clave event_family en payloads ejecutables documentados
→ cero coincidencias activas
```

## H5 — Regresión (O4)

| Smoke | Entrada | Esperado |
|-------|---------|----------|
| Forja domain | `"event_family": "domain"` | Artefacto bajo `events/domain/` |
| Forja telemetry | `"event_family": "telemetry"` | Artefacto bajo `events/telemetry/` |
| Sin familia | omitir `event_family` | `ValueError` |
| `unittest` | `test_event_forge_fractal.py` | 3 casos anteriores en tmp repo |

Suite existente: `test_eda_bus_v3plus.py` — sin regresión.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| KEC-CA1 | `event-creator.md` v1.2.0 sin default; input obligatorio |
| KEC-CA2 | `run_event_forge` enruta fractal + índice familia |
| KEC-CA3 | `creator_inputs_from_entity` propaga `event_family` |
| KEC-CA4 | Inventario O2 migrado (lab + smokes documentados) |
| KEC-CA5 | Tests forja verdes |
| KEC-CA6 | `validacion.md` APTO + PBI en `done/` (un PR) |

## Fuera de alcance

- `event_family` en envelope de instancia ECST (deuda Fase 3.C).
- Cambio enum Trinidad o rutas bus fractal.
