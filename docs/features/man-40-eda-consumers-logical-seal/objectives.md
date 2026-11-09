---















feature_name: man-40-eda-consumers-logical-seal
created: "2026-11-09"
updated: "2026-11-09"
process: feature
purpose: Objetivos estabilizados — consumidores EDA sellan materialización con PublishSeal/event_id; cero path Redb ni canal FS `.events/*` como SSOT fuera del host
branch_name: feat/man-40-eda-consumers-logical-seal
persist_ref: docs/features/man-40-eda-consumers-logical-seal
pbi_ref: MAN-40
parent_ref: MAN-36
historia_ref: MAN-36
document_id: PBI-1b004e99
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-40.jsonl"
forge_key: "bd5f4db27f549584d61e515fe85571b8fb2d5f3279161d9aa982ff2851471498"
execution_id: "be6cdeb9-09cb-4eac-a0b6-8100846f5507"
---
# Objetivos — man-40-eda-consumers-logical-seal

## Misión

Cerrar el residuo **CA-4** de MAN-36: los consumidores productivos dejan de tratar el fichero Redb y el canal `.events/*` como SSOT de materialización. El sello canónico es el token lógico **`PublishSeal`** o el **`event_id`**; implementación y ruta física viven solo en **`sddia-event-manager-host`**. Tracker **MAN-40** (historia **MAN-36**, documento **PBI-1b004e99**).

**Por qué:** tras el cutover a `EventManagerPort` 1.2 (MAN-35) y la bóveda soberana (MAN-36), varios consumidores siguen devolviendo, registrando o detectando el path `.redb` / layout histórico / nombres FS cableados. Eso reintroduce acoplamiento al backend físico y rompe la frontera del composition root.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-40** (canal `.SddIA/state/tracker-outbound/MAN-40.jsonl`); padre/historia **MAN-36** |
| Semilla / documento | **PBI-1b004e99** (forja sellada; origen metadato `bug-fix`; ciclo runtime **`feature`**) |
| Predecesores | MAN-35 (cutover puerto 1.2); MAN-36 (bóveda + host soberano) — **no** se reabren |
| Síntoma vivo | `github_bridge::materialize_to_bus` publica por puerto y aún resuelve/devuelve `resolved_redb_path`; log `Materializado` imprime `.redb` |
| Inventario sello físico | bridge GitHub, `write_processing_witness`, `processing_header`, `eda_port::runtime_db_path` (+ helpers), detección `events.redb`, gate `sddia-event-store`, fallback audit, `productive_store_present` |
| Inventario FS/nombres | `event-watcher` (`read_dir`/`is_file`/claim fractal), `terminal_witness_exists`, hooks, installer, limpieza, lab forge-doc-drift |
| Contraste sano | linear_bridge con token lógico; writers fractal; `ecst_input_loader`; dead-letter local del bridge = fuera |
| Clarificaciones | `clarify.md` (laudos L-* cerrados; veredicto **ok**) |
| `execution_id` | `d5f40be9-1866-4813-a5b2-28a26faed56c` |

## Alcance (tajos T0–T6)

| Tajo | Objetivo |
|------|----------|
| **T0** | Sello lógico en materialización (GitHub bridge y análogos): return/log/cabecera = `PublishSeal`/`event_id`. |
| **T1** | Desacoplar `eda_port` / topología / `route_domain_core` + `terminal_witness_exists` solo puerto. |
| **T2** | Retirar fallbacks de layout y detección por nombre; sin modo por existencia de fichero. |
| **T3** | `event-watcher` 100 % por `EventManagerPort` (pending + familias); sin SSOT FS. |
| **T4** | Hooks / installer / limpieza / lab drift alineados a gobernanza host (sin `SDDIA_EVENT_STORE`). |
| **T5** | Gate arquitectónico + allowlist tests/legado. |
| **T6** | Tests de no-regresión + `validacion` APTO. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Sello lógico | Ningún consumidor productivo retorna/registra path Redb como sello de materialización |
| **O2** | Host único | Cero fallback layout `.SddIA/runtime/events.redb` / `DEFAULT_REDB_REL` fuera del host |
| **O3** | Detección soberana | Path de store solo vía API del host; sin comparar `events.redb` |
| **O4** | Watcher por puerto | Claim/orquestación sin `read_dir`/`is_file` como SSOT; fractal por puerto |
| **O5** | Terminal por puerto | `terminal_witness_exists` no cae a fichero si el puerto no marca terminal |
| **O6** | Scripts limpios | Hooks/installer/limpieza sin `SDDIA_EVENT_STORE` ni asunción `events.redb` único |
| **O7** | Gate | Veto a `resolved_redb_path` / `runtime_db_path` / `DEFAULT_REDB_REL` fuera de host+allowlist |
| **O8** | Cierre | `validacion.md` APTO; Linear MAN-40 según provider |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA-1** | Ningún consumidor productivo devuelve ni registra el path del store Redb como sello de materialización. |
| **CA-2** | Cero fallback de layout `.SddIA/runtime/events.redb` fuera de `sddia-event-manager-host`. |
| **CA-3** | Detección de path de store solo vía API del host, sin comparar el nombre `events.redb`. |
| **CA-4** | Gate veta `resolved_redb_path`, `runtime_db_path` y `DEFAULT_REDB_REL` fuera del host y del allowlist de tests/legado. |
| **CA-5** | `event-watcher` reclama pending, telemetría, dominio y orquestación por `EventManagerPort`; el cierre no usa `is_file()` ni `read_dir` de testigos como SSOT. |
| **CA-6** | `terminal_witness_exists` no cae a fichero si el puerto no marca terminal. |
| **CA-7** | Hooks, installer y scripts de limpieza no leen `SDDIA_EVENT_STORE` ni asumen `events.redb` como único store. |

## Invariantes

- EventManagerPort 1.2 y semántica ECST **inalterados**.
- Bóveda/precedencia MAN-36 **no** se reabre; vars siguen siendo `SDDIA_EVENT_MANAGER_*`.
- Solo el host conoce proveedor/ruta física y defaults de layout.
- Token `.events/{family}/{id}.json` = identificador lógico / `PublishSeal`, no afirmación de I/O.
- Contraste sano (linear_bridge, writers fractal, loader estricto) se preserva.
- Dead-letter local del github_bridge permanece fuera del backend del puerto.

## Fuera de alcance

- Reabrir configuración soberana, precedencia o plantillas de bóveda (MAN-36).
- Cambiar protocolo EventManagerPort 1.2 o semántica ECST.
- Rehabilitar `SDDIA_EVENT_STORE` / dual/filesystem productivo.
- Convertir el dead-letter local del bridge en backend del puerto.
- Pirámide de tests EDA completa (MAN-37), salvo gates/tests propios de este residuo.
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-36 (`historia_ref`) — bóveda y composition root.
- MAN-35 — cutover a `EventManagerPort` 1.2.
- Host `sddia-event-manager-host` — único dueño de `resolved_redb_path`, `is_resolved_redb_path`, `DEFAULT_REDB_REL`.
- Consumidores en inventario: `sddia-daemon-runtime`, `execute-process`, `event-watcher`, `event-bus-audit`, purger, `kalma2-bridge`, hooks, installer, lab forge-doc-drift.
- Capacidad `eda:event-manager` / contrato `eda.event_manager` **1.2.0**.
- Norma `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
