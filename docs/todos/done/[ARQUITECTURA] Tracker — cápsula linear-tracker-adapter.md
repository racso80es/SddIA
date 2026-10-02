---
document_id: PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER
uuid: "87ac8a3b-a7e3-4a18-892d-9474fa073dda"
title: "[ARQUITECTURA] Tracker — cápsula linear-tracker-adapter"
format: markdown
version: "1.0.0"
status: done
closed: "2026-10-02"
execution_branch: feat/tracker-operations-context
priority: alta
type: arquitectura
process: feature
dispatch: true
feature_name: linear-tracker-adapter
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
  - PBI-ARQUITECTURA-TRACKER-OPERATIONS-CONTEXT
unblocks:
  - PBI-ARQUITECTURA-KALMA2-BACKLOG
  - PBI-ARQUITECTURA-TRACKER-SYNC-FAILED
  - PBI-ARQUITECTURA-TRACKER-STAMP
baseline_decisiones:
  - "D2: list_issues filtra labels en GraphQL; la cápsula no asigna ni exige labels"
  - "D4: context tracker-operations"
  - "Secreto de bóveda: LINEAR_API_TOKEN (no LINEAR_API_KEY)"
---

# Cápsula `linear-tracker-adapter`

Historia madre: `HU-SDDIA-TRACKER-LINEAR-001` §4.B, F1–F4, AC-1–AC-4, AC-16.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Definición `spec.json` / `spec.md` bajo el crate | Prohibido. Definición `SddIA/tools/linear-tracker-adapter.md` + crate `SddIA/tools/linear-tracker-adapter/`. Forja vía `entity-manager`. |
| WASM WASI | Preview1 no tiene sockets. Binario nativo (`ureq`), desviación §8 de `tools-contract` (precedente: `gemini-http-infer`, `send-telegram-notification`). |
| Env `LINEAR_API_KEY` | La bóveda de instancia declara **`LINEAR_API_TOKEN`**. La cápsula lee ese nombre. Alias `LINEAR_API_KEY` no es SSOT. |
| La cápsula emite `Tracker_Sync_Failed` o asigna labels | Ceguera: stdin → GraphQL → stdout. Eventos y labels son del proceso. |

## 1. Intención

Una tool ciega que habla GraphQL con Linear: leer, listar, transicionar estado y comentar. Sin decisiones de dominio.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-DEF-1 | `{name}.md` con `uuid`, `context: tracker-operations`, `io_mode: capsule-json-io`, `implementation_path_ref`. Índice `tools/index.md`. |
| R-IO-1 | Envelope `capsule-json-io` 2.0. `exitCode: 0 ⟺ success: true`. `request.operation` ∈ `fetch_issue` \| `list_issues` \| `update_issue_state` \| `create_comment`. |
| R-OP-1 | Contratos de I/O según HU §4.B (tablas). `list_issues` envía el filtro `labels` en la query GraphQL. |
| R-AUTH-1 | Token desde `LINEAR_API_TOKEN` (SO > proyecto `env_ref` > instancia > global). Ausente → `LINEAR_AUTH_MISSING`. Rechazo API → `LINEAR_AUTH_REJECTED`. Nunca en envelope. |
| R-ERR-1 | Códigos: `LINEAR_AUTH_MISSING`, `LINEAR_AUTH_REJECTED`, `LINEAR_NOT_FOUND`, `LINEAR_STATE_AMBIGUOUS`, `LINEAR_STATE_UNKNOWN`, `LINEAR_RATE_LIMITED`, `LINEAR_TRANSPORT`, `LINEAR_GRAPHQL_ERROR`. |
| R-LAB-1 | Mock `SDDIA_LAB_MOCK_OUTBOUND` + `SDDIA_LINEAR_API_URL` opcional (default `https://api.linear.app/graphql`). |

## 3. Plan

1. Init `feature` `linear-tracker-adapter` (tras el PBI de contexto).
2. Forjar definición vía `entity-manager`. Crate nativo + tests con mock.
3. Evolution + cierre documental en la misma rama.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Definición indexada con `context: tracker-operations`. | `sddia-qa verify-tools-index` |
| AC-2 | Cuatro operaciones + errores tipados contra mock. | Tests del crate |
| AC-3 | Sin secreto en envelope. Sin `LINEAR_API_TOKEN` → `LINEAR_AUTH_MISSING`. | Test + `rg` |
| AC-4 | Nombre de estado ambiguo/inexistente no muta el issue. | Test mock |
| AC-16 | `list_issues` + label `hu` no descarga otras labels (query GraphQL). | Test mock |
| AC-14 (tool) | Invocación desde proceso con `context: [tracker-operations]` pasa Cerbero; con solo `system-operations` falla. | Test RBAC |

## 5. Fuera de alcance

- Crear/borrar issues, webhooks, UI, eventos EDA, `tracker-stamp`.

## 6. Dependencias

Bloqueado por `PBI-ARQUITECTURA-TRACKER-OPERATIONS-CONTEXT`.
