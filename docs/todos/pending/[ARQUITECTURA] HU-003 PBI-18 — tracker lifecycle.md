---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
uuid: "00fd5940-9aab-4c45-918b-e5c845c3e171"
title: "[ARQUITECTURA] Tracker lifecycle — alerta Kaizen según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-kaizen
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
unblocks: []
files:
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs
---

# [ARQUITECTURA] Tracker lifecycle — alerta Kaizen según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.18.

## 1. Intención

La alerta Kaizen usa el sumidero del provider.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| La plantilla Markdown se escribe siempre en `pending/` | Solo con `file`. Con `linear`, `create_issue` y label `kaizen`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Provider `file`: el documento sigue apareciendo en `docs/todos/pending/`. |
| R-2 | Provider `linear`: issue con label `tracker.labels.kaizen` y cero fichero nuevo en `pending/` o `done/`. |
| R-3 | La acción sigue siendo la canónica. No hay una segunda acción. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Los dos providers no materializan a la vez. |
| AC-2 | El literal de la tool no queda en estos ficheros si hay despacho: usan el provider resuelto. |

## 5. Fuera de alcance

- Cambiar `alert_kind` o las plantillas de checklist más allá del sumidero.

## 6. Ficheros

- `SddIA/actions/materialize-kaizen-alert-doc.md`
- `SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs`
