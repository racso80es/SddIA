---
document_id: PBI-ARQUITECTURA-TRACKER-STAMP
uuid: "4bc88ff4-ac6b-441c-9c92-b7a7bf018173"
title: "[ARQUITECTURA] Tracker — tracker-stamp y correlación PR ↔ issue"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-stamp
historia_ref: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
historia_document_id: HU-SDDIA-TRACKER-LINEAR-001
historia_uuid: "26209dff-e413-4c6d-8838-5b785251356c"
created: "2026-10-02"
author: tekton
updated: "2026-10-02"
execution_mode: autonomo
especificacion_cerrada: "2026-10-02"
cola_ejecucion: docs/todos/pending/
blocked_by:
  - PBI-ARQUITECTURA-PROJECT-TRACKER-CONTRACT
  - PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER
  - PBI-ARQUITECTURA-TRACKER-SYNC-FAILED
  - PBI-ARQUITECTURA-WORK-INITIATED
unblocks:
  - PBI-SPIKE-LINEAR-DONE-GATE
baseline_decisiones:
  - "D2: HU padre / PBI hijo; labels precondición; warn sin corrección"
  - "D5: fail-soft → Tracker_Sync_Failed"
  - "D6: consume Work_Initiated"
  - "D7 (a): tracker_ref OPTIONAL en eventos PR"
---

# Proceso `tracker-stamp`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.D, F6, AC-8–AC-12, AC-18, AC-21, AC-22.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Añadir la tool a las fases de `feature` | Cerbero bloquearía (`context` sin `tracker-operations`). Un solo proceso suscriptor. |
| Los eventos PR ya identifican el issue | Falso. Ninguno lleva `tracker_ref`. **D7 está pendiente.** Sin laudo no se implementa la correlación. |
| Escribir estado en el markdown | D1: el markdown se archiva como hasta ahora. Este PBI solo espeja Linear. |

## 1. Intención

Suscriptor táctico que, ante señales de dominio, actualiza estado y comentarios en Linear. Los procesos de desarrollo permanecen ciegos al proveedor.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-PRC-1 | Proceso `tracker-stamp` vía `process-creator`. `context: [tracker-operations]`. Ejecutor de registro: Tekton. Fase `tool:linear-tracker-adapter` (runtime, sin LLM). |
| R-SUB-1 | Suscripción a `pbi-forged`, `work-initiated`, `pull-request-presented`, `pull-request-audited`, `pull-request-merged` en `event-domain-subscriptions.json`. |
| R-MAP-1 | Transiciones según HU §4.D. HU `done` solo si todos los hijos PBI están `done` (`fetch_issue` + `children`). HU `in_progress` si estaba `backlog` (vía `parent`). |
| R-NOP-1 | Sin `tracker` en manifiesto, o evento sin `tracker_ref` → no-op, sin invocar la cápsula. Label/`parent` incorrectos → `warn`, sin mutar Linear. |
| R-SOFT-1 | Error de cápsula → emitir `Tracker_Sync_Failed`, `warn` en envelope, éxito al padre. Discrepancia Git vs Linear al merge → `TRACKER_STATE_DIVERGED`. |
| R-D7-1 | Correlación PR ↔ issue **según laudo D7**. No implementar (a), (b) ni (c) por defecto. |

## 3. Plan

1. Init `feature` `tracker-stamp`.
3. Forjar proceso + suscripciones. Aplicar D7 en emisores de PR si el laudo es (a).
4. Suite E2E mock (ciclo completo, dos ciclos simultáneos, no-op, label incorrecta, Linear caído).
5. Cierre documental en la misma rama. El markdown del PBI de prueba termina en `done/` con `validacion.md` APTO.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-8 | Ciclo `feature` + `tracker_ref`: `in_progress` → `in_review` → `done` + comentarios (rama, `pr_url`, merge SHA). Markdown en `done/` + `validacion.md` APTO. | E2E mock |
| AC-9 | HU a `done` solo con el último hijo. | E2E |
| AC-10 | Sin `tracker_ref` / sin tracker: ciclo Core-self idéntico. | Suite existente |
| AC-11 | Linear caído: entrega completa + `Tracker_Sync_Failed` en pending. | Mock caído |
| AC-18 | Label/`parent` incorrectos → `warn`, Linear intacto. | Mock |
| AC-21 | `Work_Initiated` sin `tracker_ref` → no-op, cápsula no invocada. | Test |
| AC-22 | Dos ciclos simultáneos correlacionan el issue correcto (D7). | E2E |

## 5. Fuera de alcance

- Laudo D7 (humano). Spike del gate de Done. Escribir estado en markdown.

## 6. Dependencias

Contrato, cápsula, sync-failed, `Work_Initiated` (orden §7 HU).
