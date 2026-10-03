---
document_id: PBI-LINEAR-B-06-E2E
uuid: "f8080016-dabb-4227-90f6-4d7d799175b4"
title: "[ARQUITECTURA] Linear HU-B 06 — E2E Core-self y activación del gate"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 6
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-B-03-WEBHOOK
  - PBI-LINEAR-B-05-DONE-GATE
baseline_decisiones:
  - "D-I (a): el primer ciclo real es solo Core-self"
  - "F8.3: done_gate linear exige laudo del Vértice después del ciclo both limpio"
---

# E2E Core-self y activación del gate

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **06/06**. F8.3, AC-21. Cierra la serie. No amplia alcance: ejecuta el orden de activación y deja `done_gate: linear` detrás de un laudo.

## 1. Intención

Demostrar en laboratorio el circuito inverso completo con `done_gate: both` sobre Core-self, y dejar escrito el criterio con el que el Vértice puede pasar ese único proyecto a `linear`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Orden F8.3 dentro de este PBI: (1) HU-A ya mergeada, heredado del `blocked_by`; (2) F7 operativo con `done_gate: git`, cubierto por 02–04; (3) un ciclo Core-self en `both`. |
| R-2 | El ciclo de laboratorio: merge → `tracker-stamp` sella `done` → el watcher emite `Tracker_Issue_Changed` → `tracker-markdown-apply` archiva el PBI en `todos_done` en `tracker-sync/…`. El PR de código no mueve el PBI. |
| R-3 | Anti-eco activo: el sello de `tracker-stamp` no genera un segundo apply. |
| R-4 | El ciclo se considera limpio si no aparece `Tracker_Sync_Failed` con `error_code: TRACKER_STATE_DIVERGED`. |
| R-5 | `done_gate: linear` no se escribe en ningún manifiesto aquí. El PBI documenta la petición de laudo y se cierra con `validacion.md` APTO del ciclo `both`. El paso a `linear` queda como laudo posterior, fuera de este PBI. |
| R-6 | El ciclo de laboratorio usa mock de Linear. No abre endpoint público ni túnel en CI. |

## 3. Plan

1. Suite lab del circuito R-2, incluida la rama `tracker-sync/`.
2. Assert de R-3 y R-4.
3. Cierre documental en la misma rama. La nota de laudo queda en `validacion.md`, no como cambio de manifiesto.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-21 | Merge → `done` en Linear → `Tracker_Issue_Changed` → PBI archivado por `tracker-markdown-apply`, sin movimiento manual en el PR de código. | Suite lab |
| AC-21b | El sello saliente no dispara un apply de vuelta. | Suite lab |
| AC-21c | Cero `Tracker_Sync_Failed` con `TRACKER_STATE_DIVERGED` en el ciclo. | Suite lab |
| AC-21d | Ningún manifiesto queda con `done_gate: linear` al cerrar el PBI. | Revisión del diff |
