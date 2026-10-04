---
document_id: PBI-LINEAR-A-05-WORK-INITIATED
uuid: "7b3529df-97b5-4310-aa2a-2b2de12e7b5e"
title: "[OPERATIVO] Linear HU-A 05 — Work_Initiated al entrar en Ejecución"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-04"
execution_branch: feat/linear-hu-a-05-work-initiated
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 5
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/done/
unblocks:
  - PBI-LINEAR-A-09-E2E
baseline_decisiones:
  - "D-B (a): in_progress empieza con la forja de código"
  - "L4: la emisión vive en el motor (workspace_init.rs), no en el .md del proceso"
  - "D6 de la HU base queda superado; registro en SddIA/evolution/"
---

# `Work_Initiated` al entrar en Ejecución

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **05/09**. F4b, AC-4c, AC-10. Sin dependencia de Linear: se forja y se valida contra la suite Core-self antes de cablear transiciones nuevas.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Mover la acción dentro de la fase «Ejecución» del `.md` | La fase Ejecución es `delegates_to: agent:tekton`. La emisión la hace el motor al cerrar la Inicialización (`workspace_init.rs`). El cambio es del dispatcher. |

## 1. Intención

`Work_Initiated` se emite al **entrar** en la primera fase con `delegates_to: agent:tekton` de `feature`, `bug-fix` y `refactorization`. Deja de emitirse al cerrar «Inicialización de Espacio de Trabajo». Payload idéntico (`branch`, `persist_ref`, `tracker_ref` opcionales).

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Un ciclo que muere en Estabilización o en Diseño de Blueprint **no** emite `Work_Initiated`. |
| R-2 | Al entrar en Ejecución se emite una sola vez, fail-soft (si `emit-work-initiated-event` falla, el proceso sigue con `warn`, D5). |
| R-3 | Sin `tracker_ref` el evento se emite igual (AC-21 de la HU base). |
| R-4 | Entrada en `SddIA/evolution/` que supera D6 de la HU base, con el uuid del cambio. |
| R-5 | Los `.md` de `feature`, `bug-fix` y `refactorization` no ganan `tracker-operations` ni una fase nueva. |

## 3. Plan

1. Mover la emisión en el dispatcher del motor.
2. Tests: no emisión tras init; emisión al entrar en Ejecución; payload estable; suite Core-self verde.
3. Registro `evolution/`. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-4c | `Work_Initiated` se emite al entrar en Ejecución y no al cerrar Inicialización. Payload idéntico al actual. | Test motor |
| AC-10 | Sin `tracker` el ciclo actual queda idéntico y verde. | Suite Core-self |
