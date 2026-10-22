---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-15
uuid: "38468be1-a064-4cc2-b874-f9ca71c90d30"
title: "[ARQUITECTURA] Tracker lifecycle — regla de cierre del operador"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-closure-rule
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
unblocks: []
files:
  - .cursor/rules/task-closure-documental.mdc
---

# [ARQUITECTURA] Tracker lifecycle — regla de cierre del operador

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.15.

## 1. Intención

La regla que lee el operador dice lo mismo que `features-documentation-pattern` tras el PBI 13.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Basta con enmendar la norma de librería | Esta copia exige hoy el PBI en `done/` para declarar Done. Si no se alinea, el operador lo pedirá también en Linear. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Con provider `file`, Done sigue pidiendo `validacion.md` APTO, `pbi_archived: true` y el PBI en `docs/todos/done/` en la rama. |
| R-2 | Con provider `linear`, Done pide el issue en `state_map.done` y `validacion.md` APTO, y no pide el fichero en `done/`. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | El texto no contiene una definición única que exija el fichero para todo provider. |
| AC-2 | No aparece `done_gate: both` como camino de cierre. |

## 5. Fuera de alcance

- Editar genoma.
- Borrar históricos.

## 6. Ficheros

- `.cursor/rules/task-closure-documental.mdc`
