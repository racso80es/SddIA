---
document_id: PBI-LINEAR-A-08-STAMP
uuid: "e43eb7a6-77cb-4000-b7c7-fa9e8a1e0c6b"
title: "[OPERATIVO] Linear HU-A 08 — Sello todo, cancelled y trunk_direct"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 8
hu_order_total: 9
historia_ref: "docs/todos/done/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-01-CONTRACT
  - PBI-LINEAR-A-04-EVENTS
unblocks:
  - PBI-LINEAR-A-09-E2E
baseline_decisiones:
  - "D-C (a) + L5: la HU promociona desde backlog o todo"
  - "D-D: solo HU_Refined y PBI_Refined mueven a todo"
  - "D-E: state_map sin todo → warn no-op, sin fallback"
---

# Sello `todo`, `cancelled` y `trunk_direct`

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **08/09**. F1 (runtime), F4, F5, F6. Extiende `tracker-stamp` y `tracker-sync-replay`; no crea suscriptores nuevos.

## 1. Intención

El sello existente aprende los estados y eventos de esta HU. La decisión sigue en el proceso; la cápsula solo ejecuta `update_issue_state`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Suscripción a `hu-refined`, `pbi-refined`, `pbi-cancelled`, `delivery-committed`. |
| R-2 | `HU_Refined` / `PBI_Refined` → estado canónico `todo` + comentario (`document_id`, versión). Sin `tracker_ref`, sin tracker, o sin clave `todo` en `state_map` → `warn` no-op, sin invocar la cápsula y sin caer a `backlog`. |
| R-3 | `PBI_Forged`: la transición a `backlog` es idempotente (el issue ya nace ahí desde el PBI 07). Se conserva el comentario con `document_id`. |
| R-4 | `Work_Initiated` del primer hijo: la HU pasa a `in_progress` si está en `backlog` **o** en `todo`. Si ya está en `in_progress`, no hay llamada de transición. |
| R-5 | `PBI_Cancelled` → PBI a `cancelled` + comentario `reason`. La HU no cambia. `maybe_complete_hu_when_children_done` trata `cancelled` como cerrado: una HU con todos los hijos en `done` o `cancelled` y al menos uno `done` pasa a `done`. |
| R-6 | `delivery-close-cycle` copia `tracker_ref`, `pbi_ref`, `persist_ref` y `commit_sha` al emitir `Delivery_Committed` (mecánica D7). `tracker-stamp` mueve el PBI a `done` y evalúa la HU igual que en `PullRequest_Merged`. |
| R-7 | `tracker-sync-replay` ordena `backlog < todo < in_progress < in_review < done`. `cancelled` es terminal: una transición posterior se descarta. |
| R-8 | El markdown de un PBI cancelado se mueve a `todos_done` con `status: cancelado`. |

## 3. Plan

1. Bump de `tracker-stamp`, `tracker-sync-replay` y del emisor en `delivery-close-cycle` vía `process-creator` / `entity-manager`.
2. Suscripciones vía el mecanismo ya usado por `tracker-stamp`.
3. Tests mock AC-2, AC-2b, AC-3, AC-4, AC-4b, AC-5, AC-6.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-2 | `PBI_Refined` con `tracker_ref` → `todo` + comentario. Sin `tracker_ref` o sin tracker → no-op sin cápsula. Sin `todo` en `state_map` → `warn` no-op. | Test stamp |
| AC-2b | `HU_Refined` → HU a `todo`. | Test stamp |
| AC-3 | `Delivery_Committed` 1.1.0 → PBI `done` + comentario `commit_sha`; HU `done` si todos los hijos están `done`. | Test stamp |
| AC-4 | `PBI_Cancelled` → PBI `cancelled`. HU con un hijo `cancelled` y el resto `done` → `done`. | Test stamp |
| AC-4b | `Work_Initiated` con la HU en `todo` → HU `in_progress`. HU ya en `in_progress` → sin transición. | Test stamp |
| AC-5 | Label `fix` sin `pbi` → `warn`, Linear intacto. | Test stamp |
| AC-6 | `tracker-sync-replay` descarta un `todo` si el issue ya está en `in_progress` o más allá. | Test replay |
