---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-10
uuid: "51268a96-d378-48ee-b558-29711b3b9739"
title: "[ARQUITECTURA] Tracker lifecycle — cierre sin doble sumidero"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-done-gate
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
unblocks: []
files:
  - SddIA/engine/execute-process/src/engine/done_gate.rs
  - SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
---

# [ARQUITECTURA] Tracker lifecycle — cierre sin doble sumidero

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.10.

## 1. Intención

Con `linear` el cierre mira el issue y no escribe backlog. Con `file` el PBI pasa a `done/`.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| `done_gate: linear` sigue exigiendo el Markdown vía `tracker-markdown-apply` | Eso es el comportamiento vigente y este PBI lo retira. Con `linear` el handler no escribe `pending/` ni `done/`. |
| `done_gate: both` se conserva | El PBI 13 lo saca del enum. Aquí no se implementa un tercer cierre. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Provider `file`: `validacion` APTO más el PBI en `docs/todos/done/` en la rama. El archivado de `phase_capsules` mueve desde `pending/`. |
| R-2 | Provider `linear`: el gate comprueba `state_map.done` vía `fetch_issue` con el provider resuelto. No crea ni mueve ficheros de backlog. `pbi_archived` no exige ese path. |
| R-3 | Los tres ficheros, en sus despachos, no contienen el literal de la tool. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Ejecución `linear` de `tracker-markdown-apply`: cero ficheros nuevos bajo `docs/todos/pending` y `docs/todos/done`. |
| AC-2 | Ejecución `file`: el PBI termina en `done/` y no hay `fetch_issue`. |
| AC-3 | `rg linear-tracker-adapter` vacío en los tres ficheros. |

## 5. Fuera de alcance

- Borrar el histórico ya versionado.
- Despertar `refine-hu` desde `Tracker_Issue_Changed`.

## 6. Ficheros

- `SddIA/engine/execute-process/src/engine/done_gate.rs`
- `SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs`
- `SddIA/engine/execute-process/src/engine/phase_capsules.rs`
