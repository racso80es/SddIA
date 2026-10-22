---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
uuid: "85caf44d-e5d0-4c30-8303-bcb825610f92"
title: "[ARQUITECTURA] Tracker lifecycle — tracker-stamp por capacidad"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-stamp
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/done/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
files:
  - SddIA/process/tracker-stamp.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
---

# [ARQUITECTURA] Tracker lifecycle — tracker-stamp por capacidad

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.3.

## 1. Intención

`tracker-stamp` pide `tracker:lifecycle` y el handler deja de nombrar la tool.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| La fase puede seguir en `delegates_to: tool:linear-tracker-adapter` | Pasa a `requires_capability: tracker:lifecycle`. El nombre sale del binding. |
| Con provider `file` el sello llama a Linear | No llama. El proceso no hace mutación Linear. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `tracker-stamp.md` declara `requires_capability: tracker:lifecycle` y no contiene el literal de la tool. |
| R-2 | `tracker_stamp.rs` invoca el provider resuelto. |
| R-3 | Provider efectivo `file`: la fase no invoca la cápsula. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` en los dos ficheros devuelve vacío. |
| AC-2 | Con provider `linear`, un sello de estado usa una operación vigente y el stdout pasa el schema del PBI 01. |
| AC-3 | El contexto del proceso sigue siendo `tracker-operations`. |

## 5. Fuera de alcance

- Abrir `tracker-operations` en `feature`, `bug-fix` o `refactorization`.

## 6. Ficheros

- `SddIA/process/tracker-stamp.md`
- `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs`
