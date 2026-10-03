---
document_id: PBI-LINEAR-A-01-CONTRACT
uuid: "c4a5adba-20c7-4a03-8cac-6e7907cf5518"
title: "[OPERATIVO] Linear HU-A 01 — Contrato de proyecto tracker 1.3.0"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 1
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
unblocks:
  - PBI-LINEAR-A-02-CREATE-ISSUE
  - PBI-LINEAR-A-04-EVENTS
  - PBI-LINEAR-A-08-STAMP
baseline_decisiones:
  - "D-E: sin state_map.todo → warn no-op; prohibido fallback a backlog"
  - "Contrato de partición: done_gate se declara; esta HU solo acepta git"
  - "tracker.labels.editable se declara inerte; la consume HU-B"
---

# Contrato de proyecto tracker 1.3.0

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **01/09**. F1, F2 (declaración), AC-1.

## 1. Intención

Congelar el vocabulario que el resto de la HU consume: estado `todo`, labels extendidas y `done_gate`, sin activar la semántica de Linear como gate.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `project-config-contract` 1.2.0 → **1.3.0** vía `entity-manager`. Único bump del contrato en toda la serie; HU-B no lo repite. |
| R-2 | `tracker.state_map` admite la clave opcional `todo`. Claves previas intactas (`backlog`, `in_progress`, `in_review`, `done`, `cancelled`). |
| R-3 | `tracker.labels` admite `fix`, `kaizen`, `deuda`, `spike`, `editable` (default `sddia-editable`), además de `hu` y `pbi`. |
| R-4 | `tracker.done_gate` ∈ `{git, linear, both}`, default `git`. El validador de esta HU **rechaza** `linear` y `both`. Proyecto sin `tracker` no admite el campo. |
| R-5 | Un manifiesto 1.2.0 sin los campos nuevos sigue validando. |

## 3. Plan

1. Bump del contrato y del validador en el Core.
2. Tests: 1.3.0 válido con `todo` + labels + `done_gate: git`; `done_gate: linear` inválido; 1.2.0 sigue válido.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Manifiesto 1.3.0 con `state_map.todo`, labels extendidas y `done_gate: git` valida. | Test contrato |
| AC-1b | `done_gate: linear` o `both` → error de validación. | Test contrato |
| AC-1c | Manifiesto 1.2.0 valida sin cambios. | Test contrato |
