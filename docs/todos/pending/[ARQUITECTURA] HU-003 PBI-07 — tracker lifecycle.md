---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-07
uuid: "919e0f36-ce27-446c-855a-d354735c0542"
title: "[ARQUITECTURA] Tracker lifecycle — tracker-backlog-query por capacidad"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-backlog
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
unblocks: []
files:
  - SddIA/process/tracker-backlog-query.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_backlog_query.rs
  - SddIA/process/index.md
---

# [ARQUITECTURA] Tracker lifecycle — tracker-backlog-query por capacidad

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.7.

## 1. Intención

El listado pide la capacidad. La fila del índice cita la capacidad, no la tool.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| El índice puede conservar la frase «vía linear-tracker-adapter» | Esa fila es un despacho documental. Pasa a citar `tracker:lifecycle`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Los tres ficheros no contienen el literal de la tool. |
| R-2 | Provider `linear`: `list_issues` con el filtro de label ya existente. |
| R-3 | Provider `file`: el listado sale de `pending/` y `done/`, sin cápsula. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en los tres ficheros. |
| AC-2 | La fila `tracker-backlog-query` de `process/index.md` nombra `tracker:lifecycle`. |
| AC-3 | Provider `file` no abre red. |

## 5. Fuera de alcance

- Cambiar el contrato de entrada `kind` / `state` del proceso.

## 6. Ficheros

- `SddIA/process/tracker-backlog-query.md`
- `SddIA/engine/execute-process/src/engine/handlers/tracker_backlog_query.rs`
- `SddIA/process/index.md`
