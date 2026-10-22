---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-06
uuid: "e7c9b713-2d4e-45ab-9465-344cd12bc46c"
title: "[ARQUITECTURA] Tracker lifecycle — forge-pbi según provider"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-forge-pbi
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
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
---

# [ARQUITECTURA] Tracker lifecycle — forge-pbi según provider

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.6.

## 1. Intención

El alta de PBI escribe en un solo sumidero.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Siempre se crea el Markdown en `pending/` y además el issue | Un sumidero. `file` escribe el fichero. `linear` hace `create_issue` y no escribe `pending/` ni `done/`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Proceso y handler no contienen el literal de la tool. |
| R-2 | Provider `file`: el PBI aparece bajo `docs/todos/pending/` y no hay `create_issue`. |
| R-3 | Provider `linear`: `create_issue` con labels `pbi` + tipo y `parent_ref` de la HU. Cero fichero nuevo en esos buckets. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en los dos ficheros. |
| AC-2 | Los dos sumideros no se materializan en la misma ejecución. |
| AC-3 | Sin `team_key` y con provider `linear`, el manifiesto ya es inválido por el PBI 14; este handler no inventa un team. |

## 5. Fuera de alcance

- Cierre en `done/` (PBI 10).
- Fracturas (PBI 16).

## 6. Ficheros

- `SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md`
- `SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs`
