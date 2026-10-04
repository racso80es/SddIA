---
document_id: PBI-LINEAR-A-09-E2E
uuid: "eaa2f1ce-47d4-4d93-a794-84e06a481a0c"
title: "[OPERATIVO] Linear HU-A 09 — E2E lab ciclo directo"
format: markdown
version: "1.0.0"
status: done
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 9
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-05-WORK-INITIATED
  - PBI-LINEAR-A-06-REFINE
  - PBI-LINEAR-A-07-FORGE-PBI
  - PBI-LINEAR-A-08-STAMP
unblocks:
  - PBI-LINEAR-B-01-OUTBOUND
baseline_decisiones:
  - "AC-7 y AC-8 en verde son la compuerta de HU-LINEAR-SSOT-INVERSE (HU 2 de 2)"
  - "El gate de Done sigue siendo Git (done_gate no sale de git en esta HU)"
---

# E2E lab del ciclo directo

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **09/09**. AC-7, AC-8, AC-9, AC-10. No añade comportamiento: demuestra la cadena 01–08 y desbloquea la HU 2.

## 1. Intención

Suite de laboratorio (mock de Linear, sin red) del ciclo completo en los dos `delivery_mode`, más la regresión Core-self.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `branch_pr`: `refine-hu` → `forge-pbi` → `refine-pbi` → `backlog` → `todo` → `in_progress` (al entrar en Ejecución, no al cerrar init) → `in_review` → `done`. |
| R-2 | Comentarios presentes: forja (`document_id`), refinamiento, rama, `pr_url`, merge SHA. |
| R-3 | La HU recorre `backlog → todo → in_progress → done`, y `done` llega solo cuando todos los hijos están `done`. |
| R-4 | `trunk_direct`: `backlog → todo → in_progress → done` vía `Delivery_Committed`, sin PR. |
| R-5 | Ningún envelope ni evento de la suite contiene `LINEAR_API_TOKEN` ni cuerpo de respuesta de Linear. |
| R-6 | Proyecto sin `tracker`: la suite Core-self queda verde y no invoca la cápsula. |
| R-7 | El PBI de prueba termina en `docs/todos/done/` con `validacion.md` APTO dentro del mismo PR. `done_gate` permanece `git`. |

## 3. Plan

1. Suite lab sobre los procesos ya forjados. Sin producción de genoma nueva salvo el arnés de test.
2. Ejecutar AC-7, AC-8, AC-9, AC-10.
3. Cierre documental en la misma rama. Al mergear, `PBI-LINEAR-B-01-OUTBOUND` deja de estar bloqueado por este PBI.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-7 | Ciclo `feature` + `branch_pr` completo, HU incluida, con los cinco comentarios. | Suite lab |
| AC-8 | Ciclo `trunk_direct`: `backlog → todo → in_progress → done`. | Suite lab |
| AC-9 | Sin secretos ni cuerpo de Linear en envelopes y eventos. | Suite lab |
| AC-10 | Sin tracker, ciclo Core-self idéntico y verde. | Suite Core-self |
