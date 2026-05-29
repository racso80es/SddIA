---
feature_name: telemetria-reactiva-eda-fase1
created: "2026-05-27"
purpose: Decisiones Fase 1 y herencia del gate Fase 0
---

# Clarificación — Fase 1

## Precondición (gate Fase 0)

Fase 0 cerró con `impact-analysis.md` y PBI v1.1.0 refinado. AC0.1–AC0.5 cumplidos. No se reabre análisis salvo hallazgo bloqueante durante Tekton.

## Decisiones heredadas (aplican en Fase 1)

| ID | Resolución | Uso en Fase 1 |
|----|------------|---------------|
| D0.2 | Coexistencia V3+ `pending/` + bus fractal futuro | Migración **genoma** sin retirar `eda_bus` del SSOT; no crear rutas runtime aún |
| D0.5 | Telemetría solo CLI | `Raw_Execution_Finished`: `event_family: telemetry`; emisores = procesos/cápsulas CLI indexados |
| D0.6 | PBI maestro en `pending/` | `validacion.md` con `pbi_archived: false` |

## Decisiones cerradas — Fase 1

| ID | Pregunta | Resolución |
|----|----------|------------|
| D1.1 | ¿Nombre del proceso de forja? | SSOT: `event-creator` (`SddIA/process/event-creator.md`); alias humano «create-event» en PBI |
| D1.2 | ¿Índice raíz `SddIA/events/index.md`? | Pasa a **índice de familias** (enlaces a `telemetry/index.md`, `orchestration/index.md`, `domain/index.md`); catálogo ECST vive en índice de cada familia |
| D1.3 | ¿Códice por familia? | `index.md` obligatorio en cada subcarpeta; **prohibido** `README.md` duplicado (PBI §1.A) |
| D1.4 | ¿Versión de `events-contract`? | Bump **minor** (`1.1.0`) al introducir `event_family`; Clases migradas actualizan cabecera `contract: events-contract v1.1.0` |
| D1.5 | ¿`orchestration/` vacía al cierre? | Sí — carpeta + `index.md` con jurisdicción; sin Clases ECST hasta Fase 3 (salvo decisión explícita) |
| D1.6 | ¿Referencias en `event-subscriptions.json`? | **Fuera de alcance** Fase 1 — Fase 3.C; rutas de `event_type` no cambian en suscripciones |
| D1.7 | ¿EDA coverage / `emit-domain-mutation`? | Tras mover Clases: `entity-manager` o backfill `--backfill-coverage` si paths de artefacto cambian en SSOT |
| D1.8 | ¿Argos y familia? | Validación documental: Clase sin `event_family` en cabecera → `NO_APTO` en auditoría de genoma (norma en contrato § auditoría) |
| D1.9 | ¿`event_family` obligatorio en runtime del proceso? | **Cerrado (Kaizen 2026-05-29):** input obligatorio; sin fallback. Ver `docs/features/kaizen-event-creator-event-family-explicit/` y `event-creator` v1.2.0 |

## Payload `Raw_Execution_Finished` (§1.D)

| Campo | Estatus |
|-------|---------|
| `asset_id` | REQUIRED |
| `exit_code` | REQUIRED |
| `duration_ms` | REQUIRED |
| `process_name` | REQUIRED |
| `telemetry_receipt` | OPTIONAL (Fase 5) |

`event_context`: contexto de infraestructura/orquestación acorde `execution-contexts.md` (validar con Cerbero en forja).

## Referencias

- Gate: `docs/features/telemetria-reactiva-eda-fase0/impact-analysis.md` (H01–H03, H21, H23)
- PBI: `docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md` § Fase 1
- Kaizen D1.9 cerrado: `docs/todos/done/[Kaizen] event-creator — eliminar default event_family domain.md`
