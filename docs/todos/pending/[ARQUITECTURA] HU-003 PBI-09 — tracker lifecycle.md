---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-09
uuid: "99ac13f0-ebaf-4aa2-9fa9-6631e8c050ba"
title: "[ARQUITECTURA] Tracker lifecycle — tracker-linear-markdown-sync por capacidad"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-md-sync
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
  - SddIA/process/tracker-linear-markdown-sync.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_linear_markdown_sync.rs
---

# [ARQUITECTURA] Tracker lifecycle — tracker-linear-markdown-sync por capacidad

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.9.

## 1. Intención

El volcado de cuerpo hacia el issue deja de nombrar la tool y no fabrica ficheros de backlog.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Este proceso crea `pending/` o `done/` | No. Escribe la descripción del issue. Con provider `file` no corre. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Proceso y handler no contienen el literal. |
| R-2 | Provider `linear`: la operación de descripción usa el provider resuelto y no crea ficheros en `pending/` ni `done/`. |
| R-3 | Provider `file`: fase no-op. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en los dos ficheros. |
| AC-2 | Tras una ejecución `linear`, el árbol de `docs/todos/pending` y `done` no gana ficheros de este proceso. |

## 5. Fuera de alcance

- Réplica inversa Linear → markdown (PBI 10).

## 6. Ficheros

- `SddIA/process/tracker-linear-markdown-sync.md`
- `SddIA/engine/execute-process/src/engine/handlers/tracker_linear_markdown_sync.rs`
