---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
branch_name: feat/adecuar-ed-telemetry
persist_ref: docs/features/adecuar-ed-telemetry
pbi_ref: docs/todos/pending/event_domain_subscriptions_Adecuar_ED_Telemetry.md
document_id: PBI-ADECUAR-ED-TELEMETRY
status: validacion_apto
depends_on:
  - docs/features/eda-domain-entities-splus
  - docs/features/telemetria-reactiva-eda-fase4
related:
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/domain/
  - SddIA/actions/emit-domain-mutation.md
  - SddIA/scripts/qa/radamanto_batch_core.py
  - SddIA/scripts/qa/cerbero_governance_react_core.py
---

# Objetivos — Enrutamiento semántico agnóstico (Domain_Entity_*)

## Misión

Erradicar el acoplamiento temprano del bus EDA a entidades concretas (`Tool_Degraded`, `Tool_Deprecated`, `Status_Restored`, etc.) y elevar la física del bus al estado **S+ Grade** bajo la taxonomía universal `Domain_Entity_{Acción}`. La semántica de *qué* entidad mutó vive en el **payload** (`entity_type`, `entity_id`), no en el nombre del evento — recuperando la **Ceguera Espacial** del orquestador.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| `Domain_Entity_Created/Updated/Deleted` ya en suscripciones | Fase A parcialmente cumplida para ciclo CRUD genómico |
| `Tool_Degraded`, `Tool_Deprecated`, `Status_Restored` en `event-domain-subscriptions.json` | Acoplamiento Radamanto/Cerbero a herramientas — **objetivo de purga** |
| Contratos fósiles `tool-degraded.md`, `tool-deprecated.md`, `status-restored.md` | Deben migrar a familia `domain-entity-*` o variantes agnósticas |
| `emit-domain-mutation` traduce solo `create/update/delete` | Ampliar para `deprecated`, `degraded`, `restored` si aplica |
| Radamanto emite `Tool_*` / `Status_Restored` en runtime | Retarget a `Domain_Entity_*` + payload enriquecido |
| PBI exige `entity_type` + `entity_id` en payload | Alinear con ECST existente (`entity_class`, `entity_uuid`, `target_entity_id`) — ver `clarify.md` D2 |

## Objetivos medibles — Fases PBI A–D

| ID | Fase | Objetivo | Criterio |
|----|------|----------|----------|
| **A1** | **A — Suscripciones** | Cero claves `Tool_*` / `Status_*` acopladas en `event-domain-subscriptions.json` | Suscriptores reubicados bajo `Domain_Entity_Degraded`, `Domain_Entity_Deprecated`, `Domain_Entity_Restored` (o equivalente acordado en clarify) |
| **A2** | **A — Fan-out** | Consumidores (Cerbero, Dedalo, Radamanto) filtran por `entity_type` en payload | Tests QA + smokes verdes |
| **B1** | **B — Payload ECST** | Clases `domain-entity-*.md` incluyen `entity_type` y `entity_id` REQUIRED | Aduana ECST valida campos; sin regresión create/update/delete |
| **B2** | **B — Paridad genoma** | Nuevas clases agnósticas para degraded/deprecated/restored si no existen | `SddIA/events/domain/index.md` coherente |
| **C1** | **C — Emisión** | `emit-domain-mutation` y Radamanto depositan instancias con nomenclatura agnóstica en `pending/` | Archivos `{event_id}.json`; payload con routing fields |
| **C2** | **C — Traducción lifecycle** | Tabla lifecycle → `event_type` ampliada (`deprecated`, `degraded`, `restored`) | Documentada en acción + runtime |
| **D1** | **D — Higiene** | Eliminación hard override de contratos fósiles `tool-*` en `SddIA/events/domain/` | Solo supervivencia familia genérica acordada |
| **D2** | **D — EDA coverage** | Entidades nuevas/modificadas en `eda-coverage.json` | Gate `--scan` sin `orphan_count` |

## Alcance inicial (planificación — completado)

1. Rama `feat/adecuar-ed-telemetry` desde `main`.
2. Documentación bajo `persist_ref`: `objectives.md`, `clarify.md`, `spec.md`, `plan.md`.
3. Próximo hito Tekton: ejecución Fases B→F según `plan.md` (T1–T11).

## Fuera de alcance inmediato

- Cambiar taxonomía Trinidad (`telemetry` / `orchestration` / `domain`) — ya estabilizada.
- Migrar eventos de orquestación (`PullRequest_*`, `Suite_*`, `System_*`) — no son mutaciones ED.
- Retirar Radamanto como emisor exclusivo de gobernanza DLT — patrón Fase 4 se preserva; solo generaliza el *tipo* de evento.
- Big-bang del bus histórico en `processed/` — convivencia D0.2.

## Ley aplicada

- Proceso **`feature`** v1.3.0 (`SddIA/process/feature.md`).
- Norma **`features-documentation-pattern`** v1.2.1.
- Contrato **`events-contract`** v1.1.0.
- Cierre documental en rama: PBI → `docs/todos/done/` + `validacion.md` con `pbi_archived: true` en el mismo PR.

## Criterio de éxito (feature completa)

- Matriz de suscripciones sin escuchas rígidas a `tool` en el nombre del evento.
- Toda mutación de estado ED (CRUD + gobernanza Radamanto) emite `Domain_Entity_*` con `entity_type` + `entity_id` en payload.
- Consumidores reaccionan filtrando payload, no ampliando suscripciones por cada nueva ED.
- Genoma `SddIA/events/domain/` sin contratos fósiles acoplados a herramientas.
- Argos APTO; gate EDA genómica sin huérfanos.

## Handoff

Dedalo completado (`spec.md`, `plan.md`). Tekton ejecuta T1–T11 (Fases B, A, C, D, E, F) → Argos → cierre documental en rama.
