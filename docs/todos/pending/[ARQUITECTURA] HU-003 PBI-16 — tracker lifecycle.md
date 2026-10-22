---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
uuid: "40065401-0b52-4183-8217-2ffdc1f2a0f3"
title: "[ARQUITECTURA] Tracker lifecycle — fractura según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-fracture
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
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-17
files:
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
---

# [ARQUITECTURA] Tracker lifecycle — fractura según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.16.

## 1. Intención

La misma acción materializa la fractura en fichero o en Linear.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Hay que crear otra acción para Linear | `todos-jurisdiction` (PBI 13) mantiene esta acción como canónica. |
| Con `linear` se escribe igual el Markdown y además el issue | Solo el issue. Cero fichero nuevo en `pending/` o `done/`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Provider `file`: el PBI de fractura se crea o reabre en `docs/todos/pending/`, como hoy. Sin `create_issue`. |
| R-2 | Provider `linear`: `create_issue` (label de fix o la que fije `tracker.labels`) y `target_path` no apunta a un `.md` nuevo. La deduplicación vigente (`fracture_hash`) se conserva sobre el issue. |
| R-3 | Los dos ficheros no contienen el literal de la tool; la llamada usa el provider resuelto. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Provider `file`: existe el `.md` en `pending/` y no hay llamada de red. |
| AC-2 | Provider `linear`: no aparece un `.md` nuevo en `pending/` ni `done/`, y hay `issue_ref`. |
| AC-3 | Una segunda fractura con el mismo hash no duplica el sumidero. |

## 5. Fuera de alcance

- Enriquecer el cuerpo (PBI 17).
- Cambiar `System_Fracture_Detected`.

## 6. Ficheros

- `SddIA/actions/materialize-fracture-pbi.md`
- `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs`
