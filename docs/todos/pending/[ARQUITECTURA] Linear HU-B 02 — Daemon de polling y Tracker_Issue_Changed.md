---
document_id: PBI-LINEAR-B-02-WATCHER
uuid: "dcc3f288-89d8-47a2-8add-85300ceb6cec"
title: "[ARQUITECTURA] Linear HU-B 02 — Daemon de polling y Tracker_Issue_Changed"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 2
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-B-01-OUTBOUND
unblocks:
  - PBI-LINEAR-B-03-WEBHOOK
  - PBI-LINEAR-B-04-MARKDOWN-APPLY
baseline_decisiones:
  - "D-F (b): polling por intervalo; el webhook lab es el PBI 03"
  - "L10: no es un bucle continuo; intervalo con Daemon_Heartbeat"
---

# Daemon de polling y `Tracker_Issue_Changed`

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **02/06**. F7.1 (polling), F7.2, AC-11, AC-12.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| El daemon aplica el cambio al markdown | No. Jurisdicción de `github-bridge-watcher`: solo materializa eventos. El apply es el PBI 04. |
| Polling continuo | Intervalo configurable, default 60 s, más `Daemon_Heartbeat` cada 60 s. |

## 1. Intención

Sensor `linear-bridge-watcher` (vía `daemon-creator`, `daemons-contract` v1.0.0, `context: tracker-operations`) y evento `Tracker_Issue_Changed` (vía `event-creator`).

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Polling de `list_issues` por `team_key` + `project_id`. Cursor `updated_at` en `.SddIA/state/linear-bridge-watcher/{project_slug}.json`. |
| R-2 | Solo issues con label `tracker.labels.hu` o `tracker.labels.pbi`. El resto no genera evento. |
| R-3 | Anti-eco: descarta el cambio si el registro del PBI 01 tiene una operación sobre ese `issue_ref` dentro de la ventana, o si el último comentario lleva `sddia-sync-id`. |
| R-4 | Evento REQUIRED: `event_id`, `correlation_id`, `occurred_at`, `project_slug`, `issue_ref`, `changes[]` (`field` ∈ `state`, `title`, `description`, `priority`, `labels`, `parent`, `cancelled`), `source: poll`. OPTIONAL: `actor`, `linear_updated_at`. FORBIDDEN: token y cuerpo de respuesta. |
| R-5 | Diff de estado contra el snapshot local del cursor, no contra el markdown. |
| R-6 | Proyecto sin `tracker` → el daemon no lo sondea. |

## 3. Plan

1. Forja del daemon y del evento.
2. Tests con fixture lab (sin red): AC-11, AC-12.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-11 | Fixture `Backlog → Todo` → `Tracker_Issue_Changed` con `source: poll` y `changes[]` correcto. Issue sin label `hu`/`pbi` → sin evento. | Test daemon |
| AC-12 | Cambio presente en el registro saliente o con `sddia-sync-id` → sin evento. | Test daemon |
