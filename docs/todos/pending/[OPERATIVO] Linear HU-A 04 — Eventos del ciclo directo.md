---
document_id: PBI-LINEAR-A-04-EVENTS
uuid: "23005b41-5c43-401f-ab00-8fd380f25ec2"
title: "[OPERATIVO] Linear HU-A 04 — Eventos del ciclo directo"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: operativo
process: feature
dispatch: true
hu_order: 4
hu_order_total: 9
historia_ref: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
historia_document_id: HU-LINEAR-DIRECT-CYCLE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-01-CONTRACT
unblocks:
  - PBI-LINEAR-A-06-REFINE
  - PBI-LINEAR-A-08-STAMP
baseline_decisiones:
  - "D-D: HU_Refined y PBI_Refined son el único trigger hacia todo"
  - "D7: tracker_ref viaja como cadena opaca; sin campos de proveedor"
---

# Eventos del ciclo directo

HU `HU-LINEAR-DIRECT-CYCLE` (1 de 2), orden **04/09**. F3, F4, F5, F6 (contratos de evento). Emisores los cablean los PBI 05, 06 y el `delivery-close-cycle` de 08.

## 1. Intención

Cuatro contratos ECST vía `event-creator`, antes de que exista quien los emita. Ninguno lleva campos de proveedor (`team_key`, ids de `WorkflowState`, URLs de Linear).

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `PBI_Refined`. REQUIRED: `event_id`, `correlation_id`, `source_process: refine-pbi`, `pbi_ref`, `occurred_at`. OPTIONAL: `project_slug`, `tracker_ref`. |
| R-2 | `HU_Refined`. REQUIRED: `event_id`, `correlation_id`, `source_process: refine-hu`, `hu_ref`, `occurred_at`. OPTIONAL: `project_slug`, `tracker_ref`. |
| R-3 | `PBI_Cancelled`. REQUIRED: `tracker_ref` o `pbi_ref`, `reason`. Emisores autorizados en esta HU: `task-queue-manager` y acción explícita desde Kalma2. HU-B añadirá `tracker-markdown-apply`. |
| R-4 | `Delivery_Committed` 1.0.0 → **1.1.0**. OPTIONAL nuevos: `tracker_ref`, `pbi_ref`, `persist_ref`, `commit_sha`. REQUIRED previos intactos. Sigue FORBIDDEN `hash_signature`. |
| R-5 | Los cuatro quedan suscribibles. La suscripción de `tracker-stamp` la hace el PBI 08, no este. |

## 3. Plan

1. Forja de los tres eventos nuevos y bump de `delivery-committed` vía `event-creator`.
2. Test de validación ECST: payload mínimo válido; payload con `team_key` rechazado.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-EV-1 | Los tres eventos nuevos validan con su payload mínimo y rechazan campos de proveedor. | Test ECST |
| AC-EV-2 | `Delivery_Committed` 1.1.0 acepta `tracker_ref` opcional; un emisor 1.0.0 sin ese campo sigue validando. | Test ECST |
