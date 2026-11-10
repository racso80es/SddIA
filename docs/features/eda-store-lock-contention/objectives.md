---



feature_name: eda-store-lock-contention
created: "2026-11-10"
updated: "2026-11-10"
process: feature
purpose: Objetivos estabilizados — contención Redb clasificada F-EDA-STORE-LOCK-CONTENTION (suppress), sin System_Fracture_Detected ni dead-letter; 1 Database + N tx sin reabrir MAN-38 ni subir presupuesto AUT-67
branch_name: feat/eda-store-lock-contention
persist_ref: docs/features/eda-store-lock-contention
pbi_ref: MAN-42
parent_ref: MAN-38
historia_ref: MAN-35
related_refs: ["AUT-67", "AUT-16", "MAN-40"]
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
fracture_signature: F-EDA-STORE-LOCK-CONTENTION
fracture_policy: suppress
delivery_on_contention: pending
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-42.jsonl"
execution_id: "5b4a78b0-9bca-4033-b460-da44927393dd"
---
# Objetivos — eda-store-lock-contention

## Misión

Cerrar el ruido operativo de contención de lock Redb (`Database already open` / `Cannot acquire lock` / `StoreError::Locked`): clasificarlo como firma **`F-EDA-STORE-LOCK-CONTENTION`** con **`fracture_policy: suppress`**, mantener la entrega en **`pending`**, y **no** emitir `System_Fracture_Detected` ni dead-letter. Afirmar el invariante **un propietario del `Database` y N transacciones** sin reabrir MAN-38 ni subir el presupuesto de apertura (AUT-67). Tracker **MAN-42** (kaizen `process_fix`; historia **MAN-35**, padre técnico **MAN-38**).

**Por qué:** tras MAN-38 (lifetime scoped) y AUT-67 (hook pre-push distingue contención), el plano EDA/fractura aún puede tratar Locked como fractura sistémica o DL. Eso regenera ruido Kaizen y enmascara contención transitoria como defecto ontológico.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-42** (canal `.SddIA/state/tracker-outbound/MAN-42.jsonl`); cuerpo Linear/`document_id` **no** en forge-pbi local — no bloquea |
| Semilla | Kaizen process_fix; síntomas Locked/already-open; T1 firma+suppress+pending; T2 1 DB + N tx |
| Predecesores | MAN-38 (scoped port — **no reabrir**); AUT-67 (hook `STORE_LOCK_CONTENTION` — **no subir budget**); AUT-16 (retry open); MAN-40 (sello lógico — adyacente) |
| Evidencia store | `EventStore::open` → `StoreError::Locked` tras presupuesto; `DEFAULT_OPEN_BUDGET_MS = 10_000` (baseline código/test AUT-67) |
| Evidencia host | `with_productive_port` + telemetría `open_contention` (MAN-38) |
| Clarificaciones | `clarify.md` (laudos L-* cerrados; veredicto **ok**) |
| `execution_id` | `cd45182d-1075-43f1-89d5-f2fda26a09f5` |

## Alcance (tajos T1–T2)

| Tajo | Objetivo |
|------|----------|
| **T1** | Firma `F-EDA-STORE-LOCK-CONTENTION` + `fracture_policy suppress`; match → sin `System_Fracture_Detected` ni DL; delivery → `pending`. |
| **T2** | Verificar/mantener 1 owner `Database` + N txs en paths productivos; sin reabrir MAN-38; sin incrementar open budgets AUT-67/store. |
| **T-close** | Métricas KA-LOCK-1..4 + `validacion` APTO. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Firma canónica | `F-EDA-STORE-LOCK-CONTENTION` reconocida por clasificador/catálogo |
| **O2** | Suppress | Match no emite `System_Fracture_Detected` |
| **O3** | Sin DL | Match no produce dead-letter por Locked |
| **O4** | Delivery | Contención deja/restaura delivery en `pending` |
| **O5** | Ownership | 1 Database + N tx; cero remedio vía segundo `Database::create` |
| **O6** | Presupuesto | Defaults open budget sin incremento (AUT-67 / store baseline) |
| **O7** | Métricas | KA-LOCK-1..4 evidenciada en tests/lab o telemetría |
| **O8** | Cierre | `validacion.md` APTO; Linear MAN-42 según provider |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA-1** | Ante `StoreError::Locked` / `Database already open` / `Cannot acquire lock`, el sistema clasifica **`F-EDA-STORE-LOCK-CONTENTION`** con **`fracture_policy: suppress`**. |
| **CA-2** | Ese match **no** emite `System_Fracture_Detected`. |
| **CA-3** | Ese match **no** escribe dead-letter / DLQ por contención de lock. |
| **CA-4** | La entrega del suscriptor afectado permanece o se restaura a **`pending`**. |
| **CA-5** | Paths productivos respetan **un** `Database` owner y **N** transacciones; no se introduce segundo `Database::create` concurrente sobre el mismo fichero. |
| **CA-6** | No se reabre el diseño de lifetime MAN-38 (`with_productive_port` / scopes watcher). |
| **CA-7** | No se sube el default de presupuesto de apertura (store `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` ni host `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS`) como remedio. |
| **CA-8** | Métricas **KA-LOCK-1..4** verificables en cierre (clasificación, cero fractura, cero DL, ownership+budget). |

## Métricas KA-LOCK

| ID | Métrica |
|----|---------|
| **KA-LOCK-1** | Match síntoma → firma `F-EDA-STORE-LOCK-CONTENTION` |
| **KA-LOCK-2** | 0 × `System_Fracture_Detected` por Locked |
| **KA-LOCK-3** | 0 × dead-letter por contención de lock |
| **KA-LOCK-4** | 1 Database + N tx; open budget defaults sin incremento |

## Invariantes

- EventManagerPort 1.2 y semántica ECST **inalterados**.
- MAN-38 scoped ownership **preservado** (no reabrir).
- AUT-67 hook `STORE_LOCK_CONTENTION` **preservado** (compatibilidad).
- Contención = ruido operativo suppressible, no fractura inédita.
- Un `Database` por path de store; concurrencia vía transacciones.
- Retry/open budget = escudo; no «solución» por inflación de presupuesto.

## Fuera de alcance

- Doble `Database::create` concurrente sobre el mismo fichero como remedio.
- Sidechannel / firma **`F-REFINE-ITEM-REDB-LOCK`**.
- Reabrir MAN-38 (lifetime), MAN-40 (sello lógico) o bóveda MAN-36.
- Subir defaults de `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` / `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS`.
- Cambiar contrato EventManagerPort 1.2 o rehabilitar FS/`SDDIA_EVENT_STORE` productivo.
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-42 (tracker); padre técnico MAN-38; historia MAN-35.
- AUT-67 (`docs/fixes/aut-67-prepush-store-lock`) — presupuesto + hook.
- AUT-16 / `docs/fixes/eda-redb-lock` — retry histórico.
- Host `sddia-event-manager-host` + crate `sddia-event-store`.
- Capacidad `eda:event-manager` / contrato `eda.event_manager` **1.2.0**.
- Norma `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
