---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
uuid: "c62ed63e-28b5-4a6b-b742-90661afcfed8"
title: "[ARQUITECTURA] Tracker lifecycle — traza persist_ref por señal"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-persist-ref
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
unblocks: []
files: []
---

# [ARQUITECTURA] Tracker lifecycle — traza persist_ref por señal

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.4.

## 1. Intención

La forja informa `persist_ref` al tracker por una señal que consume un proceso con `tracker-operations`. El emisor concreto no está localizado: el spec de la feature lo nombra antes de tocar código, con techo de tres ficheros.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Cada fase de `feature` llama a la cápsula | Esos procesos no llevan `tracker-operations`. Cerbero bloquea el cruce. No se les añade el contexto. |
| La fase de forja es un estado Linear | La traza es `create_comment` o `update_issue_description`. El workflow no pasa a `done`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | El spec nombra como máximo tres ficheros: el emisor de la señal y el consumidor (proceso del PBI 03 o hermano con el mismo contexto). |
| R-2 | Con provider `linear`, el issue permanece en `in_progress` o `in_review` y gana `persist_ref`. |
| R-3 | Con provider `file` no hay comentario Linear. El `.md` de forja sigue en `docs/features/` o `docs/fixes/`. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `feature`, `bug-fix` y `refactorization` no ganan `tracker-operations` ni una fase `tool:`. |
| AC-2 | El issue no transiciona a `state_map.done` por el solo hecho de escribir `spec.md` o `validacion.md`. |
| AC-3 | Diff de como máximo tres ficheros de genoma, los que fije el spec. |

## 5. Fuera de alcance

- Inventario de estados `Draft` / `HU Refined` / `PBI Forged`.
- Mover artefactos de forja fuera de `docs/features` y `docs/fixes`.

## 6. Ficheros

- Techo de tres, fijados en el `spec.md` de la feature antes de editar genoma.
