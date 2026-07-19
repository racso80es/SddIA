---
feature_name: telemetria-activa-domain-entity-updated
created: "2026-07-17"
process: feature
branch_name: feat/telemetria-activa-domain-entity-updated
persist_ref: docs/features/telemetria-activa-domain-entity-updated
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated.md
document_id: PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED
status: validacion_apto
depends_on:
  - docs/features/telemetria-reactiva-eda-fase4
  - docs/features/adecuar-ed-telemetry
  - docs/features/memoria-vectorial
  - docs/features/boveda-evolucion-epigenetica
laudo: plan_b_domain_entity_telemetry_captured
---

# Objetivos — telemetria-activa-domain-entity-updated

## Misión

Cerrar el circuito de telemetría activa: cada `Raw_Execution_Finished` consumido por Radamanto debe emitir `Domain_Entity_Telemetry_Captured` (Plan B Dedalo) e indexarlo en LanceDB vía `EvolutionProxyService` + `lancedb_evolution_repo`, sin evaporar el rastro ni tocar el CRUD de `Domain_Entity_Updated`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Clase ECST `Domain_Entity_Telemetry_Captured` | Extender/contaminar `Domain_Entity_Updated` (Plan A rechazado) |
| Emisión desde `radamanto-batch` | Sustituir `telemetry-compliance-audit` |
| Proceso `memory-evolution-ingest` + suscripción | Poda `telemetry_batch_stub` (Kaizen aparte) |
| Persistencia evolution store (mínimo durable) | Embeddings obligatorios / UI |
| Smoke + cierre documental single-PR | Anclaje DLT del snapshot (v1) |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Emisión snapshot | AC1 spec — chispa `Domain_Entity_Telemetry_Captured` |
| O2 | Enrutamiento | AC2 — `memory-evolution-ingest`; no sync-index |
| O3 | Ingesta vectorial | AC3 — registro en vector_store/evolution |
| O4 | No regresión CRUD | AC4 |
| O5 | EDA + Done | AC5–AC6 |

## Ley aplicada

- Git vía `skill:git-manager`.
- Genoma solo vía `entity-manager` / creators (DA-2–DA-3).
- Rutas solo vía Cúmulo (`eda_fractal`, `paths.featurePath`).
- Patrón documental `features-documentation-pattern` v1.2.x bajo `persist_ref`.
- Cierre: un único PR; sin segundo PR documental post-merge.

## Handoff

- PBI: v1.1.0+ alineado a Plan B.
- `clarify.md` · `spec.md` · `plan.md`.
- Siguiente: **Tekton** T0 → genoma antes de runtime.
