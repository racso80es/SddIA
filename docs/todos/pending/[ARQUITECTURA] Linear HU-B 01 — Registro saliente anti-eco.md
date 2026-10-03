---
document_id: PBI-LINEAR-B-01-OUTBOUND
uuid: "6cc65ea4-6984-4e40-aa08-ab8b38ed6c8a"
title: "[ARQUITECTURA] Linear HU-B 01 — Registro saliente anti-eco"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 1
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-A-09-E2E
unblocks:
  - PBI-LINEAR-B-02-WATCHER
baseline_decisiones:
  - "HU 2 de 2: no se forja hasta PBI-LINEAR-A-09-E2E mergeado"
  - "D-F: el sensor ignora lo que SddIA acaba de escribir"
---

# Registro saliente anti-eco

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **01/06**. Prerrequisito de F7.1. Sin esto, el daemon del PBI 02 reaplicaría en el markdown cada sello de `tracker-stamp`.

## 1. Intención

Todo emisor que muta Linear deja constancia local **antes** de que el sensor inverso exista. El registro no sale del repo hacia Linear.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | `tracker-stamp`, `tracker-sync-replay`, la fase «Registro en tracker» de `forge-pbi` y la fase `tool:` de `refine-hu` appenden una línea en `.SddIA/state/tracker-outbound/{issue_ref}.jsonl` tras cada operación aceptada. |
| R-2 | Línea: `event_id`, `operation`, `issue_ref`, `occurred_at`, `source_process`. Sin token, sin cuerpo de respuesta. |
| R-3 | Escritura fail-soft: si el registro falla, la operación Linear ya hecha no se revierte; queda `warn`. |
| R-4 | Retención: el consumidor (PBI 02) lee una ventana configurable. Este PBI no poda. |
| R-5 | Bumps vía `process-creator` / `entity-manager`. Ningún `.md` de `feature` gana `tracker-operations`. |

## 3. Plan

1. Punto único de registro invocado por los cuatro emisores.
2. Tests: línea presente tras `update_issue_state` y tras `create_issue`; ausente si la cápsula falla; sin secretos.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-OUT-1 | Operación aceptada → línea JSON con los cinco campos. Operación rechazada → fichero intacto. | Test proceso |
| AC-OUT-2 | La línea no contiene `LINEAR_API_TOKEN` ni el cuerpo GraphQL. | Test proceso |
