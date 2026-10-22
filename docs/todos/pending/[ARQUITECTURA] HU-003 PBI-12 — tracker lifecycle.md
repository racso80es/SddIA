---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-12
uuid: "da93d4c5-a748-4919-a92c-5eb4f015fdf3"
title: "[ARQUITECTURA] Tracker lifecycle — binario del watcher por binding"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-watcher
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
  - SddIA/daemons/linear-bridge-watcher/src/main.rs
---

# [ARQUITECTURA] Tracker lifecycle — binario del watcher por binding

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.12.

## 1. Intención

El watcher resuelve el binario desde el provider del binding y no arranca en `file`.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| Hay que renombrar el crate | El nombre del binario sigue siendo el `name` del tool. Deja de estar escrito como path fijo en el watcher. |
| Con `file` el poll a Linear sigue | No arranca. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `resolve_adapter_bin` construye `SddIA/target/{profile}/{name}` con el `name` de la fila `provider`. |
| R-2 | El fuente del watcher no contiene el literal de la tool. |
| R-3 | Provider `file`: el proceso del watcher sale sin poll y sin webhook. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | `rg linear-tracker-adapter` vacío en `main.rs`. |
| AC-2 | Provider `linear`: localiza el binario release o debug del `name` resuelto. |
| AC-3 | La ventana anti-eco de `linear_bridge.rs` no se modifica. |

## 5. Fuera de alcance

- Cambiar el suscriptor de `Tracker_Issue_Changed` para lanzar forja.
- Reescribir `linear_bridge.rs`.

## 6. Ficheros

- `SddIA/daemons/linear-bridge-watcher/src/main.rs`
