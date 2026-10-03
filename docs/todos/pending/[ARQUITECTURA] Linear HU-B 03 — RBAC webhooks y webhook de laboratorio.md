---
document_id: PBI-LINEAR-B-03-WEBHOOK
uuid: "0a41adec-30b2-4d02-8971-f03a2c06fec7"
title: "[ARQUITECTURA] Linear HU-B 03 — RBAC webhooks y webhook de laboratorio"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 3
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-B-02-WATCHER
unblocks:
  - PBI-LINEAR-B-06-E2E
baseline_decisiones:
  - "D-F (b): webhook solo lab, túnel; sin endpoint público"
  - "Contrato de partición: RBAC 1.3.0 → 1.4.0 añade webhooks; crear issues ya está"
---

# RBAC webhooks y webhook de laboratorio

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **03/06**. F7.1 (webhook), AC-13, AC-22. El daemon del PBI 02 ya existe; este PBI le añade el puerto de entrada y el permiso.

## 1. Intención

`execution-contexts.md` §2.10 1.3.0 → **1.4.0**: el alcance gana «recibir webhooks firmados». El daemon acepta un POST local, valida `Linear-Signature` y emite el mismo `Tracker_Issue_Changed` con `source: webhook`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | Bump RBAC vía `entity-manager`. Siguen excluidos: borrar issues y administrar equipos. |
| R-2 | HMAC-SHA256 sobre el cuerpo con `LINEAR_WEBHOOK_SECRET`, leído de `env_ref` del proyecto. Prohibido en el manifiesto (`project-config-contract` ya lo veta). |
| R-3 | Firma ausente o inválida → descartado, sin evento, sin escribir el cuerpo en logs. |
| R-4 | Firma válida → mismo filtro de labels y el mismo anti-eco del PBI 02. `source: webhook`. |
| R-5 | El bind es local. Ningún artefacto documenta ni abre un endpoint público. |
| R-6 | El evento y los logs no contienen `LINEAR_WEBHOOK_SECRET` ni el cuerpo bruto. |

## 3. Plan

1. Bump de la norma y del modo webhook del daemon.
2. Tests de firma (AC-13, AC-22) con fixture, sin túnel real.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-13 | Firma inválida → sin evento. Firma válida → `Tracker_Issue_Changed` `source: webhook`. El payload bruto no está en el evento. | Test daemon |
| AC-22 | Ningún envelope ni evento contiene `LINEAR_WEBHOOK_SECRET`. | Test daemon |
| AC-WH-1 | §2.10 1.4.0 menciona webhooks firmados y sigue excluyendo borrado y administración. | Revisión del artefacto |
