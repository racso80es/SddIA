---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-08
uuid: "4f8dcdff-2977-47e5-bd14-eeb8618f3ef5"
title: "[ARQUITECTURA] Tracker lifecycle — tracker-sync-replay por capacidad"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-replay
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
  - SddIA/process/tracker-sync-replay.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs
---

# [ARQUITECTURA] Tracker lifecycle — tracker-sync-replay por capacidad

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.8.

## 1. Intención

El replay deja de nombrar la tool. Sigue siendo el fail-soft de red, no el validador de schema.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Un fallo de schema entra en `tracker-sync-replay` | El schema roto es `CONTRACT_OUTPUT_SCHEMA_MISMATCH` y dead-letter. El replay sigue siendo `Tracker_Sync_Failed`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Proceso y handler no contienen el literal. Usan el provider resuelto. |
| R-2 | Provider `file`: no hay replay contra Linear. |
| R-3 | Provider `linear`: el orden de replay vigente se conserva. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en los dos ficheros. |
| AC-2 | Un `Tracker_Sync_Failed` con provider `linear` sigue pudiendo reintentarse. |

## 5. Fuera de alcance

- Cambiar la ventana anti-eco.
- Nuevas operaciones.

## 6. Ficheros

- `SddIA/process/tracker-sync-replay.md`
- `SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs`
