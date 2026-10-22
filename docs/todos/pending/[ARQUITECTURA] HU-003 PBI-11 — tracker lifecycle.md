---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-11
uuid: "398d6479-cf4e-4494-8761-461e10fe4aad"
title: "[ARQUITECTURA] Tracker lifecycle — peaje por provider resuelto"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-rbac-gate
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-14
unblocks: []
files:
  - SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/norms/execution-contexts.md
---

# [ARQUITECTURA] Tracker lifecycle — peaje por provider resuelto

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.11.

## 1. Intención

El peaje compara contra el provider del binding, no contra una constante. Con `file` no corre.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| `LINEAR_TRACKER_TOOL` puede quedar como atajo | Es el literal que este corte retira de `capsules.rs`. |
| Hay que borrar el nombre del ejemplo en `execution-contexts.md` | §2.10 nombra la capacidad y conserva el proveedor vigente como ejemplo. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | Desaparece `LINEAR_TRACKER_TOOL`. `capsules.rs` aplica el gate cuando `tool_name` es el provider resuelto de `tracker:lifecycle`. |
| R-2 | Provider `file`: `invoke_tool_for_process` no entra en ese gate. |
| R-3 | `execution-contexts.md` §2.10 cita `tracker:lifecycle` y mantiene el ejemplo del adaptador. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg LINEAR_TRACKER_TOOL` vacío. |
| AC-2 | Un `create_issue` con provider `linear` sigue exigiendo contexto `tracker-operations`, como hoy para esa operación. |
| AC-3 | En `SddIA/norms/` el literal solo queda en ese ejemplo. |

## 5. Fuera de alcance

- Ampliar `tracker-operations` a los procesos de forja.

## 6. Ficheros

- `SddIA/engine/execute-process/src/engine/tracker_operations_gate.rs`
- `SddIA/engine/execute-process/src/engine/capsules.rs`
- `SddIA/norms/execution-contexts.md`
