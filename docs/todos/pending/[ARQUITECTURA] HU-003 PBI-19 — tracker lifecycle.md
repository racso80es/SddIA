---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
uuid: "9fff0083-f194-42fa-894d-8224a588b044"
title: "[ARQUITECTURA] Tracker lifecycle — fallo crónico de CI según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-ci-chronic
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
  - SddIA/actions/materialize-ci-chronic-failure-pbi.md
  - SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs
---

# [ARQUITECTURA] Tracker lifecycle — fallo crónico de CI según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.19.

## 1. Intención

El PBI de fallo crónico de CI usa el sumidero del provider.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Con `linear` el PBI de CI se versiona en `pending/` | No. Alta en Linear. Cero fichero nuevo en esos buckets. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Provider `file`: el PBI se escribe en `docs/todos/pending/`. |
| R-2 | Provider `linear`: `create_issue` y ningún `.md` nuevo en `pending/` o `done/`. |
| R-3 | La acción sigue siendo la canónica. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Un solo sumidero por ejecución. |
| AC-2 | Provider `file` no llama a la cápsula. |

## 5. Fuera de alcance

- Cambiar la detección del fallo crónico. Este PBI solo mueve el sumidero de la materialización.

## 6. Ficheros

- `SddIA/actions/materialize-ci-chronic-failure-pbi.md`
- `SddIA/engine/execute-process/src/engine/materialize_ci_chronic_failure_pbi.rs`
