---
feature_name: telemetria-reactiva-eda-fase1
created: "2026-05-27"
process: feature
branch_name: feat/telemetria-reactiva-eda-fase1
persist_ref: docs/features/telemetria-reactiva-eda-fase1
master_pbi_ref: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
master_pbi_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
phase: 1
pbi_archived_at_close: false
status: planificacion
depends_on:
  - docs/features/telemetria-reactiva-eda-fase0
gate_ref: docs/features/telemetria-reactiva-eda-fase0/impact-analysis.md
---

# Objetivos — Telemetría Reactiva EDA · Fase 1 (Trinidad de Estímulos)

## Misión

Ejecutar la **Fase 1** del PBI maestro `PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO` como **feature independiente**: aplicar **Simetría Fractal** al genoma de eventos (`SddIA/events/`), erradicar la topología plana, blindar `events-contract.md` con `event_family` obligatorio, alinear `create-event` (`event-creator`) y forjar la clase **`Raw_Execution_Finished`** en `telemetry/` como pre-requisito del Peaje Termodinámico (Fase 3).

El PBI unificado permanece en `docs/todos/pending/` como plan de ruta. Esta feature **no** archiva el PBI maestro al cerrar (`pbi_archived: false` en `validacion.md`).

## Relación con el programa multi-fase

| Fase PBI | Feature | Estado |
|----------|---------|--------|
| 0 | `telemetria-reactiva-eda-fase0` | Gate cumplido — `impact-analysis.md` AC0.x |
| **1** | **`telemetria-reactiva-eda-fase1` (esta)** | Genoma fractal + `event_family` + `Raw_Execution_Finished` |
| 2 | `telemetria-reactiva-eda-fase2` (futura) | Workspaces dinámicos |
| 3–6 | features independientes | Según PBI § Fases 3–6 |

## Contexto heredado (Fase 0)

| Decisión | Implicación Fase 1 |
|----------|-------------------|
| **D0.2** Coexistencia V3+ + bus fractal | Migrar **solo genoma**; instancias legacy siguen en `eda_bus.pending` hasta Fase 3 |
| **D0.5** Telemetría solo CLI | `Raw_Execution_Finished`: familia `telemetry`, emisor autorizado CLI |
| **H01–H03** Genoma plano, sin `event_family`, `create-event` sin familia | Objetivos directos de esta feature |

## Objetivos medibles (Fase 1)

| ID | Objetivo | Criterio (AC PBI) |
|----|----------|-------------------|
| **F1-O1** | **Topología fractal del genoma** | Subcarpetas `telemetry/`, `orchestration/`, `domain/` + `index.md` por familia; raíz solo `events-contract.md` | AC1.1 |
| **F1-O2** | **Contrato `event_family`** | `events-contract.md` exige enum `{ telemetry, orchestration, domain }`; Argos rechaza Clases sin familia | AC1.3 |
| **F1-O3** | **`create-event` enrutado** | `event_family` con fallback **`domain`** (retrocompat); telemetría explícita `telemetry`; Kaizen para eliminar default | AC1.4 |
| **F1-O4** | **Migración 7 ECST** | Clases actuales en `domain/`; índices sincronizados | AC1.1, AC1.2 |
| **F1-O5** | **`Raw_Execution_Finished`** | Clase ECST en `SddIA/events/telemetry/` vía flujo `create-event` | PBI §1.D (pre-requisito Fase 3) |
| **F1-O6** | **Regresión genoma** | `test_eda_bus_v3plus.py` y plantilla `eda-instance-events` alineados sin romper Vía C | AC implícito §1.E |

## Taxonomía normativa (SSOT)

| Familia | Emisor autorizado | Destino runtime (futuro Fase 3) |
|---------|-------------------|-------------------------------|
| `telemetry` | Solo CLI | `./.events/telemetry/` |
| `orchestration` | CLI (éxito) o agentes auditores | `./.events/orchestration/` |
| `domain` | Agentes Core (Cúmulo, Cerbero, Radamanto) | `./.events/domain/` |

> **Regla de oro:** prohibido mezclar telemetría cruda con orquestación u ontología en la misma ruta de consumo.

## No objetivos (esta feature)

- Implementar rutas runtime `./.events/{telemetry,orchestration,domain}/` ni enrutadores (Fase 3).
- Peaje Termodinámico en CLI ni emisión real de instancias telemetría (Fase 3).
- Workspaces dinámicos (`workspace_template`) (Fase 2).
- Radamanto, split de suscripciones, actualización `README.md` raíz (Fases 4–6).
- Mover el PBI maestro a `docs/todos/done/`.

## Ley aplicada

- `features-documentation-pattern` v1.2.1
- Proceso `feature` v1.3.0
- PBI maestro § Fase 1; gate Fase 0: `impact-analysis.md`, `clarify.md` (D0.x)

## Artefactos previstos

| Artefacto | Estado |
|-----------|--------|
| `objectives.md` | ✅ Este documento |
| `clarify.md` | ✅ |
| `spec.md` | ✅ |
| `plan.md` | ✅ |
| `implementation.md` / `execution.md` | Pendiente (Tekton) |
| `validacion.md` | Pendiente (Argos); `pbi_archived: false` |

## Estado del proceso feature

| Fase proceso | Estado |
|--------------|--------|
| Inicialización (`workspace-init` / rama) | Pendiente operador |
| Estabilización (Mayeuta) | ✅ `objectives.md` + `clarify.md` |
| Diseño (Dedalo) | ✅ `spec.md` + `plan.md` |
| Ejecución (Tekton) | ⏸ Detenido — alcance usuario: solo planificación |
| Verificación / cierre | Pendiente |
