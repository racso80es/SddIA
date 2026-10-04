---
document_id: PBI-LINEAR-A-06-REFINE
uuid: "2451a4af-19cf-435d-be13-c570785a5674"
title: "[OPERATIVO] Linear HU-A 06 — Procesos refine-hu y refine-pbi"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 6
hu_order_total: 9
historia_ref: "docs/todos/done/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-02-CREATE-ISSUE
  - PBI-LINEAR-A-03-RBAC-CREATE
  - PBI-LINEAR-A-04-EVENTS
unblocks:
  - PBI-LINEAR-A-07-FORGE-PBI
  - PBI-LINEAR-A-09-E2E
baseline_decisiones:
  - "D-D: refine-hu → HU_Refined; refine-pbi → PBI_Refined"
  - "D-J (a): refine-hu crea el issue de la HU si falta tracker_ref"
  - "L6: las fases Mayeuta/Dédalo de feature no emiten estado"
---

# Procesos `refine-hu` y `refine-pbi`

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **06/09**. F3, F4, D-J, AC-0c. Refinamiento **pre-forja** del markdown de `historias/` y de `todos_pending`. No sustituye «Estabilización de Requisitos» ni «Diseño de Blueprint».

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `refine-pbi` es la fase de plan de `feature` | No. Esas fases escriben la cascada F2 bajo `persist_ref` y no cambian estado en Linear. `refine-pbi` opera sobre el PBI en `todos_pending` antes de abrir la rama de código. |

## 1. Intención

Dos procesos vía `process-creator`. Son el único camino hacia el estado `todo` (la suscripción la cablea el PBI 08).

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `refine-pbi`. `context: [knowledge-management, filesystem-ops]`. Inputs `pbi_ref`, `project_slug`. Mayeuta refina el markdown en `todos_pending` (no toca `done/` ni la cascada F2). Argos sella `status: refinado`, `refined: {fecha}` y emite `PBI_Refined`. |
| R-2 | `refine-hu`. `context: [knowledge-management, filesystem-ops, tracker-operations]`. Inputs `hu_ref`, `project_slug`. |
| R-3 | Fase `tool:linear-tracker-adapter` de `refine-hu`: si el frontmatter no tiene `tracker_ref` y el proyecto tiene tracker, `create_issue` con `labels: [tracker.labels.hu]`, sin `parent_ref`, `state_name: state_map.backlog`, `project_id: tracker.project_id`. Escribe `tracker_ref` en el markdown de `historias/`. Si ya hay `tracker_ref`, no llama a la cápsula. |
| R-4 | Argos sella `status: refinada` y emite `HU_Refined`. |
| R-5 | Sin tracker en el manifiesto: no se crea issue; el refinamiento documental sigue y el evento sale sin `tracker_ref`. Linear caído: fail-soft, `Tracker_Sync_Failed` con `operation: create_issue`, el markdown queda refinado sin `tracker_ref`. |

## 3. Plan

1. Forja de ambos procesos.
2. Tests mock: creación, idempotencia de `tracker_ref`, no-op sin tracker, fallo de Linear.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-0c | `refine-hu` sin `tracker_ref` crea el issue con label `hu` y lo escribe en `historias/`. Con `tracker_ref` no invoca `create_issue`. | Test proceso |
| AC-REF-1 | `refine-pbi` emite `PBI_Refined` con `pbi_ref` y sella `status: refinado`. No modifica ficheros bajo `persist_ref`. | Test proceso |
| AC-REF-2 | Sin tracker, ambos refinan y emiten el evento sin invocar la cápsula. | Test proceso |
