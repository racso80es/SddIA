---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-17
uuid: "7571be3c-2a34-4403-8ae6-4da29999fcd6"
title: "[ARQUITECTURA] Tracker lifecycle — enriquecido de fractura según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-enrich
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
unblocks: []
files:
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
---

# [ARQUITECTURA] Tracker lifecycle — enriquecido de fractura según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.17.

## 1. Intención

El enriquecido edita el mismo sumidero que materializó la fractura.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Con `linear` se enriquece un Markdown que no existe | Se actualiza el issue (`update_issue_description` o `create_comment`). No se crea el fichero. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Provider `file`: se edita el `.md` de `pending/` ya materializado. |
| R-2 | Provider `linear`: se edita el issue referido y no se crea `.md` en `pending/` ni `done/`. |
| R-3 | No nace una acción paralela. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | El sumidero editado es uno solo. |
| AC-2 | Provider `linear` sin `tracker_ref` no inventa un path bajo `docs/todos/`. |

## 5. Fuera de alcance

- Cambiar el agente titular (Mayeuta / Cúmulo) del protocolo de fractura.

## 6. Ficheros

- `SddIA/actions/enrich-fracture-pbi-kaizen.md`
- `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs`
