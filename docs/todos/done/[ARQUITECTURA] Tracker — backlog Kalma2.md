---
document_id: PBI-ARQUITECTURA-KALMA2-BACKLOG
uuid: "20106dc7-1881-4d97-aaf0-9bce8353c339"
title: "[ARQUITECTURA] Tracker — backlog HU/PBI en Kalma2 por proyecto"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: kalma2-tracker-backlog
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
unblocks: []
baseline_decisiones:
  - "D3: sin tracker → tracker_configured false; no parseo markdown"
  - "D4.1: proceso tracker-backlog-query, context tracker-operations, tool directa, síncrono, sin LLM"
---

# Backlog HU/PBI en Kalma2

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.C, F5, AC-5–AC-7, AC-17.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| El puente llama a la cápsula con `Command::new` | Se saltaría Cerbero. El puente ejecuta `tracker-backlog-query` vía `execute-process`. |
| Fase `agent:tekton` | Despertaría LLM. Fase `tool:linear-tracker-adapter`. |
| El proceso puede ser `detached` | El panel necesita respuesta síncrona (`items` en el envelope). |
| Sin tracker, leer `docs/todos` | D3: aviso, `items: []`. Hoy Core-self y `barcelonaxplorer` no tienen tracker. |

## 1. Intención

Listar HU y PBI del proyecto seleccionado. Sin selección: Core-self. Sin tracker: aviso explícito.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-PRC-1 | Proceso `tracker-backlog-query` vía `process-creator`. `context: [tracker-operations]`. Ejecutor de registro: Tekton. Inputs: `project_slug` opcional, `kind` (`hu`\|`pbi`\|`all`), `state`. Una fase `tool:linear-tracker-adapter` (`list_issues` + filtro de label). Síncrono, sin `detached`, sin `llm:*`. |
| R-API-1 | `GET /api/backlog?project_slug=&kind=&state=` en `kalma2-bridge`. Sin tracker: `200` `{ tracker_configured: false, items: [] }` sin ejecutar el proceso. Con tracker: `items[]` (`kind`, `id`, `title`, `state`, `priority`, `parent_id`, `url`). `kind` se deriva de `labels[]` vs `tracker.labels.*`. Issues sin `hu`/`pbi` no se listan. |
| R-API-2 | Fallo de Linear al listar: error visible. No emitir `Tracker_Sync_Failed` (es lectura). Cero `project_root` ni rutas absolutas (O3). |
| R-UI-1 | Panel «Backlog» en `interfaces/kalma2/` que se recarga al cambiar el selector. Filtros tipo/estado. Texto: «Proyecto sin tracker configurado». Solo lectura. |

## 3. Plan

1. Init `feature` `kalma2-tracker-backlog`.
2. Forjar el proceso. Ruta + UI.
3. Tests de puente (con/sin tracker, aislamiento de `team_key`) + prueba manual.
4. Cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-5 | Proyecto `provider: linear` (mock) lista solo su `team_key`. | Test puente |
| AC-6 | `barcelonaxplorer` y Core-self sin tracker → `tracker_configured: false` + aviso UI. | Test + prueba Kalma2 |
| AC-7 | Cero `project_root` / rutas absolutas en API y UI. | `rg` |
| AC-17 | Proceso síncrono, sin LLM. | Envelope + telemetría |

## 5. Fuera de alcance

- Transiciones desde la UI. Crear issues. Parseo markdown.

## 6. Dependencias

Bloqueado por contrato 1.2.0 y por la cápsula.
