---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
tracker_ref: OSC-16
uuid: "4b7aae4b-1fc6-477f-be07-d03216258493"
title: "[ARQUITECTURA] Tracker lifecycle — provides y anti-eco del adaptador"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-provider
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/done/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-06
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-07
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-08
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-09
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
files:
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/engine/execute-process/src/engine/tracker_outbound.rs
---

# [ARQUITECTURA] Tracker lifecycle — provides y anti-eco del adaptador

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.2.

## 1. Intención

El adaptador declara la capacidad y el anti-eco deja de llevar el nombre clavado.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Hay que añadir verbos (`update_lifecycle_stage`, etc.) | Las operaciones siguen siendo las seis vigentes. |
| `const TOOL` puede quedar | El registro outbound usa el provider resuelto, no una constante con el nombre de la tool. |
| `update_issue_description` ya está en `RECORDED_OPS` | No está. Si la traza la usa, entra en esa lista. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `linear-tracker-adapter.md` declara `provides` con `id: tracker:lifecycle`, `contract: tracker.lifecycle`, `version: 1.0.0`, misma forma que `telegram-gateway`. |
| R-2 | `tracker_outbound.rs` elimina `const TOOL` y registra según el provider resuelto. |
| R-3 | `update_issue_description` queda en `RECORDED_OPS` junto a `create_issue`, `update_issue_state` y `create_comment`. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | El frontmatter `provides` coincide con la fila del PBI 01. |
| AC-2 | Un `update_issue_description` aceptado escribe `.SddIA/state/tracker-outbound/{issue}.jsonl`. |
| AC-3 | El crate no gana operaciones. |

## 5. Fuera de alcance

- Pasar la cápsula a WASI.
- Cambiar `ENTITY_ID` o el nombre del binario.

## 6. Ficheros

- `SddIA/tools/linear-tracker-adapter.md`
- `SddIA/engine/execute-process/src/engine/tracker_outbound.rs`
