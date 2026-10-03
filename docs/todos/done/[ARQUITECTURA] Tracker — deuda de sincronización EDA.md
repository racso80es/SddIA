---
document_id: PBI-ARQUITECTURA-TRACKER-SYNC-FAILED
uuid: "046e786b-3d0e-412d-aa3a-b4c28105d691"
title: "[ARQUITECTURA] Tracker — Tracker_Sync_Failed y replay fail-soft"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: tracker-sync-failed
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
  - PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER
unblocks:
  - PBI-ARQUITECTURA-TRACKER-STAMP
baseline_decisiones:
  - "D5: fail-soft; evento en eda_bus.pending; replay en suscriptor; dead-letter si el reintento falla"
---

# Deuda de sincronización — `Tracker_Sync_Failed`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.F, F8, AC-11–AC-13.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| El evento «queda en pending (Dead-letter)» | Buzones distintos: `./.events/pending` vs `./.events/dead-letter`. |
| `event-sweeper` reintenta Linear | Es ciego. Purge + alerta Kaizen ante `dead-letter`. El reintento es `tracker-sync-replay`. |
| Reaplicar a ciegas la transición | Puede retroceder un issue o duplicar comentarios. Consultar estado; marca `sddia-sync-id`. |
| Fallo de Linear = `System_Fracture_Detected` | No. No aplica Kintsugi. |

## 1. Intención

Que un fallo de Linear no aborte la entrega y deje deuda recuperable en el bus.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-EVT-1 | Evento `Tracker_Sync_Failed` (`SddIA/events/domain/tracker-sync-failed.md`) vía `event-creator`. REQUIRED: `event_id`, `correlation_id`, `source_process`, `issue_ref`, `operation`, `target_state` o `comment_kind`, `error_code`, `occurred_at`. OPTIONAL: `project_slug`, `pr_url`, `commit_sha`, `attempt`. FORBIDDEN: token y cuerpo HTTP de Linear. Incluir `TRACKER_STATE_DIVERGED`. |
| R-PRC-1 | Proceso `tracker-sync-replay` vía `process-creator`. `context: [tracker-operations]`. Suscrito en `event-domain-subscriptions.json`. `fetch_issue` → aplicar solo si el ciclo avanza (orden HU §4.D). Comentarios con `sddia-sync-id: {event_id}`. Segundo fallo → `dead-letter`. |
| R-SWP-1 | No mutar `event-sweeper`. La alerta Kaizen existente cubre `dead-letter`. |

## 3. Plan

1. Init `feature` `tracker-sync-failed`.
2. Forjar evento, proceso y suscripción vía creators.
3. Tests mock (éxito, obsoleto, segundo fallo).
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-11 (emisión) | Evento en `eda_bus.pending` sin secretos. | Test + `rg` |
| AC-12 | Replay aplica o descarta; cero comentarios duplicados. | Test mock |
| AC-13 | Segundo fallo → `dead-letter` + alerta Kaizen del sweeper. | Suite EDA lab |

La emisión desde `tracker-stamp` se cierra en `PBI-ARQUITECTURA-TRACKER-STAMP` (AC-11 e2e).

## 5. Fuera de alcance

- `tracker-stamp`. Webhooks. Kintsugi.

## 6. Dependencias

Bloqueado por la cápsula. Desbloquea `tracker-stamp`.
