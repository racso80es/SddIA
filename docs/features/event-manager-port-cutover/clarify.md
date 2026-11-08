---
feature_name: event-manager-port-cutover
created: "2026-11-08"
updated: "2026-11-08"
process: refactorization
purpose: Estabilización de alcance — cutover total a EventManagerPort; Redb exclusivo; retirada legacy filesystem/dual/sddia-event-store
branch_name: refactor/event-manager-port-cutover
persist_ref: docs/features/event-manager-port-cutover
pbi_ref: MAN-35
parent_ref: MAN-33
document_id: PBI-176f9f2f
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
execution_id: "8b39f170-38b9-4aea-961b-31c4c63512b7"
tracker_channel: ".SddIA/state/tracker-outbound/MAN-35.jsonl"
---
# Clarificación — event-manager-port-cutover

Transcript Mayeuta (2026-11-08). Fase **Estabilización de alcance** del proceso `refactorization`.
Canal tracker: **MAN-35** (padre **MAN-33**). Documento semilla **PBI-176f9f2f**. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-35** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-35.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `refactorization`, 2026-11-08). |
| Padre | **MAN-33** (auditoría de bypass productivos; canal `.SddIA/state/tracker-outbound/MAN-33.jsonl`). |
| Documento | **PBI-176f9f2f** — cutover total a `EventManagerPort` (semilla de orquestación). |
| Cuerpo Linear en disco | **No materializado** en esta sesión: `.SddIA/lab-linear-store.json` tiene `issues: {}`; el jsonl outbound no vuelca descripción. **No bloquea**: semilla + canal + AC/tajos bastan para estabilizar. |
| Corrección de ids | Artefactos previos en `persist_ref` citaban **MAN-101** / **PBI-f84fbfbc** (ciclo paralelo/lab). **Este execution_id** fija **MAN-35** / **PBI-176f9f2f**. |
| Predecesor aditivo | `eda-event-manager-additive` (MAN-32 / contrato v1.0.0) — este ciclo **extiende a 1.2** y **cablea** el motor. |
| Modo | `branch_pr` / `refactorization` — tajos T0…T8 verificables. |

## D1 — Matriz PBI × alcance estabilizado

| Afirmación (semilla / PBI) | Laudo |
|---------------------------|-------|
| Runtime único `.SddIA/runtime/events.redb` | **L-RUNTIME** — SSOT físico; JSON bajo `.events/*` no es cola productiva. |
| `.events/*` solo identificadores lógicos | **L-LOGICAL-PATH** — compat de nombres/tablas; cero I/O de ECST por consumidores. |
| Bypass MAN-33 (route_domain/fractal, daemons, DLT, kalma2, audit, purger, lancedb, instance_creator, DLQ directo) | **L-BYPASS** — todos migran al puerto; lista cerrada de tajos T3–T6. |
| Contrato EventManager 1.2 (`list_events`, `list_deliveries`, sellos DLT, consultas admin) | **L-CONTRACT-12** — ampliar schema/suite/adapters/host; provider-neutral. |
| Composition root único | **L-COMPOSE** — solo el host conoce el adapter; consumidores ven `Arc<dyn EventManagerPort>`. |
| Retirada `sddia-event-store`, `StoreMode::{Filesystem,Dual}`, `SDDIA_EVENT_STORE`, `fs_event_adapter` productivo | **L-RETIRE** — T7; allowlist solo migration/adapter autorizado. |
| Gate CI arquitectónico | **L-GATE** — veto a `EventStore::open`, `sddia_event_store`, modos legacy, open de `events.redb` y `fs::*` sobre `.events` fuera de allowlist. |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Frontera productiva única: **`EventManagerPort`** / capacidad **`eda:event-manager`**. El plano interno del adapter (Redb) no es API de consumidores. |
| **L-RUNTIME** | Único store productivo: **`.SddIA/runtime/events.redb`**. Prohibidos modos `filesystem` y `dual` en runtime. |
| **L-LOGICAL-PATH** | Rutas `.events/*` = identificadores lógicos / nombres de tabla; **nunca** fuente de verdad ni cola. Un JSON físico conflictivo **nunca** prevalece sobre Redb. |
| **L-ECST** | ECST inmutable. `delivery_state` **no** se muta dentro del evento. Entregas solo en tabla/contrato **`subscriber_delivery`**. |
| **L-OPS-PORT** | DLQ, replay, purge, progreso y DLT **exclusivamente** vía puerto (ops 1.2 + existentes). |
| **L-CONTRACT-12** | Extensión contractual **1.2**: `list_events` (filtros+cursor), `list_deliveries`, sellos DLT y consultas administrativas provider-neutral; actualizar RedbEventAdapter, mock, adapter externo, host y suite. |
| **L-COMPOSE** | Composition root único: factory/host elige adapter; consumidores y daemons **sin** conocimiento de tablas, paths ni tipos del adapter. |
| **L-STRICT-CORE** | `route_domain_core` / loader: verificar eventos inline; carga siempre por `event_id`; eliminar precedencia de fichero, mutaciones `delivery_state`, barridos y movimientos filesystem. |
| **L-FRACTAL** | `route_fractal_core`, `fractal_bus`, heartbeat, memory evolution, radamanto y compliance → ACK/NACK, DLQ y DLT del puerto (T4). |
| **L-PRODUCERS** | Migrar kalma2-bridge, lancedb_thought_repo, instance_creator, Cerbero, output validator y github bridge; eliminar buses/DLQ paralelos (T5). |
| **L-AUDIT-ADMIN** | `event-bus-audit` solo por contrato; purger sin open directo a Redb; limpieza histórica vía operación administrativa oficial (T6). |
| **L-NON-EVENT-FS** | Proofs, configuración y estado local **no-evento** pueden seguir en filesystem bajo rutas SSOT separadas (fuera del bus). |
| **L-VERIFY** | Suite contractual (Redb + InMemoryEventManagerMock + adapter externo); `cargo test --workspace`; gate arquitectónico verde; `validacion` APTO solo tras invariantes. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |

## D3 — Tajos canónicos (orden de forja)

| Tajo | Contenido estabilizado |
|------|------------------------|
| **T0** | Norma: Redb exclusivo; prohibir filesystem/dual; diferenciar ECST vs proofs/config/estado local. |
| **T1** | Contrato EventManager **1.2** + adapters/mock/host/suite. |
| **T2** | Composition root único; DI puerto-only en consumidores. |
| **T3** | Loader + `route_domain_core` estrictos. |
| **T4** | Bus fractal + daemons. |
| **T5** | Productores (kalma2, lancedb, instance_creator, Cerbero, github bridge, validators). |
| **T6** | Auditoría + purger administrativo. |
| **T7** | Retirada `sddia-event-store` y modos legacy. |
| **T8** | Pruebas con mock + gate arquitectónico CI. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Migración de JSON históricos en `.events/`? | **No** como cola productiva. Residuos = fractura / limpieza administrativa; Redb prevalece (**L-LOGICAL-PATH**). |
| 2 | ¿`fs_event_adapter` sobrevive? | Solo no-productivo / tests si la suite lo exige; **retirada productiva** en T7 (**L-RETIRE**). |
| 3 | ¿Convivencia con contrato v1.0.0 aditivo? | Este refactor **sube a 1.2** y cablea el motor; el enabler aditivo queda absorbido por el corte. |
| 4 | ¿Cuerpo Linear MAN-35 / MAN-33 completo en disco? | No bloquea: canal MAN-35 confirmado; semilla **PBI-176f9f2f** + laudos + AC bastan para Dedalo/Tekton. |
| 5 | ¿MAN-101 vs MAN-35? | **MAN-35** es el canal canónico de este `execution_id`; MAN-101 queda como traza lab previa, no como `pbi_ref` de esta estabilización. |

## D5 — Veredicto

**ok** — Alcance estabilizado. Handoff a planificación/forja con tajos T0…T8, invariantes L-ECST/L-OPS-PORT/L-LOGICAL-PATH/L-COMPOSE y AC 1…10 como contrato de Done. Tracker **MAN-35** / documento **PBI-176f9f2f** / padre **MAN-33**.
