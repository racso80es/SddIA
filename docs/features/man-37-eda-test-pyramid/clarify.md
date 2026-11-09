---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
purpose: Estabilización de alcance — pirámide de tests EDA; unitarios con InMemoryEventManagerMock vía EventManagerTestHarness RAII; backends reales solo en contract/integration/e2e
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
pbi_ref: MAN-37
parent_ref: MAN-35
document_id: PBI-94396a84
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
execution_id: "1718099f-ca1b-44cb-8fb4-5a4dcccb98fe"
tracker_channel: ".SddIA/state/tracker-outbound/MAN-37.jsonl"
---
# Clarificación — man-37-eda-test-pyramid

Transcript Mayeuta (2026-11-09). Fase **Estabilización de alcance** del proceso `refactorization`.
Canal tracker: **MAN-37** (padre **MAN-35**). Documento forja **PBI-94396a84**. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-37** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-37.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `refactorization`, 2026-11-08…09). |
| Padre | **MAN-35** (`event-manager-port-cutover`) — cutover productivo a `EventManagerPort`; este PBI cierra la **pirámide de tests** sobre ese puerto. |
| Documento | **PBI-94396a84** (`.SddIA/state/forge-pbi/e9619407475a3d7ef01ec6189289255051514411c5404f7c5b5ecd8de66ac28f.json`). |
| Cuerpo Linear en disco | **No materializado**: `.SddIA/lab-linear-store.json` tiene `issues: {}`; el jsonl outbound no vuelca descripción. **No bloquea**: semilla de orquestación + canal + laudos bastan. |
| Semilla estabilizada | Unitarios EDA → `InMemoryEventManagerMock` vía `EventManagerTestHarness` (RAII); fallos con `FaultInjection`; Redb/FS/host/CLI/cápsulas externas → contract/integration/e2e; inventariar/migrar unitarios `execute-process` y consumidores EDA; retirar backend real de unitarios salvo allowlist; gate CI; taxonomía documentada. |
| Preexistente en puerto | `InMemoryEventManagerMock` + `FaultInjection` (`fail_write`, `steal_lease`, `duplicate_ack_noop`) en `sddia-core` ports; **`EventManagerTestHarness` aún no existe** → forja en alcance. |
| Modo | `branch_pr` / `refactorization` — tajos T0…T6 verificables. |

## D1 — Matriz PBI × alcance estabilizado

| Afirmación (semilla / PBI) | Laudo |
|---------------------------|-------|
| Todo unitario que use eventos instala mock vía harness RAII | **L-HARNESS** — `EventManagerTestHarness` instala `InMemoryEventManagerMock` como `EventManagerPort` al entrar y restaura al salir (Drop). |
| Fallos publicación / lease robado / ACK duplicado | **L-FAULTS** — solo vía `FaultInjection` del mock; sin I/O real ni race de locks. |
| Redb, FS, host, CLI, cápsulas externas | **L-TAXONOMY** — viven en **contract / integration / e2e** aislados; **prohibidos** en `#[cfg(test)]` unitarios de consumidores salvo allowlist. |
| Inventario + migración `execute-process` y consumidores EDA | **L-INVENTORY** — lista cerrada de crates/módulos; cada unitario migrado o reclasificado. |
| Retirar `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode`, inspección `.events/*` | **L-STRIP** — fuera de unitarios (salvo allowlist de adapter/suite contractual). |
| Gate CI anti-backend-real en unitarios | **L-GATE** — rechazo a patrones de backend real fuera de allowlist. |
| Documentar taxonomía unit/integration/e2e | **L-DOCS** — norma o sección canónica en `persist_ref` + README/Constitución según plan. |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Frontera de test unitario EDA = **`EventManagerPort`** + mock en memoria. El físico Redb/FS no es dependencia de unitarios de consumidores. |
| **L-HARNESS** | API canónica: `EventManagerTestHarness` RAII. Todo test unitario que toque eventos **debe** instalar el mock mediante el harness (no wiring ad hoc disperso). |
| **L-FAULTS** | Escenarios de error de bus en unitarios: `FaultInjection::{fail_write, steal_lease, duplicate_ack_noop}` (extensiones solo si el contrato lo exige y se documentan). |
| **L-TAXONOMY** | **Unit** = lógica + puerto mock, sin Redb/FS/host/CLI/cápsula. **Contract** = suite compartida sobre mock + adapters (tempdir). **Integration** = composition root / host con backend real efímero. **E2E** = CLI/cápsulas/procesos de extremo a extremo aislados. |
| **L-STRIP** | De unitarios de `execute-process` y consumidores EDA: retirar `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` e inspección de `.events/*` salvo allowlist explícita. |
| **L-ALLOWLIST** | Allowlist mínima y versionada: (1) crate/adapter Redb o FS bajo test contractual; (2) `run_event_manager_contract_suite` y peers; (3) tests marcados integration/e2e. Cualquier otra aparición de backend real en unitarios = **fractura de gate**. |
| **L-INVENTORY** | Tajo T1 produce inventario machine-readable (tabla en `plan.md`/`execution.md`) de unitarios a migrar; cero pendientes al cierre. |
| **L-GATE** | Gate CI (script o check en workflow) escanea unitarios fuera de allowlist y falla ante: `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode`, open de `events.redb`, I/O `.events/` en tests unitarios. |
| **L-DOCS** | Taxonomía unit/contract/integration/e2e documentada en este `persist_ref` (clarify + objectives; detalle operativo en plan/spec). |
| **L-NON-GOALS** | No reabrir cutover productivo MAN-35; no reintroducir dual/filesystem en runtime; no tokenizar NFT; no escribir TODOs Kaizen desde Tekton/Argos. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |

## D3 — Tajos canónicos (orden de forja)

| Tajo | Contenido estabilizado |
|------|------------------------|
| **T0** | Forjar `EventManagerTestHarness` RAII + guía de uso; cablear a `InMemoryEventManagerMock` / `FaultInjection`. |
| **T1** | Inventario de unitarios `execute-process` + consumidores EDA que tocan eventos/store. |
| **T2** | Migrar unitarios inventariados al harness; retirar APIs legacy listadas en **L-STRIP**. |
| **T3** | Reclasificar/aislar tests que requieran Redb/FS/host/CLI/cápsula → contract/integration/e2e. |
| **T4** | Gate CI anti-backend-real en unitarios + allowlist versionada. |
| **T5** | Documentar taxonomía unit/contract/integration/e2e (norma o sección canónica). |
| **T6** | Verificación: `cargo test` unitarios verdes sin backend real; contract suite; gate CI verde; `validacion` APTO. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Cuerpo Linear MAN-37 completo en disco? | No bloquea: canal + **PBI-94396a84** + semilla + laudos bastan (**D0**). |
| 2 | ¿El harness ya existe? | **No** en `sddia-core` ports hoy; es **T0** (forja), no prerequisito externo. |
| 3 | ¿Contract suite cuenta como unitario? | **No** para el gate: es **contract** (allowlist). Unitarios de consumidores no pueden abrir Redb aunque la suite sí. |
| 4 | ¿Inspección `.events/*` en tests de legacy paths? | Solo en integration/e2e o allowlist; unitarios aserten contra el mock/puerto (**L-STRIP**). |
| 5 | ¿Relación con MAN-35? | MAN-35 = cutover productivo; MAN-37 = **disciplina de tests** sobre el puerto ya canónico. |

## D5 — Veredicto

**ok** — Alcance estabilizado. Handoff a planificación/forja con tajos T0…T6, invariantes L-HARNESS/L-FAULTS/L-TAXONOMY/L-STRIP/L-GATE y AC 1…8 como contrato de Done. Tracker **MAN-37** / documento **PBI-94396a84** / padre **MAN-35**.
