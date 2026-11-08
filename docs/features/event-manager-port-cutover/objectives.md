---
feature_name: event-manager-port-cutover
created: "2026-11-08"
updated: "2026-11-08"
process: refactorization
branch_name: refactor/event-manager-port-cutover
persist_ref: docs/features/event-manager-port-cutover
pbi_ref: MAN-35
parent_ref: MAN-33
document_id: PBI-176f9f2f
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
execution_id: "8b39f170-38b9-4aea-961b-31c4c63512b7"
---
# Objetivos — event-manager-port-cutover

## Misión

Cutover total a **`EventManagerPort`**: eliminar accesos directos al almacenamiento de eventos y fijar el puerto como única frontera productiva para publicación, consulta, entregas, DLQ, trazas y sellos DLT. Runtime canónico **`.SddIA/runtime/events.redb`**; rutas **`.events/*`** solo como identificadores lógicos. Tracker **MAN-35** (padre **MAN-33**, documento **PBI-176f9f2f**).

**Por qué:** la auditoría MAN-33 confirmó bypass productivos (route_domain/fractal, daemons, DLT, kalma2-bridge, event-bus-audit, purger, lancedb, instance_creator, productores DLQ) y deuda de `sddia-event-store` / modos filesystem-dual. El enabler aditivo (contrato 1.0) no basta: hay que extender a **1.2**, cablear el motor y retirar el legado.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-35** (canal `.SddIA/state/tracker-outbound/MAN-35.jsonl`); padre **MAN-33** |
| Semilla | **PBI-176f9f2f** (`refactorization`, `branch_pr`) |
| Predecesor | `eda-event-manager-additive` (MAN-32 / v1.0.0) |
| Runtime | `.SddIA/runtime/events.redb` exclusivo |
| Clarificaciones | `clarify.md` (laudos L-*) |
| Nota ids | Sustituye frontmatter lab previo MAN-101 / PBI-f84fbfbc para este `execution_id` |

## Alcance (tajos T0–T8)

| Tajo | Objetivo |
|------|----------|
| **T0** | Norma Redb exclusiva; prohibir filesystem/dual; separar ECST de proofs/config/estado local. |
| **T1** | Contrato EventManager **1.2**: `list_events` (filtros+cursor), `list_deliveries`, sellos DLT y consultas admin; actualizar RedbEventAdapter, mock, adapter externo, host y suite. |
| **T2** | Composition root único: consumidores → puerto; solo host conoce adapter; solo adapter conoce físico. |
| **T3** | Loader y `route_domain_core` estrictos: verify inline; carga por `event_id`; sin precedencia de fichero ni mutaciones/barridos filesystem. |
| **T4** | Bus fractal y daemons (route_fractal, fractal_bus, heartbeat, memory evolution, radamanto, compliance) → ACK/NACK/DLQ/DLT del puerto. |
| **T5** | Productores: kalma2-bridge, lancedb_thought_repo, instance_creator, Cerbero, output validator, github bridge; sin buses/DLQ paralelos. |
| **T6** | `event-bus-audit` solo por contrato; purger sin open directo a Redb; purge histórico vía op administrativa. |
| **T7** | Retirar dependencias productivas de `sddia-event-store`, `StoreMode`, `SDDIA_EVENT_STORE`, filesystem/dual y `fs_event_adapter` productivo. |
| **T8** | `InMemoryEventManagerMock` + suite contractual; gate CI contra `EventStore::open`, `sddia_event_store`, modos legacy, open de `events.redb` y `fs::*` sobre `.events` fuera de allowlist. |

## Invariantes

- ECST inmutable; `delivery_state` no se modifica dentro del evento.
- Entregas exclusivamente en `subscriber_delivery`.
- DLQ, replay, purge, progreso y DLT solo por `EventManagerPort`.
- Un JSON físico conflictivo nunca prevalece sobre Redb.
- Ningún consumidor conoce rutas, tablas o tipos internos del adapter.
- Proofs y estado local no-evento pueden persistir en filesystem bajo rutas SSOT separadas.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | Cero uso productivo de `sddia_event_store` fuera del adapter o migración autorizada. |
| **AC-2** | Cero `StoreMode::Filesystem`, `StoreMode::Dual` o `SDDIA_EVENT_STORE` en consumidores. |
| **AC-3** | Cero lectura/escritura/rename/borrado de ECST bajo `.events` por consumidores. |
| **AC-4** | Eventos inline inexistentes o divergentes rechazados antes de efectos remotos. |
| **AC-5** | `route-domain` y `route-fractal` funcionan sin crear archivos `.events`. |
| **AC-6** | Kalma2, watchers, tools y repositorios publican/consultan mediante el puerto. |
| **AC-7** | Contract suite aprobada para Redb, mock y adapter externo. |
| **AC-8** | `cargo test --workspace` y gate arquitectónico verdes. |
| **AC-9** | README, Constitución, `cumulo.paths` y evolution convergen con el corte. |
| **AC-10** | `validacion` global APTO únicamente tras demostrar los invariantes anteriores. |

## Fuera de alcance

- Reimplementar el store Redb desde cero (se reutiliza/adapta el adapter existente).
- Tokenizar o publicar cápsulas NFT reales (el slot externo ya existe; no es meta de este corte).
- Migrar masiva de JSON históricos a Redb como cola (residuos = fractura/admin, no dual-mode).
- Cambiar el modelo de proofs/config/estado local no-evento (permanecen en FS SSOT).
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-35 / MAN-33; documento PBI-176f9f2f.
- Capacidad `eda:event-manager` y contrato `eda.event_manager` (base v1.0 → extensión 1.2).
- Norma `features-documentation-pattern` v1.3.1.
- `adapters-contract` / RedbEventAdapter existente.
- Suite contractual + mock en memoria.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `refactorization`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
