---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
uuid: "c5c6aad7-bdb1-4f58-b80b-577bdab49983"
title: "[ARQUITECTURA] Tracker lifecycle — refine-hu según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-refine-hu
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
unblocks: []
files:
  - SddIA/process/refine-hu.md
  - SddIA/engine/execute-process/src/engine/handlers/refine.rs
---

# [ARQUITECTURA] Tracker lifecycle — refine-hu según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.5.

## 1. Intención

El alta de HU deja de nombrar la tool. `historias/` se sigue refinando. `pending/` y `done/` no reciben un fichero nuevo.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Sin `tracker` la fase es no-op | El provider efectivo es `file`. No se llama a la cápsula. El markdown de la HU sigue en `docs/todos/historias/`. |
| `refine-hu` escribe el backlog en `pending/` | El proceso refina `historias/`. Ese bucket no es la cola. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Proceso y handler no contienen el literal de la tool. La llamada, si existe, usa el provider resuelto. |
| R-2 | Provider `linear`: `create_issue` con la label `tracker.labels.hu` si falta `tracker_ref`. Cero ficheros nuevos en `pending/` o `done/`. |
| R-3 | Provider `file`: no hay `create_issue`. El refinado de `historias/` se mantiene. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en los dos ficheros. |
| AC-2 | Manifiesto sin `tracker`: no hay invocación de cápsula y el fichero de `historias/` se actualiza. |
| AC-3 | Provider `linear`: el issue existe y `pending/` no gana un `.md` de esta ejecución. |

## 5. Fuera de alcance

- Vaciar `docs/todos/historias/`.
- Cambiar labels distintas de las del contrato.

## 6. Ficheros

- `SddIA/process/refine-hu.md`
- `SddIA/engine/execute-process/src/engine/handlers/refine.rs`
