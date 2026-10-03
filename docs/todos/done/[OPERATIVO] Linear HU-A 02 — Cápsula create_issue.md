---
document_id: PBI-LINEAR-A-02-CREATE-ISSUE
uuid: "101efc29-0ebf-4549-9ae6-00dc84758e0a"
title: "[OPERATIVO] Linear HU-A 02 — Cápsula create_issue"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 2
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-01-CONTRACT
unblocks:
  - PBI-LINEAR-A-03-RBAC-CREATE
  - PBI-LINEAR-A-06-REFINE
  - PBI-LINEAR-A-07-FORGE-PBI
baseline_decisiones:
  - "D-A (b): operación create_issue; la cápsula sigue ciega"
  - "L13 de la HU base: la label se asigna al crear, no al leer"
---

# Cápsula `create_issue`

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **02/09**. F0 (cápsula), AC-0a. Depende de 01 para congelar los nombres de label que los tests usan.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| La cápsula decide tipo HU/PBI | No. Recibe `labels[]` ya resueltas por el proceso. |
| Devuelve un identificador con formato `{alias}-{item}` | Linear asigna `TEAMKEY-n`. La cápsula lo devuelve; no lo compone. |

## 1. Intención

`linear-tracker-adapter` 1.0.0 → **1.1.0** gana `create_issue`. Resuelve ids de GraphQL; no decide transiciones ni redacta negocio.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Request: `team_key`, `title`, `description`, `labels[]` (nombres), opcionales `parent_ref`, `project_id`, `state_name`, `priority`. |
| R-2 | Lookup: `team_key → teamId`, label → `labelIds`, `parent_ref → parentId`, `state_name → stateId` (misma resolución que `update_issue_state`). |
| R-3 | Respuesta: `issue_ref` (`identifier`), `id`, `url`. |
| R-4 | Errores nuevos: `LINEAR_LABEL_UNKNOWN`, `LINEAR_PARENT_NOT_FOUND`. Sin mutación si el lookup falla. Resto de códigos (`LINEAR_AUTH_MISSING`, transporte, GraphQL) intactos. |
| R-5 | Lab mock (`SDDIA_LAB_MOCK_OUTBOUND`) cubre la operación. El envelope no contiene `LINEAR_API_TOKEN`. |
| R-6 | Bump del artefacto `SddIA/tools/linear-tracker-adapter.md` vía `entity-manager`, con `hash_signature` regenerado. |

## 3. Plan

1. Forja de la operación y tests de cápsula (mock inline + HTTP).
2. Hash del artefacto.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-0a | `create_issue` con labels, `parent_ref` y `state_name` devuelve `issue_ref`. | Test cápsula |
| AC-0a2 | Label desconocida → `LINEAR_LABEL_UNKNOWN` sin mutación. `parent_ref` inexistente → `LINEAR_PARENT_NOT_FOUND`. Sin token → `LINEAR_AUTH_MISSING`. | Test cápsula |
| AC-9 | El resultado no contiene `LINEAR_API_TOKEN`. | Test cápsula |
