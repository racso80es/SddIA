---
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
tracker_ref: OSC-15
uuid: "1b17e6fa-a8f3-4ce3-91a4-7aff1954a910"
title: "[ARQUITECTURA] Tracker lifecycle — contrato tracker:lifecycle"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-lifecycle-contract
historia_ref: docs/todos/historias/HU-SDDIA-TRACKER-LIFECYCLE-DI-003.md
historia_document_id: HU-SDDIA-TRACKER-LIFECYCLE-DI-003
created: "2026-10-22"
author: tekton
updated: "2026-10-22"
cola_ejecucion: docs/todos/done/
blocked_by: []
unblocks:
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-02
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-05
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-06
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-07
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-08
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-09
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-11
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-12
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-16
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-18
  - PBI-ARQUITECTURA-TRACKER-LIFECYCLE-19
files:
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/core/capability-bindings.md
  - SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json
---

# [ARQUITECTURA] Tracker lifecycle — contrato tracker:lifecycle

Historia: `HU-SDDIA-TRACKER-LIFECYCLE-DI-003` §7.1.

## 1. Intención

Alta de la capacidad `tracker:lifecycle` antes de que ningún proceso la pida.

## 2. Filtro A

| Afirmación | Corrección |
|---|---|
| El id `tracker.lifecycle` (con punto) es válido | El catálogo usa `dominio:verbo`. El id es `tracker:lifecycle`. El contrato es `tracker.lifecycle`. |
| El schema puede copiar `doc.closure` (`exitCode` + `data`) | `validate_output_payload` recibe `cap.body` de `emit_v2` (`success`, `exitCode`, `message`, `result`). |
| La fila de binding dispara Linear siempre | Solo se resuelve cuando el provider efectivo es `linear` (PBI 14). Con `file` no hay `di_binding`. |

## 3. Requisitos

| ID | Requisito |
|---|---|
| R-1 | `capability-taxonomy` gana `id: tracker:lifecycle`, `contract: tracker.lifecycle`, `version: 1.0.0`. |
| R-2 | `capability-bindings.md` gana una sola fila hacia `tool:linear-tracker-adapter`. |
| R-3 | `tracker.lifecycle.schema.json` valida el stdout real de `emit_v2`, con `additionalProperties`. |

## 4. Criterios de aceptación

| ID | Criterio |
|---|---|
| AC-1 | Un `requires_capability: tracker:lifecycle` no falla el filtro A por id ausente. |
| AC-2 | El schema acepta un cuerpo `emit_v2` de éxito y rechaza uno sin `success`/`exitCode`. |
| AC-3 | Sigue habiendo una sola fila por `capability_id`. |

## 5. Fuera de alcance

- Operaciones nuevas en la cápsula.
- Enum `file` del manifiesto (PBI 13 y 14).

## 6. Ficheros

- `SddIA/library/norms/capability-taxonomy.md`
- `SddIA/core/capability-bindings.md`
- `SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json`
