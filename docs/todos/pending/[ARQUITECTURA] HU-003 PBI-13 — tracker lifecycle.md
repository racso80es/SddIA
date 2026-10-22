---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
uuid: "a35fba7b-80f5-43a4-87e5-fabc84bb86ef"
title: "[ARQUITECTURA] Tracker lifecycle — tres normas del sumidero"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-norms
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by: []
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-10
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-15
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
files:
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/library/norms/todos-jurisdiction.md
---

# [ARQUITECTURA] Tracker lifecycle — tres normas del sumidero

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.13.

## 1. Intención

Default `file`. Linear no genera `pending/` ni `done/`. Las acciones de fractura siguen siendo las canónicas y eligen sumidero.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Sin `tracker` el default sigue siendo `done_gate: git` con copia a Markdown | El default pasa a `file`: se generan los ficheros y no hay Linear. |
| `done_gate: both` convive | Sale del enum. Un `done_gate` que contradiga al provider invalida el manifiesto. |
| La prohibición de reimplementar `materialize-fracture-pbi` se mantiene literal | Se sustituye: la acción sigue siendo la única canónica y elige sumidero. No nace una segunda acción. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `project-config-contract` bump. `tracker.provider`: `file` | `linear`. Ausente el objeto o el provider ⇒ `file`. `linear` sigue exigiendo `team_key` y `state_map`. `both` no es valor de `done_gate`. |
| R-2 | `features-documentation-pattern`: `file` exige `pbi_archived: true` y el PBI en `done/` en la rama. `linear` exige el issue en `state_map.done` y prohíbe fichero nuevo en `pending/` y `done/`. La forja en `docs/features` y `docs/fixes` no cambia. `validacion.md` APTO sigue en ambos. |
| R-3 | `todos-jurisdiction`: `pending/` y `done/` son cola solo con `file`. Con `linear` el ancla es `tracker_ref`. La frase de prohibición de reimplementar queda sustituida por el sumidero único de las acciones canónicas. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Un lector del contrato no puede concluir que el default es Linear ni que `both` es legal. |
| AC-2 | La definición de Done partida nombra los dos providers y no pide el fichero cuando es `linear`. |
| AC-3 | La norma de jurisdicción nombra `materialize-fracture-pbi`, `enrich-fracture-pbi-kaizen`, `materialize-kaizen-alert-doc` y `materialize-ci-chronic-failure-pbi` como canónicas con sumidero según provider. |

## 5. Fuera de alcance

- Implementar el parser (PBI 14).
- Borrar ficheros históricos.
- Proveedor distinto de `file` y `linear`.

## 6. Ficheros

- `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md`
- `SddIA/library/norms/features-documentation-pattern.md`
- `SddIA/library/norms/todos-jurisdiction.md`
