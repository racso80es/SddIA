---
document_id: PBI-KAIZEN-EVENT-CREATOR-EVENT-FAMILY-EXPLICIT
title: "[Kaizen] event-creator — eliminar default event_family domain"
format: markdown
version: "1.0.0"
created: "2026-05-27"
status: cerrado
priority: media
process: refactorization
persist_ref: docs/features/kaizen-event-creator-event-family-explicit
branch_name: feat/refactorization-kaizen-event-creator-event-family-explicit
related:
  - docs/features/kaizen-event-creator-event-family-explicit/
  - SddIA/process/event-creator.md
  - docs/features/telemetria-reactiva-eda-fase1/
  - docs/todos/done/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
introduced_by: docs/features/telemetria-reactiva-eda-fase1/clarify.md (D1.9)
closed_by: docs/features/kaizen-event-creator-event-family-explicit/validacion.md
---

# [Kaizen] event-creator — eliminar default `event_family: domain`

## 0. Mandato

Refactorizar `event-creator` (`SddIA/process/event-creator.md` y runtime que resuelva `process_inputs`) para **eliminar el fallback** `event_family → "domain"` introducido en Fase 1 (decisión **D1.9**).

Iniciar como **`refactorization`** cuando el ecosistema haya migrado invocaciones explícitas.

## 1. Contexto

Durante `telemetria-reactiva-eda-fase1` (Paso **1.C**), `event_family` se añade con **valor por defecto obligatorio `domain`** si el input está ausente o vacío. Esto preserva retrocompatibilidad absoluta: procesos legacy sin el campo siguen forjando en `SddIA/events/domain/` como hoy.

La telemetría nueva (`Raw_Execution_Finished`, Peaje Termodinámico Fase 3) debe pasar explícitamente `"event_family": "telemetry"`.

## 2. Objetivo

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | **Input explícito** | `event_family` requerido en contrato del proceso; ausencia → error de validación (fase Arquitectura) |
| **O2** | **Inventario migrado** | `rg`/índice: cero invocaciones de `event-creator` / `entity-manager` → event sin `event_family` en payloads documentados |
| **O3** | **Documentación** | PBI Telemetría §1.C y `clarify.md` Fase 1 actualizados; nota de deprecación del default retirada |
| **O4** | **Regresión** | Tests QA de forja de eventos y flujos `create-event` verdes |

## 3. Alcance

- Contrato `event-creator.md`: quitar `default: domain` y marcar input como obligatorio.
- Scripts/cápsulas que normalicen `process_inputs` (p. ej. `execute_process_capsules`, handlers lab).
- Actualizar specs de procesos que deleguen en `event-creator` vía `entity-manager`.

## 4. Fuera de alcance

- Cambiar taxonomía Trinidad ni rutas runtime (ya definidas en fases 1–3 del PBI maestro).
- Forzar `event_family` en envelope de **instancia** ECST (deuda Fase 3).

## 5. Disparador sugerido

Ejecutar tras merge de Fase 3 (Aduana Universal) o cuando el barrido muestre que todos los emisores activos declaran familia explícita.

## 6. Referencias

- Decisión origen: `docs/features/telemetria-reactiva-eda-fase1/clarify.md` — **D1.9**
- Especificación: `docs/features/telemetria-reactiva-eda-fase1/spec.md` — §6.1
- Cierre: `docs/features/kaizen-event-creator-event-family-explicit/validacion.md`
