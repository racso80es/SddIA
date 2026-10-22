---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
uuid: "a627c8c0-75cb-436f-a691-e51a2a4584e0"
title: "[ARQUITECTURA] Tracker lifecycle — parser file|linear"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-binding
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-13
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-06
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-09
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-10
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-11
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-12
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-20
files:
  - SddIA/engine/execute-process/src/engine/project_binding.rs
---

# [ARQUITECTURA] Tracker lifecycle — parser file|linear

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.14.

## 1. Intención

`project_binding` aplica el contrato del PBI 13.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Ausencia de `tracker` sigue forzando `done_gate: git` | Fuerza provider efectivo `file`. |
| `provider: file` hoy es inválido (`solo linear`) | Pasa a ser el valor de fichero. |
| Un `done_gate: linear` con provider `file`, o `git` con provider `linear`, se ignora | Invalida el manifiesto. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `provider` ∈ {`file`, `linear`}. Otro valor → `PROJECT_CONFIG_INVALID`. |
| R-2 | Sin objeto `tracker` o sin `provider`: efectivo `file`. No exige `team_key`. |
| R-3 | `provider: linear` sin `team_key` → `PROJECT_CONFIG_INVALID`. `done_gate: both` → `PROJECT_CONFIG_INVALID`. `done_gate` contradictorio con el provider → `PROJECT_CONFIG_INVALID`. Coherente (`linear`/`linear`, `file`/`git`) se acepta. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Manifiesto Core-self actual (`provider: linear`, `team_key: OSC`, `done_gate: linear`) sigue pasando. |
| AC-2 | Manifiesto sin `tracker` resuelve `file` y no exige team. |
| AC-3 | `provider: jira`, `done_gate: both` y `provider: file` + `done_gate: linear` fallan. |

## 5. Fuera de alcance

- Escribir los manifest de Paciente 0 (PBI 20).
- Tocar la cápsula.

## 6. Ficheros

- `SddIA/engine/execute-process/src/engine/project_binding.rs`
