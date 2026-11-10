---

feature_name: eda-store-lock-contention
created: "2026-11-10"
updated: "2026-11-10"
process: feature
purpose: Estabilización de requisitos — contención Redb (StoreError::Locked) con firma F-EDA-STORE-LOCK-CONTENTION (fracture_policy suppress), sin System_Fracture_Detected ni dead-letter; 1 Database + N tx sin reabrir MAN-38 ni subir presupuesto AUT-67
branch_name: feat/eda-store-lock-contention
persist_ref: docs/features/eda-store-lock-contention
pbi_ref: MAN-42
parent_ref: MAN-38
historia_ref: MAN-35
related_refs: ["AUT-67", "AUT-16", "MAN-40"]
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
fracture_signature: F-EDA-STORE-LOCK-CONTENTION
fracture_policy: suppress
delivery_on_contention: pending
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-42.jsonl"
execution_id: "4ca3514c-eb8e-4940-ba87-c0c2c7b1b98b"
---
# Clarificación — eda-store-lock-contention

Transcript Mayeuta (2026-11-10). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-42**. Origen semilla: kaizen `process_fix`. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-42** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-42.jsonl` (`forge-pbi` 2026-11-09 → `refine-item` / `tracker-stamp` → `lifecycle-phase-action` / `feature` 2026-11-10 ~15:15Z). |
| Cuerpo Linear / forge-pbi | **No materializado** en `.SddIA/state/forge-pbi/*` ni en `lab-linear-store.json` (solo operaciones outbound). **No bloquea**: semilla de orquestación + antecedentes MAN-38/AUT-67 + síntomas canónicos bastan (**L-TRACKER**). |
| Semilla orquestación | Contención de lock Redb (`Database already open` / `Cannot acquire lock` / `StoreError::Locked`) **no** debe emitir `System_Fracture_Detected` ni dead-letter. T1 firma + suppress; T2 ownership 1 Database + N tx. Métricas KA-LOCK-1..4. |
| Predecesores | **MAN-38** (lifetime scoped `with_productive_port` — **cerrado, no reabrir**); **AUT-67** (pre-push distingue `STORE_LOCK_CONTENTION`, reintento hook, **sin subir** presupuesto open); **AUT-16** (retry histórico store); **MAN-40** (sello lógico — adyacente, no reabrir). |
| Evidencia viva (store) | `sddia-event-store::EventStore::open` clasifica `DatabaseAlreadyOpen` / texto lock → `StoreError::Locked` tras agotar `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` (default en código: `DEFAULT_OPEN_BUDGET_MS = 10_000`). Comentario legado «2000» ≠ valor actual; **prohibido subir** el default como remedio (**L-NO-BUDGET**). |
| Evidencia viva (host) | MAN-38: ownership scoped + telemetría `open_contention`; retry host `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS` (default ~2 s) es escudo, no diseño. |
| Modo | `branch_pr` / `feature` — tajos T1–T2 (+ cierre métricas/validacion). |
| `execution_id` | **cd45182d-1075-43f1-89d5-f2fda26a09f5** |

## D1 — Matriz semilla × alcance estabilizado

| Afirmación (semilla) | Laudo |
|----------------------|-------|
| Lock contention ≠ fractura sistémica | **L-NOT-FRACTURE** — no emitir `System_Fracture_Detected` ante Locked / already open / cannot acquire lock. |
| Lock contention ≠ dead-letter | **L-NOT-DL** — no enrutar a dead-letter por causa de contención de fichero. |
| Firma canónica `F-EDA-STORE-LOCK-CONTENTION` | **L-SIGNATURE** — catálogo Mayeuta/ruta de fractura reconoce la firma. |
| `fracture_policy: suppress` | **L-SUPPRESS** — política suppress; no fan-out de fractura ni PBI nuevos por este síntoma. |
| Entrega permanece `pending` | **L-DELIVERY-PENDING** — el testigo/delivery del suscriptor afectado queda o vuelve a `pending` (reintento operativo), no `failed`/DL. |
| Un propietario `Database` + N transacciones | **L-ONE-DB-N-TX** — un solo poseedor del `redb::Database` por fichero; concurrencia vía N txs sobre ese owner, no N `Database::create`. |
| No reabrir MAN-38 | **L-NO-REOPEN-38** — lifetime scoped ya cerrado; este ciclo no rediseña `with_productive_port` / watcher scopes. |
| No subir presupuesto AUT-67 | **L-NO-BUDGET** — defaults `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` / `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS` no se incrementan como solución. |
| Métricas KA-LOCK-1..4 | **L-METRICS** — cuatro indicadores de cierre (tabla D3). |
| Fuera: doble `Database::create` | **L-NON-GOALS** |
| Fuera: sidechannel `F-REFINE-ITEM-REDB-LOCK` | **L-NON-GOALS** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Contención de lock del store Redb es **estado operativo transitorio**, no defecto ontológico ni fractura inédita. |
| **L-SYMPTOM** | Síntomas canónicos (OR): `StoreError::Locked`, `Database already open`, `Cannot acquire lock`, `DatabaseAlreadyOpen`. |
| **L-SIGNATURE** | Firma estable **`F-EDA-STORE-LOCK-CONTENTION`** en catálogo de firmas / clasificador de fractura. |
| **L-SUPPRESS** | `fracture_policy = suppress`: match → **no** emitir `System_Fracture_Detected`; **no** abrir ciclo Kaizen/PBI por este patrón. |
| **L-NOT-FRACTURE** | Emisores/handlers que hoy transforman Locked en `System_Fracture_Detected` deben clasificar+suprimir. |
| **L-NOT-DL** | Locked **no** escribe `dead_letter` / DLQ por contención de fichero. |
| **L-DELIVERY-PENDING** | Ante contención en claim/route/ACK: delivery permanece o se restaura a **`pending`** para reintento; no terminal fallido por lock. |
| **L-ONE-DB-N-TX** | Invariante: **1** owner `Database` por path de store; **N** transacciones (`begin_read`/`begin_write`) bajo ese owner. Prohibido «solucionar» con segundo `Database::create` concurrente sobre el mismo fichero. |
| **L-NO-REOPEN-38** | MAN-38 (scoped port, watcher claim/ACK, telemetría `open_contention`) es **precondición**, no alcance. No reescribir `port-lifetime.md` ni deshacer scoped ownership. |
| **L-NO-BUDGET** | AUT-67 / store: no subir defaults de presupuesto de open. Retry existente = escudo; suppress+pending = política de ruido. |
| **L-COMPAT-67** | Pre-push `STORE_LOCK_CONTENTION` (AUT-67) **se preserva**; MAN-42 cubre el plano EDA/fractura/delivery, no sustituye el hook. |
| **L-METRICS** | Cierre exige telemetría/checks **KA-LOCK-1..4** (ver D3). |
| **L-NON-GOALS** | Fuera: doble `Database::create`; sidechannel/firma `F-REFINE-ITEM-REDB-LOCK`; reabrir MAN-38/MAN-40/bóveda; cambiar EventManagerPort 1.2; rehabilitar FS/`SDDIA_EVENT_STORE` productivo. |
| **L-PROCESS** | Ciclo = `feature` / `branch_pr` (kaizen process_fix como origen). |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o `Kaizen_Alert_Required`. |

## D3 — Métricas KA-LOCK (cierre)

| ID | Qué mide | Criterio de cierre |
|----|----------|--------------------|
| **KA-LOCK-1** | Clasificación | Match de síntoma → firma `F-EDA-STORE-LOCK-CONTENTION` (evidencia en test/lab o log estructurado). |
| **KA-LOCK-2** | Silencio de fractura | Tras match: **0** emisiones `System_Fracture_Detected` atribuibles a Locked/already-open. |
| **KA-LOCK-3** | Silencio DL | Tras match: **0** dead-letters cuya causa sea contención de lock del store. |
| **KA-LOCK-4** | Ownership + presupuesto | 1 Database owner + N tx respetado; defaults de open budget **sin incremento** vs baseline AUT-67/test (`DEFAULT_OPEN_BUDGET_MS` store / host MAN-38). |

## D4 — Tajos canónicos (orden de forja)

| Tajo | Contenido estabilizado |
|------|------------------------|
| **T1** | Registrar/cablear firma **`F-EDA-STORE-LOCK-CONTENTION`** con `fracture_policy: suppress`; clasificador/ruta de fractura: match → suppress; delivery afectada → **`pending`**; sin `System_Fracture_Detected` ni DL. |
| **T2** | Afirmar y verificar invariante **1 Database + N transacciones** en paths productivos (host/store/consumidores calientes) **sin** reabrir diseño MAN-38 y **sin** subir presupuestos AUT-67; alinear telemetría existente (`open_contention` / hook) con KA-LOCK-4. |
| **T-close** | Tests/lab KA-LOCK-1..4 + `validacion.md` APTO (Dedalo/Tekton descomponen commits; Mayeuta fija el *qué*). |

## D5 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Cuerpo Linear / `document_id` en disco? | No bloquea (**L-TRACKER**); canal MAN-42 + semilla + antecedentes bastan. |
| 2 | ¿Reabrir MAN-38 scoped port? | **No** (**L-NO-REOPEN-38**). |
| 3 | ¿Subir open budget para «arreglar» lock? | **No** (**L-NO-BUDGET**). |
| 4 | ¿Locked es fractura o ruido operativo? | Ruido operativo con firma suppress (**L-DOMAIN** / **L-SUPPRESS**). |
| 5 | ¿Qué pasa con la entrega del suscriptor? | Queda/`pending` (**L-DELIVERY-PENDING**). |
| 6 | ¿AUT-67 pre-push se toca? | Solo compatibilidad; no reabrir defecto hook (**L-COMPAT-67**). |
| 7 | ¿Sidechannel refine-item `F-REFINE-ITEM-REDB-LOCK`? | **Fuera** (**L-NON-GOALS**). |
| 8 | ¿Default store 10_000 vs docs «2000»? | Baseline = **código + test AUT-67** (`DEFAULT_OPEN_BUDGET_MS`); no incrementar; drift documental ≠ alcance de subir presupuesto. |

## D6 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: firma `F-EDA-STORE-LOCK-CONTENTION` + `fracture_policy suppress` + delivery `pending`; cero `System_Fracture_Detected`/DL por Locked; invariante 1 Database + N tx sin reabrir MAN-38 ni subir presupuesto AUT-67; métricas KA-LOCK-1..4; fuera doble `Database::create` y `F-REFINE-ITEM-REDB-LOCK`. Tracker **MAN-42** / `execution_id` **cd45182d-1075-43f1-89d5-f2fda26a09f5**.
