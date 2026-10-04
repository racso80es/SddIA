---
document_id: PBI-LINEAR-A-07-FORGE-PBI
uuid: "894eb9cf-e40a-417f-8956-ac65899bd6b0"
title: "[OPERATIVO] Linear HU-A 07 — forge-pbi registra el issue"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 7
hu_order_total: 9
historia_ref: "docs/todos/done/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-02-CREATE-ISSUE
  - PBI-LINEAR-A-03-RBAC-CREATE
  - PBI-LINEAR-A-06-REFINE
unblocks:
  - PBI-LINEAR-A-09-E2E
baseline_decisiones:
  - "D-A (b): forge-pbi inyecta tracker_ref al sellar"
  - "L1: hace falta fase tool: y tracker-operations; hoy solo tiene fases agent:"
  - "L2 / D-J: el padre lo crea refine-hu; sin tracker_ref de HU, fail-soft"
---

# `forge-pbi` registra el issue

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **07/09**. F0 (proceso), AC-0b. Depende de 06 porque el `parent_ref` sale del `tracker_ref` que `refine-hu` escribió en la HU.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Argos llama a Linear al sellar | Argos no tiene fase `tool:`. La creación es una fase propia entre Recepción y Sellado; Argos solo escribe el `tracker_ref` ya obtenido. |
| Desaparece toda creación manual | Solo si la HU pasó por `refine-hu` (06). Un PBI cuya HU no tiene `tracker_ref` se sella igual, sin issue. |

## 1. Intención

`forge-pbi` 1.0.0 → **1.1.0**. El PBI nace en Linear como hijo de su HU, con la label de tipo, y el markdown sellado ya lleva `tracker_ref`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `context` gana `tracker-operations`. `feature` / `bug-fix` / `refactorization` no. |
| R-2 | Fase nueva `tool:linear-tracker-adapter` «Registro en tracker», entre Recepción y Sellado. `create_issue` con `labels: [tracker.labels.pbi, tracker.labels.{tipo}]`, `parent_ref` = `tracker_ref` de la HU (`historia_ref`), `state_name: state_map.backlog`, `project_id: tracker.project_id`. |
| R-3 | Mapa de tipo: `process: bug-fix` → `fix`; PBI de mejora → `kaizen`; deuda → `deuda`; spike → `spike`. Sin tipo reconocido → solo `pbi`. |
| R-4 | Argos inyecta el `issue_ref` devuelto como `tracker_ref` del frontmatter sellado. |
| R-5 | Sin tracker en el manifiesto → fase no-op, PBI sin `tracker_ref`. Linear caído o HU sin `tracker_ref` → PBI sellado sin `tracker_ref`, `Tracker_Sync_Failed` con `operation: create_issue`. |
| R-6 | Bump vía `process-creator`. |

## 3. Plan

1. Bump del proceso y handler nativo de la fase (sin LLM en la fase tool).
2. Tests mock de AC-0b y del mapa de labels.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-0b | Con tracker, el PBI sellado lleva el `tracker_ref` del issue creado: labels `pbi` + tipo, `parent` = HU, `project_id` del manifiesto. | Test proceso |
| AC-0b2 | Sin tracker → no-op. Linear caído o HU sin `tracker_ref` → PBI sellado sin `tracker_ref` y `Tracker_Sync_Failed` `operation: create_issue`. | Test proceso |
