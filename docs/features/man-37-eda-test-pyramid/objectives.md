---





feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
purpose: Objetivos estabilizados — pirámide de tests EDA (harness RAII, FaultInjection, taxonomía, gate CI unitario)
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
pbi_ref: MAN-37
parent_ref: MAN-35
document_id: PBI-94396a84
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-37.jsonl"
spec_ref: "spec.md@1.2.0"
execution_id: "ceccf5b7-2243-4505-9387-ec50a3af2a04"
---
# Objetivos — man-37-eda-test-pyramid

## Misión

Establecer la **pirámide de tests EDA**: todo test **unitario** que use eventos instala `InMemoryEventManagerMock` como `EventManagerPort` mediante `EventManagerTestHarness` (RAII). Fallos de publicación, lease robado y ACK duplicado se ejercitan solo con `FaultInjection`. Redb, filesystem, host, CLI y cápsulas externas quedan en capas **contract / integration / e2e** aisladas. Inventariar y migrar unitarios de `execute-process` y consumidores EDA; retirar de esos unitarios `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` e inspección `.events/*` salvo allowlist. Gate CI unitario (complementario a MAN-35) que rechace backend real en unitarios fuera de allowlist. Documentar la taxonomía. Tracker **MAN-37** (padre **MAN-35**, documento **PBI-94396a84**).

**Por qué:** el cutover productivo (MAN-35) fija el puerto; sin disciplina de tests, los unitarios reacoplan Redb/FS/legacy y reintroducen fricción termodinámica (locks, I/O, no-determinismo). La pirámide protege el plano interno mockeable y aísla el coste de backends reales.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-37** (canal `.SddIA/state/tracker-outbound/MAN-37.jsonl`); padre **MAN-35** |
| Semilla / documento | **PBI-94396a84** (`refactorization`, `branch_pr`) |
| Predecesor | `event-manager-port-cutover` (MAN-35) — puerto productivo + gate que omite `#[cfg(test)]` |
| Mock existente | `InMemoryEventManagerMock` + `FaultInjection` en `sddia-core` (revalidado) |
| Harness | **Presente** en `eda_port::test_harness` (RAII + faults canónicos); requisito **O1/AC-1** = uso canónico en unitarios EDA |
| Gate unitario | **Presente** (`gate_eda_unit_no_real_backend` + `allowlists/eda-unit-backend.toml`); Done exige verde (**O6/AC-6**) |
| Blueprint | `spec.md` **v1.2.0** + `plan.md` T0…T6 en `persist_ref` |
| Taxonomía | Canónica en `spec.md` **§5** (unit / contract / integration / e2e) — **O5/AC-7** |
| Clarificaciones | `clarify.md` (laudos L-* cerrados; veredicto **ok**) |
| `execution_id` | `08a7564d-06ee-4558-8b37-9666572bcf00` |

## Alcance (tajos T0–T6)

| Tajo | Objetivo |
|------|----------|
| **T0** | Forjar `EventManagerTestHarness` RAII que instala/restaura `InMemoryEventManagerMock` como `EventManagerPort`; tres unitarios canónicos FaultInjection. |
| **T1** | Inventariar unitarios de `execute-process` y consumidores EDA que toquen eventos/store (migrate \| reclassify \| allowlist). |
| **T2** | Migrar unitarios al harness; retirar `sddia_event_store`, `EventStore::open`, `StoreMode`, `override_store_mode` e inspección `.events/*` (salvo allowlist). |
| **T3** | Aislar Redb/FS/host/CLI/cápsulas externas en contract/integration/e2e. |
| **T4** | Gate CI: rechazo de backend real en unitarios fuera de allowlist (no debilitar MAN-35). |
| **T5** | Documentar taxonomía unit / contract / integration / e2e. |
| **T6** | Verificación verde + `validacion` APTO. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Harness RAII canónico | `EventManagerTestHarness` disponible y usado por unitarios EDA |
| **O2** | Faults deterministas | `fail_write`, `steal_lease`, `duplicate_ack_noop` cubiertos en unitarios |
| **O3** | Inventario cerrado | Cero unitarios EDA pendientes de migración/reclasificación (allowlist + capa documentada = reclasificado) |
| **O4** | Strip legacy en unitarios | Cero hits de APIs **L-STRIP** fuera de allowlist |
| **O5** | Taxonomía documentada | Definición unit/contract/integration/e2e en `persist_ref` (`spec.md` §5) |
| **O6** | Gate CI unitario | Check verde rechazando backend real en unitarios no allowlisted |
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
| **AC-7** | Taxonomía unit/contract/integration/e2e documentada y referenciada (`spec.md` §5). |
| **AC-8** | `validacion` global APTO solo tras AC-1…AC-7. |

## Fuera de alcance

- Reabrir o revertir el cutover productivo MAN-35 / composición scoped MAN-38.
- Reintroducir modos `filesystem`/`dual` o `sddia-event-store` en runtime.
- Reimplementar RedbEventAdapter / store desde cero.
- Tokenizar o publicar cápsulas NFT.
- Migrar masiva de JSON históricos a Redb como cola.
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-37 / padre MAN-35; documento PBI-94396a84.
- Capacidad `eda:event-manager` / contrato `eda.event_manager` **1.2.0**.
- `InMemoryEventManagerMock`, `FaultInjection`, `eda_port::install_port`/`clear_port`, `run_event_manager_contract_suite`.
- Norma `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Blueprint: `spec.md@1.2.0` / `plan.md` T0…T6.
- Taxonomía operativa: `spec.md` §5.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `refactorization`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
