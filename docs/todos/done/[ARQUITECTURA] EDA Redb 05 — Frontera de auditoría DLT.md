---
document_id: PBI-EDA-REDB-05-DLT-FRONTIER
uuid: "2d7f4a91-6c8e-4b35-a1d9-5e0c8b7f3a26"
title: "[ARQUITECTURA] EDA Redb 05 — Frontera de auditoría DLT"
format: markdown
version: "1.0.0"
created: "2026-10-21"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
historia_ref: "docs/todos/done/historias/HU-SDDIA-EDA-REDB-001: Evolución del Bus de Eventos a Redb, Jurisdicción Cognitiva y Estandarización del Marco SddIA.md"
historia_document_id: HU-SDDIA-EDA-REDB-001
historia_uuid: "7c4e9a12-8b3d-4f61-a9e0-2d5c6b8f1a47"
suggested_branch: feat/eda-redb-05-dlt-frontier
persist_ref_suggested: docs/features/eda-redb-05-dlt-frontier
blocked_by:
  - PBI-EDA-REDB-02-WRITERS
unblocks:
  - PBI-EDA-REDB-07-CUTOVER
laudos:
  - L-REDB-07
related:
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/events-contract.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/events/domain/system-immunity-certified.md
  - SddIA/engine/execute-process/src/engine/dlt_telemetry_anchor.rs
  - SddIA/tools/iota-immutable-publisher/src/main.rs
  - SddIA/tools/sddia-qa/src/verify_domain_subscription_parity.rs
  - SddIA/norms/pull-request-orchestration.md
acceptance:
  - AC-5
  - AC-6
  - AC-10
---

# Frontera de auditoría DLT

Historia madre: §5, L-REDB-07. Línea base medida en `event-domain-subscriptions.json`: `iota-immutable-publisher` está en 18 claves (14 `cumulo`, 4 `radamanto`).

Genoma solo vía `execute-process` → `entity-manager`. La paridad la exige `verify_domain_subscription_parity.rs`: la tabla `## Suscripciones` de cada `{evento}.md` y el JSON deben quedar iguales.

## 1. Se conservan

| Evento | Suscriptor | Cambio |
| :--- | :--- | :--- |
| `PullRequest_Merged` | `cumulo` / `tool:iota-immutable-publisher` | `dlt_audit_scope: external_public_interest`. OPTIONAL nuevos: `participating_agents[]` (`uuid`, `name`, `version`, `role`) y `authorizer`. Ausentes: el ancla sigue. Presentes: entran al digest. |
| `System_Immunity_Certified` | `radamanto` / `tool:iota-immutable-publisher` | Mismo scope. Payload REQUIRED intacto. |

`events-contract` 1.1.0 → 1.2.0 añade el enum `dlt_audit_scope: local_only | external_public_interest`. Default de quien no lo declara: `local_only`.

## 2. Bajan el suscriptor IOTA

El resto de suscriptores de cada clave no se toca. Scope `local_only` en la clase.

| Evento | Agente del ancla que se retira |
| :--- | :--- |
| `PullRequest_Presented` | cumulo |
| `Domain_Entity_Created` | cumulo |
| `Domain_Entity_Updated` | cumulo |
| `Domain_Entity_Deleted` | cumulo |
| `Domain_Entity_Telemetry_Captured` | cumulo |
| `Domain_Entity_Degraded` | radamanto |
| `Domain_Entity_Restored` | radamanto |
| `Domain_Entity_Deprecated` | radamanto |
| `TelegramMessage_Received` | cumulo |
| `Manual_Task_Requested` | cumulo |
| `Kalma2_Process_Requested` | cumulo |
| `Kaizen_Idea_Captured` | cumulo |
| `Thought_Persisted` | cumulo |
| `Vector_Memory_Indexed` | cumulo |
| `Instance_Deployed` | cumulo |
| `Instance_Torn_Down` | cumulo |

`dlt_telemetry_anchor.rs`: `Domain_Entity_Telemetry_Captured` sigue escribiendo `.SddIA/proofs/dlt-telemetry/`. Deja de invocar `iota-immutable-publisher`.

Con `SDDIA_EVENT_STORE=dual|redb`, el `tx_hash` de las dos claves que siguen en IOTA se persiste en `dlt_audit_seals`. En `filesystem`, el recibo queda donde está hoy.

`pull-request-orchestration.md`: la aceptación (`PullRequest_Merged`) es el ancla externa. Los `Domain_Entity_*` del mismo cambio no salen a IOTA; su digest puede ir dentro del sello de merge. Bump de la norma vía `entity-manager`.

No se añade `IOTA_ANCHOR_PACKAGE_ID`. Siguen `IOTA_WALLET_SECRET`, `IOTA_PUBLISH_RELAY_URL`, `SDDIA_LAB_SIMULATE_IOTA`, `SDDIA_LAB_MOCK_IOTA_URL`. `iota-publish-relay` no se rediseña.

## 3. Fuera de alcance

- Cutover del bus de ficheros.
- Descriptores de agentes (PBI 06).
- Cambiar el binario del publisher salvo que un test mock lo exija para contar invocaciones.

## 4. Criterios

| ID | Criterio |
| :--- | :--- |
| AC-5 | `Raw_Execution_Finished`, `PullRequest_Presented` y los 16 eventos de §2 no invocan IOTA. `PullRequest_Merged` y `System_Immunity_Certified` sí, y dejan `tx_hash` en `dlt_audit_seals` cuando el store está activo. |
| AC-6 | `PullRequest_Merged` valida con y sin `participating_agents` / `authorizer`. |
| AC-10 | `verify-domain-subscription-parity` en verde. `rg iota-immutable-publisher SddIA/core/event-domain-subscriptions.json` solo acierta en las dos claves de §1. |
