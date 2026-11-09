---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
pbi_ref: MAN-37
parent_ref: MAN-35
document_id: PBI-94396a84
capability_id: "eda:event-manager"
contract: "eda.event_manager"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
execution_id: "1718099f-ca1b-44cb-8fb4-5a4dcccb98fe"
---
# Objetivos — man-37-eda-test-pyramid

## Misión

Establecer la **pirámide de tests EDA**: todo test **unitario** que use eventos instala `InMemoryEventManagerMock` como `EventManagerPort` mediante `EventManagerTestHarness` (RAII). Fallos de publicación, lease robado y ACK duplicado se ejercitan solo con `FaultInjection`. Redb, filesystem, host, CLI y cápsulas externas quedan en capas **contract / integration / e2e** aisladas. Inventariar y migrar unitarios de `execute-process` y consumidores EDA; retirar de esos unitarios `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` e inspección `.events/*` salvo allowlist. Gate CI que rechace backend real en unitarios fuera de allowlist. Documentar la taxonomía. Tracker **MAN-37** (padre **MAN-35**, documento **PBI-94396a84**).

**Por qué:** el cutover productivo (MAN-35) fija el puerto; sin disciplina de tests, los unitarios reacoplan Redb/FS/legacy y reintroducen fricción termodinámica (locks, I/O, no-determinismo). La pirámide protege el plano interno mockeable y aísla el coste de backends reales.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-37** (canal `.SddIA/state/tracker-outbound/MAN-37.jsonl`); padre **MAN-35** |
| Semilla / documento | **PBI-94396a84** (`refactorization`, `branch_pr`) |
| Predecesor | `event-manager-port-cutover` (MAN-35) — puerto productivo |
| Mock existente | `InMemoryEventManagerMock` + `FaultInjection` en `sddia-core` |
| Harness | **Pendiente de forja** (`EventManagerTestHarness` RAII) |
| Clarificaciones | `clarify.md` (laudos L-*) |

## Alcance (tajos T0–T6)

| Tajo | Objetivo |
|------|----------|
| **T0** | Forjar `EventManagerTestHarness` RAII que instala/restaura `InMemoryEventManagerMock` como `EventManagerPort`. |
| **T1** | Inventariar unitarios de `execute-process` y consumidores EDA que toquen eventos/store. |
| **T2** | Migrar unitarios al harness; retirar `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` e inspección `.events/*` (salvo allowlist). |
| **T3** | Aislar Redb/FS/host/CLI/cápsulas externas en contract/integration/e2e. |
| **T4** | Gate CI: rechazo de backend real en unitarios fuera de allowlist. |
| **T5** | Documentar taxonomía unit / contract / integration / e2e. |
| **T6** | Verificación verde + `validacion` APTO. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Harness RAII canónico | `EventManagerTestHarness` disponible y usado por unitarios EDA |
| **O2** | Faults deterministas | `fail_write`, `steal_lease`, `duplicate_ack_noop` cubiertos en unitarios |
| **O3** | Inventario cerrado | Cero unitarios EDA pendientes de migración/reclasificación |
| **O4** | Strip legacy en unitarios | Cero hits de APIs **L-STRIP** fuera de allowlist |
| **O5** | Taxonomía documentada | Definición unit/contract/integration/e2e en `persist_ref` |
| **O6** | Gate CI | Check verde rechazando backend real en unitarios no allowlisted |
| **O7** | Suite contractual intacta | `run_event_manager_contract_suite` (y peers) siguen en capa contract |
| **O8** | Cierre documental | `validacion.md` APTO; Linear MAN-37 en estado done según provider |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | Todo unitario que use eventos pasa por `EventManagerTestHarness` → `InMemoryEventManagerMock`. |
| **AC-2** | Unitarios demuestran fallos de publicación, lease robado y ACK duplicado vía `FaultInjection`. |
| **AC-3** | Ningún unitario de `execute-process`/consumidores EDA usa `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` ni inspecciona `.events/*` fuera de allowlist. |
| **AC-4** | Redb, FS, host, CLI y cápsulas externas solo en contract/integration/e2e aislados. |
| **AC-5** | Inventario T1 cerrado: migrados o reclasificados sin pendientes. |
| **AC-6** | Gate CI falla ante backend real en unitarios no allowlisted. |
| **AC-7** | Taxonomía unit/contract/integration/e2e documentada y referenciada. |
| **AC-8** | `validacion` global APTO solo tras AC-1…AC-7. |

## Fuera de alcance

- Reabrir o revertir el cutover productivo MAN-35.
- Reintroducir modos `filesystem`/`dual` o `sddia-event-store` en runtime.
- Reimplementar RedbEventAdapter / store desde cero.
- Tokenizar o publicar cápsulas NFT.
- Migrar masiva de JSON históricos a Redb como cola.
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-37 / padre MAN-35; documento PBI-94396a84.
- Capacidad `eda:event-manager` / contrato `eda.event_manager`.
- `InMemoryEventManagerMock`, `FaultInjection`, `run_event_manager_contract_suite`.
- Norma `features-documentation-pattern` v1.3.1.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `refactorization`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
