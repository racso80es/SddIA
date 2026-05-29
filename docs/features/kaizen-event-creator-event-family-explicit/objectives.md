---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
branch_name: feat/refactorization-kaizen-event-creator-event-family-explicit
persist_ref: docs/features/kaizen-event-creator-event-family-explicit
pbi_ref: docs/todos/pending/[Kaizen] event-creator — eliminar default event_family domain.md
document_id: PBI-KAIZEN-EVENT-CREATOR-EVENT-FAMILY-EXPLICIT
related:
  - SddIA/process/event-creator.md
  - SddIA/process/entity-manager.md
  - SddIA/scripts/qa/execute_process_capsules.py
  - docs/features/telemetria-reactiva-eda-fase1/clarify.md
  - docs/features/telemetria-reactiva-eda-fase1/spec.md
---

# Objetivos — Kaizen event-creator event_family explícito

## Misión

Refactorizar `event-creator` y el runtime que resuelve `process_inputs` para **eliminar el fallback** `event_family → "domain"` (decisión **D1.9**, Fase 1) y exigir familia Trinidad explícita en toda invocación de forja ECST.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Fase 1 mergeada (`event-creator.md` v1.1.0) | Contrato documenta default `domain`; Kaizen diferido |
| Fase 3 mergeada (PR #54, `validacion.md` APTO) | Disparador PBI §5 cumplido — Aduana Universal operativa |
| `run_event_forge` en `execute_process_capsules.py` | **Desalineado**: escribe en `SddIA/events/{name}.md` plano; no usa `event_family` |
| `creator_inputs_from_entity` (piloto `event`) | No propaga `event_family` desde `semantic_seed` |
| Emisores fractales (`eda_bus_utils`, acciones) | Ya declaran `event_family` explícito en instancias |

## Objetivos medibles — PBI O1–O4

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | **Input explícito** | `event-creator.md`: `event_family` obligatorio; ausencia/vacío → error en fase Validación de Arquitectura |
| **O2** | **Inventario migrado** | Cero invocaciones `event-creator` / `entity-manager` → `event` sin `event_family` en payloads documentados y seeds runtime |
| **O3** | **Documentación** | `telemetria-reactiva-eda-fase1` §1.C y `clarify.md` D1.9 actualizados; nota deprecación default retirada |
| **O4** | **Regresión** | Tests QA forja ECST y flujos `create-event` / `entity-manager` piloto verdes |

## Alcance técnico

| Touchpoint | Cambio esperado |
|------------|-----------------|
| `SddIA/process/event-creator.md` | Quitar default; fase 0 → validación estricta; bump versión |
| `execute_process_capsules.run_event_forge` | Rutas `{family}/{name}.md`; cabecera `event_family`; índice por familia |
| `execute_process_capsules.creator_inputs_from_entity` | Propagar `event_family` desde `semantic_seed` |
| Specs / smokes con `semantic_seed` event | Añadir `event_family` explícito |
| `telemetria-reactiva-eda-fase1` docs | Cerrar deuda D1.9 / Kaizen pointer |

## Fuera de alcance

- Cambiar enum Trinidad ni rutas runtime del bus fractal (Fases 1–3).
- Forzar `event_family` en envelope de **instancia** ECST (deuda Fase 3.C documentada).

## Ley aplicada

- Proceso **`refactorization`** v1.2.0 + `features-documentation-pattern` v1.2.1.
- Contrato **`events-contract`** v1.1.0 (cabecera Clase sigue exigiendo `event_family`).
- Cierre documental en rama (un PR): PBI → `done/` + `validacion.md` `pbi_archived: true`.
