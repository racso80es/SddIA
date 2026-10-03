---
document_id: PBI-LINEAR-A-03-RBAC-CREATE
uuid: "730a4b12-ca2b-493a-b21b-d45c1fd10100"
title: "[OPERATIVO] Linear HU-A 03 — RBAC tracker-operations crear issues"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 3
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-02-CREATE-ISSUE
unblocks:
  - PBI-LINEAR-A-06-REFINE
  - PBI-LINEAR-A-07-FORGE-PBI
baseline_decisiones:
  - "D-A: el contexto gana «crear issues»"
  - "Contrato de partición: webhooks siguen excluidos hasta HU-B (1.4.0)"
  - "L22–L27: feature, bug-fix y refactorization no ganan tracker-operations"
---

# RBAC `tracker-operations` — crear issues

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **03/09**. F0 (RBAC), L3 parcial.

## 1. Intención

`execution-contexts.md` 1.2.0 → **1.3.0**, §2.10. Sin este bump, Cerbero bloquea `create_issue` desde `forge-pbi` y `refine-hu`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Alcance de `tracker-operations` gana «crear issues». Bump vía `entity-manager`. |
| R-2 | Siguen fuera de alcance: borrar issues, administrar equipos o proyectos, **webhooks**. |
| R-3 | `feature`, `bug-fix` y `refactorization` no ganan el contexto. Solo lo ganarán `forge-pbi` (07) y `refine-hu` (06). |
| R-4 | Tekton conserva `tracker-operations` en `allowed_policies` (ya lo tiene desde la HU base). |

## 3. Plan

1. Bump de la norma de contextos.
2. Test de Cerbero: proceso con el contexto puede invocar `create_issue`; proceso sin él, no.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-RBAC-1 | Un proceso con `context: [tracker-operations]` invoca `create_issue` en lab. | Test Cerbero |
| AC-RBAC-2 | `feature` sin ese contexto es rechazado al intentarlo. | Test Cerbero |
| AC-RBAC-3 | El texto de §2.10 sigue excluyendo webhooks y borrado. | Revisión del artefacto |
