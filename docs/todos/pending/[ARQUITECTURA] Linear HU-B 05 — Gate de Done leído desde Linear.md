---
document_id: PBI-LINEAR-B-05-DONE-GATE
uuid: "84f4193e-9058-4a4a-93ab-517e6b4cd675"
title: "[ARQUITECTURA] Linear HU-B 05 — Gate de Done leído desde Linear"
format: markdown
version: "1.0.0"
status: pending
created: "2026-10-03"
author: tekton
priority: alta
type: arquitectura
process: feature
dispatch: true
hu_order: 5
hu_order_total: 6
historia_ref: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
historia_document_id: HU-LINEAR-SSOT-INVERSE
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-LINEAR-B-04-MARKDOWN-APPLY
unblocks:
  - PBI-LINEAR-B-06-E2E
baseline_decisiones:
  - "D-I (a): done_gate linear solo se activará en Core-self, y no en este PBI"
  - "Reabre el laudo tracker-done-gate; no elimina el gate, cambia su fuente"
  - "Sin bump de project-config-contract: 1.3.0 ya declaró el campo y solo aceptaba git"
---

# Gate de Done leído desde Linear

HU `HU-LINEAR-SSOT-INVERSE` (2 de 2), orden **05/06**. F8.1, F8.2, AC-17…AC-20, AC-23. Implementa la fuente nueva **apagada**: el default sigue siendo `git` y nadie pasa un proyecto a `linear` aquí (eso es el paso 4 de F8.3, PBI 06, con laudo).

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `TRACKER_STATE_DIVERGED` es un evento | Es un `error_code` de `Tracker_Sync_Failed`. |
| Editar `.cursor/rules/task-closure-documental.mdc` a mano | Se regenera desde la norma tras el bump por `norm-creator`. |
| Subir `project-config-contract` a 1.4.0 | El campo ya existe desde HU-A. Este PBI solo afloja el validador. |

## 1. Intención

Con `done_gate: linear`, Done = merge (o `Delivery_Committed`) + issue en `done` + `validacion.md` APTO. `pbi_archived` y el movimiento a `done/` dejan de ser gate y pasan a efecto de `tracker-markdown-apply`.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-1 | El validador 1.3.0 acepta `done_gate` ∈ `{git, linear, both}`. Default `git`. Sin `tracker` → `git` forzado. No hay bump de `contract_version`. |
| R-2 | `feature-pbi-archive` lee `done_gate`. En `linear`/`both` consulta `fetch_issue`. `accept-pr` gana `tracker-operations` solo en «Sincronización y Limpieza». Linear caído → `Tracker_Sync_Failed` + fallback `git` con `warn`; la entrega no se bloquea. |
| R-3 | `tracker-stamp`, al sellar `done`, escribe `.SddIA/proofs/tracker-done/{tracker_ref}.json` (issue, estado, `occurred_at`, sha). Sin token. |
| R-4 | Pre-push lee esa evidencia y no abre red. Con `done_gate: linear` y sin evidencia → bloqueo citando el `tracker_ref`. Con `git` → comportamiento actual (`pbi_archived`, PBI en `done/`). |
| R-5 | `tracker-markdown-apply`: con `linear`/`both`, `state → done` archiva el PBI en `todos_done`. Con `git`, se queda en R-4 del PBI 04. |
| R-6 | `task-closure-documental` y `features-documentation-pattern` 1.2.0 → **1.3.0** vía `norm-creator`: «mover el PBI en el mismo PR» solo obliga con `done_gate: git` o `both`. |
| R-7 | `SddIA/evolution/`: reapertura del laudo `tracker-done-gate`, con su uuid. |
| R-8 | Ningún manifiesto de proyecto cambia a `linear` en este PBI. |

## 3. Plan

1. Validador, handler, prueba de sello, pre-push, apply y normas.
2. Tests AC-17…AC-20 y AC-23, en fixture, sin tocar el manifiesto real del Core.
3. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-23 | Manifiesto 1.3.0 con `done_gate: linear` o `both` valida, sin bump de versión de contrato. | Test contrato |
| AC-17 | `done_gate: git` → `feature-pbi-archive` y pre-push idénticos a hoy. | Suite Core-self |
| AC-18 | `linear` + issue `done` → acepta sin `pbi_archived` y sin PBI en `done/`. Issue `in_review` → rechaza citando `tracker_ref`. | Test handler |
| AC-19 | `both` con Linear caído → fallback `git`, `warn`, `Tracker_Sync_Failed`, entrega no bloqueada. | Test handler |
| AC-20 | El pre-push lee `.SddIA/proofs/tracker-done/{tracker_ref}.json` y no abre red. | Test hook |
