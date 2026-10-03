---
document_id: PBI-LINEAR-B-04-MARKDOWN-APPLY
uuid: "f3bacc6c-5d39-45a2-8428-cbdf08e89962"
title: "[ARQUITECTURA] Linear HU-B 04 — tracker-markdown-apply"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 4
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-B-02-WATCHER
unblocks:
  - PBI-LINEAR-B-05-DONE-GATE
baseline_decisiones:
  - "D-G (a): un PR en tracker-sync/{project_slug} vía la skill git-manager"
  - "D-H (a): el cuerpo solo se escribe con tracker.labels.editable"
  - "L7: git-manager es una skill, no una cápsula"
  - "L8: la cascada F2 nunca es destino"
---

# `tracker-markdown-apply`

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **04/06**. F7.3, AC-14, AC-15, AC-16. Suscriptor de `Tracker_Issue_Changed`. Con `done_gate: git` (único valor legal hasta el PBI 05) un `state → done` solo escribe `tracker_state`.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Hay que proteger `objectives.md` y `spec.md` con la label | Esos ficheros no se resuelven por `tracker_ref`. La label protege el cuerpo del PBI o de la HU. |
| Push directo a `default_branch` con un bypass | Lo veta el pre-push y `task-closure-documental`. La salida es un PR. |

## 1. Intención

Proceso `tracker-markdown-apply` (`context: [tracker-operations, filesystem-ops, source-control]`, handler nativo, sin LLM). Réplica el issue sobre el markdown local y abre PR; no crea PBI ni HU nuevos.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Resuelve el markdown por `tracker_ref` en `todos_pending`, `todos_done` e `historias/`. Sin markdown → `warn` no-op. |
| R-2 | `cancelled` → `status: cancelado`, mover a `todos_done`, emitir `PBI_Cancelled` (emisor autorizado nuevo sobre el evento del PBI A-04). |
| R-3 | `todo`, `backlog`, `in_progress`, `in_review` → solo `tracker_state`. No mueve ficheros ni cambia `status`. |
| R-4 | `done` con `done_gate: git` → solo `tracker_state: done`. El archivado llega con el PBI 05. |
| R-5 | `title` → frontmatter `title`, sin renombrar. `priority` 0–4 → `sin-prioridad\|urgente\|alta\|media\|baja`. `parent` → `historia_ref` solo si esa HU existe en local. |
| R-6 | `description` → cuerpo solo si el issue tiene `tracker.labels.editable`. Si no, `warn` no-op. |
| R-7 | Commit en `tracker-sync/{project_slug}` mediante la skill `git-manager`. Un PR por día o por lote de N. Prohibido push a `default_branch`. |
| R-8 | Idempotencia: `tracker_sync_last_event` en frontmatter; `occurred_at` anterior se descarta. |
| R-9 | Conflicto (markdown tocado después de `linear_updated_at`) → gana el repo, `Tracker_Sync_Failed` con `error_code: TRACKER_STATE_DIVERGED`, fichero intacto. |

## 3. Plan

1. Forja del proceso vía `process-creator` y suscripción al evento.
2. Tests mock AC-14, AC-15, AC-16. El PR se afirma sobre la rama, no se mergea.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-14 | `cancelled` mueve y emite `PBI_Cancelled`. `title` no renombra. `description` sin label `editable` → `warn`; con label → cuerpo actualizado. | Test proceso |
| AC-15 | Markdown modificado tras `linear_updated_at` → repo gana, `TRACKER_STATE_DIVERGED`, fichero intacto. | Test proceso |
| AC-16 | Los commits caen en `tracker-sync/{project_slug}`. Ninguno en `default_branch`. | Test proceso |
