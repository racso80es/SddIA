---
document_id: PBI-ARQUITECTURA-WORK-INITIATED
uuid: "98d21320-e629-4c8e-a2bd-5406298468cc"
title: "[ARQUITECTURA] Tracker — evento Work_Initiated (D6)"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: work-initiated-event
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
execution_mode: autonomo
especificacion_cerrada: "2026-10-02"
cola_ejecucion: docs/todos/pending/
blocked_by: []
unblocks:
  - PBI-ARQUITECTURA-TRACKER-STAMP
baseline_decisiones:
  - "D6 (a): emisión ciega al iniciar feature / bug-fix / refactorization"
---

# Evento `Work_Initiated`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.D, D6, AC-20.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| El proceso escribe en el bus | Lo hace `action:emit-*-event`. Aquí: `emit-work-initiated-event`, `context: ecosystem-evolution`. |
| Los procesos de desarrollo ganan `tracker-operations` | Prohibido (D6). El emisor no conoce el proveedor. |
| `Delivery_Started` | Nombre laudado: `Work_Initiated`. Definición `work-initiated.md`. |

## 1. Intención

Señal de dominio al cerrar la inicialización de espacio de trabajo. Quien sella Linear es un suscriptor posterior.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-EVT-1 | Evento `Work_Initiated` vía `event-creator`. REQUIRED: `event_id`, `correlation_id`, `source_process` (`feature`\|`bug-fix`\|`refactorization`), `branch`, `persist_ref`, `occurred_at`. OPTIONAL: `project_slug`, `pbi_ref`, `tracker_ref` (opaco). FORBIDDEN: `team_key`, ids de `WorkflowState`, URLs de Linear. |
| R-ACT-1 | Acción `emit-work-initiated-event` vía `entity-manager`. `context: ecosystem-evolution`. Mismo patrón que `emit-pr-presented-event` (broker + filesystem-manager). |
| R-PRC-1 | Fase al cierre de «Inicialización de Espacio de Trabajo» en `feature`, `bug-fix` y `refactorization` vía `entity-manager` / `process-creator`. Fail-soft: fallo de emisión → `warn`, el ciclo sigue. Sin `tracker_ref` el evento se emite igual. |
| R-CTX-1 | El `context` de esos tres procesos no incluye `tracker-operations`. |

## 3. Plan

1. Init `feature` `work-initiated-event`.
2. Forjar evento + acción. Mutar los tres procesos.
3. Tests de emisión (con/sin `tracker_ref`) y de fail-soft.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-20 | Tras init aparece `Work_Initiated` en `eda_bus.pending` sin campos de proveedor; `rg` no encuentra `tracker-operations` en los tres procesos. | Suite + `rg` |
| AC-F6-1 | Emisión fallida no aborta el proceso (`warn`). | Test |

El consumo lo cierra `PBI-ARQUITECTURA-TRACKER-STAMP`.

## 5. Fuera de alcance

- `tracker-stamp`. Ampliar payloads de eventos PR (D7).

## 6. Dependencias

Ninguna. Desbloquea `tracker-stamp`.
