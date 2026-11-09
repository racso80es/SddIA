---
feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
purpose: Objetivos estabilizados — configuración soberana por instancia para backends Event Manager (bóveda, precedencia, Cúmulo lógico)
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
parent_ref: MAN-35
document_id: PBI-0604fb3c
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-36.jsonl"
forge_key: "582d8a29c4f772596e7de259a8af20da3b0352e2f1354317c12574eadd31fa16"
execution_id: "4fc6eadd-78b9-4022-8991-ed39d911e9af"
---
# Objetivos — man-36-event-manager-instance-config

## Misión

Desacoplar la **configuración física** de los backends Redb y FS del Core: cada instancia resuelve proveedor y ruta desde su bóveda; Cúmulo conserva únicamente topología/identificadores lógicos ECST. El composition root `sddia-event-manager-host` valida y construye el adaptador; los consumidores siguen viendo solo `EventManagerPort` 1.2. Tracker **MAN-36** (padre **MAN-35**, documento **PBI-0604fb3c**).

**Por qué:** tras MAN-32/33/35 el puerto y el cutover Redb están asentados, pero el host sigue cableando `.SddIA/runtime/events.redb` (`events_redb_path`) y Cúmulo declara esa ruta en `event_bus` / `eda_*.store`. Esa mezcla acopla topología lógica a decisiones de despliegue y obliga a tocar código o `cumulo.paths.json` para reubicar el store.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | **MAN-36** (canal `.SddIA/state/tracker-outbound/MAN-36.jsonl`); padre **MAN-35** |
| Semilla / documento | **PBI-0604fb3c** (`refactorization`, `branch_pr`; forge sellada) |
| Predecesor | `event-manager-port-cutover` (MAN-35) — puerto 1.2 + Redb productivo |
| Adyacente | MAN-37 (pirámide tests) — no bloqueante |
| Deuda viva (revalidada) | `events_redb_path` cableado en host; stores físicos en `cumulo.paths.json` v1.11.1; `.dev/.env.example` aún menciona `SDDIA_EVENT_STORE` |
| Bóvedas | `env_hierarchy`: global `.dev/.env`, instancia `.SddIA/.dev/.env` |
| Clarificaciones | `clarify.md` (laudos L-* cerrados; veredicto **ok**) |
| Continuidad | Blueprint Dedalo (`spec.md` / `plan.md`) alineado a estos objetivos |
| `execution_id` | `4fc6eadd-78b9-4022-8991-ed39d911e9af` |

## Alcance (tajos T0–T5)

| Tajo | Objetivo |
|------|----------|
| **T0** | Contrato de variables + plantillas de bóveda sin secretos + precedencia documentada. |
| **T1** | Resolución/validación en `sddia-event-manager-host` (proveedor + rutas). |
| **T2** | Sustituir `events_redb_path` y desacoplar consumidores productivos del físico. |
| **T3** | Cúmulo lógico-only + gates alineados. |
| **T4** | README / Constitución / contratos EDA (límite Core/instancia). |
| **T5** | Matriz de tests (precedencia, rutas, contract suite) + `validacion` APTO. |

## Objetivos medibles

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Contrato de bóveda | Variables `SDDIA_EVENT_MANAGER_*` documentadas y plantilladas |
| **O2** | Host soberano | Host resuelve/valida; cero consumidores productivos abren Redb por ruta propia |
| **O3** | Cúmulo lógico | Cero stores físicos Redb/`.events/*` en `cumulo.paths.json` |
| **O4** | Política de rutas | Relativas acotadas a `repo_root`; absolutas válidas; inválidas fallan antes de open |
| **O5** | Precedencia | Tests SO > instancia > global verdes |
| **O6** | No regresión MAN-35 | Redb default productivo; sin fallback FS productivo |
| **O7** | Docs | README/Constitución/contratos describen el límite |
| **O8** | Cierre | `validacion.md` APTO; Linear MAN-36 según provider |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **CA-1** | Una instancia ubica Redb en ruta de bóveda (relativa o absoluta válida) sin cambio de código ni de `cumulo.paths.json`. |
| **CA-2** | El adaptador FS toma raíz solo del composition root y solo en test/migración. |
| **CA-3** | Cero rutas físicas Redb o `.events/*` como almacenamiento en `cumulo.paths.json`; rutas lógicas ECST no se interpretan como disco. |
| **CA-4** | Ningún consumidor productivo construye, abre o asume la ruta Redb; solo host/adaptador conocen el físico. |
| **CA-5** | Rutas inválidas (vacía, traversal `..`) fallan determinísticamente antes de abrir el backend; absolutas válidas se aceptan. |
| **CA-6** | Precedencia SO > bóveda de instancia > bóveda global cubierta por tests. |
| **CA-7** | Redb sigue siendo el default productivo; no hay fallback FS productivo ni regresión de MAN-35. |
| **CA-8** | README, Constitución y documentación de contrato describen el límite Core/instancia y las plantillas de bóveda. |

## Invariantes

- EventManagerPort 1.2 y semántica ECST **inalterados**.
- Precedencia de configuración: SO > instancia > global.
- Solo el composition root conoce proveedor/ruta física.
- Cúmulo = topología lógica; nunca store path de despliegue.
- Relativas no escapan de `repo_root`; absolutas no se reescriben.
- Sin secretos en Cúmulo, manifiestos o cápsulas; sin `.env` en cápsulas.

## Fuera de alcance

- Cambiar semántica ECST, tablas lógicas o protocolo EventManagerPort 1.2.
- Rehabilitar `SDDIA_EVENT_STORE`, modos dual/filesystem o acceso directo al store en consumidores.
- Persistir secretos en Cúmulo, manifiestos o configuración de cápsulas.
- Pirámide de tests EDA / harness mock (MAN-37).
- Semillas Kaizen en `docs/todos/` desde Tekton/Argos (solo `agent:cumulo` / `Kaizen_Alert_Required`).

## Dependencias

- MAN-36 / padre MAN-35 (`historia_ref`); documento PBI-0604fb3c.
- MAN-32 / MAN-33 (puerto, adaptadores, DI).
- Capacidad `eda:event-manager` / contrato `eda.event_manager` **1.2.0**.
- Composition root `sddia-event-manager-host`, `cumulo.paths.json`, bóvedas `.SddIA/.dev/.env` / `.dev/.env`.
- Norma `features-documentation-pattern` v1.3.1.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `refactorization`; Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Genoma: mutaciones solo vía `entity-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
