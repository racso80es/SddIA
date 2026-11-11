---
feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
purpose: Objetivos estabilizados — EventManagerPort v2 / EventUri única frontera; cero dualismo Redb+FS ni topología runtime `.events/`
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
historia_ref: MAN-40
document_id: PBI-0f4f0f5f
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "2.0.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-43.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/c7976303-3b56-4175-9303-415e8b187623"
execution_id: "3278ec5b-21ae-4fe0-af14-62a7e5ba0e07"
seed: continue-tekton
---
# Objetivos — man-43-event-manager-desacople-total

## Misión

Convertir **`EventManagerPort` v2** en la única frontera para existencia, identidad, estado, entrega, DLQ, progreso y purga de eventos, eliminando el dualismo Redb + filesystem y cualquier dependencia productiva de `.events/` como topología de runtime. Identidad canónica = **`EventUri`** (`eda://{family}/{event_id}`). Tracker **MAN-43** (padre **MAN-40**, documento **PBI-0f4f0f5f**).

**Por qué:** tras el cutover a puerto 1.2 (MAN-35), bóveda (MAN-36) y sello lógico (MAN-40), el runtime aún conserva descubrimiento/estado vía `.events/*`, doble materialización, testigos y barridos FS, identificadores con apariencia de ruta, API física Redb expuesta por `eda_port`, y `sddia-event-store` / puentes legacy en laboratorio. El gap parental de desacople permanece hasta este ciclo (meta AC-13 → `APTO_TOTAL`).

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-43** (canal `.SddIA/state/tracker-outbound/MAN-43.jsonl`); padre/historia **MAN-40** |
| Semilla / documento | **PBI-0f4f0f5f** (refine `verdict: refined`; origen metadato `refactorization`; ciclo runtime **`feature`**) |
| Semilla esta pasada | **`continue tekton`** — reconfirmar requisitos; handoff a Ejecución sin nuevas preguntas |
| Predecesores | MAN-32…MAN-40 — **no** se reabre bóveda/precedencia; sí se **supera** el token T0 `.events/...json` por `EventUri` |
| Gap parental | R1 watcher FS, R2 store lab, R3 API física — meta `APTO_TOTAL` (AC-13). Paths `docs/audits/desacople-event-manager-man-32-20261110.md` / `legacy-pending-events-eda-20261110.md` **ausentes en disco** esta pasada; inventarios vigentes vía refine + `clarify.md` + `spec.md` §0 |
| Inventario residual | F-1…F-6 / L-P* (actions doble write, Kalma2 fallback, workspace-server telemetría FS, github DL FS, sweeper, watcher) |
| Blueprint | `spec.md` + `plan.md` v1.0.0 `blueprint-ready` (Dedalo); no reabre el *qué* |
| Clarificaciones | `clarify.md` (laudos L-1…L-7 + L-CONTINUE-TEKTON / L-AUDIT-DISK; veredicto **ok**) |
| `execution_id` | `3278ec5b-21ae-4fe0-af14-62a7e5ba0e07` |

## Alcance (hijos H1–H6)

| Hijo | Objetivo |
|------|----------|
| **H1** | Contrato `eda.event_manager` **2.0.0** + `EventUri`; `PublishSeal.event_uri`; eliminar `logical_rel`/`target_path`; schema/bindings/taxonomía/`version_gate` v2; suite completa; rechazo v1. |
| **H2** | Absorber entregas y testigos detrás del puerto; fin de escrituras FS en `eda_bus_topology`. |
| **H3** | Watcher y sweeper 100 % por puerto; sin doble despacho ni `read_dir`/`is_file` como SSOT. |
| **H4** | Periféricos (`actions`, Kalma2, workspace-server, GitHub bridge) sin fallback/emisión FS; consumidores sin `runtime_db_path`/`path_is_resolved_redb`. |
| **H5** | Migrar matrices de test; demoler `eda_port_legacy` y `sddia-event-store`; allowlists legacy vacías; gates verdes. |
| **H6** | Convergencia documental + evolution; `cumulo.paths.json` solo IDs/tablas/suscripciones lógicas; auditorías → `APTO_TOTAL` (recuperar evidencia si falta en disco). |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Contrato v2 + `EventUri` | Suite contractual mock/Redb/externo verde; consumidores v1 rechazados |
| **O2** | Identidad opaca | `event_id` no UUID-mandatorio; familia vía enum |
| **O3** | API sin paths | Cero `logical_rel`/`target_path`/`event_file_path` en API pública |
| **O4** | Runtime sin `.events/` | Cero I/O/testigos/fallbacks FS del bus en código productivo |
| **O5** | Centinela por puerto | Watcher/sweeper solo operaciones del puerto |
| **O6** | Estado consultable | Entregas/DLQ/progreso/purga/estados vía puerto |
| **O7** | Periféricos limpios | Kalma2 / workspace-server / GitHub bridge sin FS; sin path Redb en consumidores |
| **O8** | Demolición | `sddia-event-store` + `eda_port_legacy` eliminados; allowlist legacy vacía |
| **O9** | Calidad | `cargo test --workspace`, clippy, contract suite y gates verdes |
| **O10** | Cierre parental | Auditorías `APTO_TOTAL` + evolution + docs convergentes |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | `EventManagerPort` v2 y `EventUri` con suite contractual completa. |
| **AC-2** | `event_id` permanece opaco; familia validada mediante enum. |
| **AC-3** | Ninguna API pública contiene `logical_rel`, `target_path` o `event_file_path`. |
| **AC-4** | Cero `.events/` en código productivo. |
| **AC-5** | Cero operaciones FS asociadas al runtime EDA. |
| **AC-6** | Watcher y sweeper operan exclusivamente por puerto. |
| **AC-7** | Toda entrega y estado es consultable por puerto. |
| **AC-8** | `runtime_db_path` y `path_is_resolved_redb` desaparecen de consumidores. |
| **AC-9** | Kalma2, workspace-server y GitHub bridge carecen de fallback FS. |
| **AC-10** | `sddia-event-store` y `eda_port_legacy` quedan eliminados físicamente. |
| **AC-11** | Gates con allowlist legacy vacía. |
| **AC-12** | `cargo test --workspace`, clippy, contract suite y gates verdes. |
| **AC-13** | Auditorías parentales pasan a `APTO_TOTAL` (con evidencia materializada; no inventar APTO). |
| **AC-14** | Cambio registrado en `SddIA/evolution/` y documentación constitucional convergente. |

## Invariantes

- Contrato productivo único tras cutover: **`eda.event_manager` 2.0.0** (sin dualidad v1/v2).
- URI canónica `eda://{domain\|telemetry\|orchestration}/{event_id}`; estados de cola **no** son familias.
- ECST y entregas solo detrás del puerto; Redb físico solo en el host/adapter.
- `SddIA/events/` y `.SddIA/events/` permanecen (genoma / personalización).
- Cutover final atómico: productores v2 → puerto v2 → consumidores v2.
- Demolición de `sddia-event-store` condicionada a L-7.
- Semilla `continue tekton` no altera el *qué*; H0 (`forge_key`, inventario, ops) es gate de forja, no de Mayeuta.

## Fuera de alcance

- Eliminar `SddIA/events/` o `.SddIA/events/`.
- Cambiar `docs/todos/pending/`.
- Exponer Redb fuera del host.
- Mantener compatibilidad productiva con EventManager v1.
- Imponer UUID a `event_id` sin auditoría y migración específica.
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).
- Reabrir bóveda/precedencia MAN-36.

## Dependencias

- Padre: **MAN-40** (`historia_ref`) — sello lógico previo; este ciclo lo supersede con `EventUri`.
- Seis hijos enlazados H1…H6 (contrato, entregas/testigos, watcher/sweeper, periféricos, demolición, convergencia documental).
- Precondiciones de forge: inventario completo de consumidores v1; decisión contractual v2 aprobada; operaciones nuevas del puerto especificadas; política de purga/migración de residuos; orden de cutover sin doble vía; estrategia de pruebas por hijo; `forge_key` si el runtime lo exige.
- Capacidad `eda:event-manager` / contrato `eda.event_manager` **2.0.0** (objetivo).
- Norma `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Blueprint de solo lectura para Tekton: `spec.md` / `plan.md`.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
