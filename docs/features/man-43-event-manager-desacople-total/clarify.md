---

feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
purpose: Estabilización de requisitos — desacople total EventManagerPort v2 / EventUri; cero topología runtime `.events/` ni dualismo Redb+FS
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
historia_ref: MAN-40
document_id: PBI-0f4f0f5f
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "2.0.0"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-43.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/c7976303-3b56-4175-9303-415e8b187623"
seed: continue-tekton
execution_id: "3654517b-c915-4e35-a965-3c7550a7b706"
---
# Clarificación — man-43-event-manager-desacople-total

Transcript Mayeuta (2026-11-11). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-43** (padre **MAN-40**). Documento semilla **PBI-0f4f0f5f**. Provider Linear: sin copia nueva bajo `docs/todos/`.
Semilla de esta pasada: **`continue tekton`** — revalidar el *qué* tras blueprint Dedalo; **no** reabrir laudos ni diseñar forja.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Semilla operador | **`continue tekton`** → requisitos ya estabilizados; confirmar handoff a Ejecución sin nuevas preguntas abiertas (**L-CONTINUE-TEKTON**). |
| Tracker `pbi_ref` | **MAN-43** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-43.jsonl` (`refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`; huellas hasta ~07:33Z). |
| Padre / historia | **MAN-40** (`man-40-eda-consumers-logical-seal`) — sello lógico PublishSeal/`event_id`; este ciclo **cierra el desacople total** (URI canónica, absorción de testigos/FS, demolición de legado). |
| Cadena | MAN-32 → MAN-33 → MAN-35 → MAN-36 → MAN-38 → MAN-40 → **MAN-43**. Meta parental: cerrar gap de desacople (R1 watcher FS, R2 store lab, R3 API física `eda_port`) → **`APTO_TOTAL`**. |
| Documento canónico | **PBI-0f4f0f5f** (`uuid` `0f4f0f5f-139a-476f-a9eb-9307d3c2493e`). Refine-item canónico: `.SddIA/workspaces/refine-item/c7976303-3b56-4175-9303-415e8b187623/` (`verdict: refined`, `open_questions: []`). |
| Canal LLM audit | `docs/audits/MAN-43.md` (interacciones LLM; **no** sustituye auditorías de desacople). |
| Auditorías desacople en disco | Paths citados `docs/audits/desacople-event-manager-man-32-20261110.md` y `docs/audits/legacy-pending-events-eda-20261110.md`: **ausentes** en esta pasada MCP (**L-AUDIT-DISK**). Inventario F-*/R*/L-P* permanece por refine + laudos + evidencia Dedalo en `spec.md` §0. |
| Forja sellada en disco | **Ausente** JSON `tracker_ref: MAN-43` bajo `.SddIA/state/forge-pbi/`. **No bloquea** estabilización; sí es gate pre-forja de código (**L-FORGE**). |
| Semilla orquestación | Frontmatter PBI declara `process: refactorization`; ciclo runtime actual = **`feature`** (`execution_id` 3278ec5b…). **L-PROCESS**: documentar y ejecutar como `feature` sin reabrir el defecto. |
| Cuerpo Linear en disco | **No materializado** en jsonl (solo operaciones). Cuerpo DoR leído de `item.refined.md` (**L-TRACKER**). |
| Residuo paralelo | Workspace refine `49bd8657-…` cita MAN-43 con `document_id` distinto (**PBI-29d1e254**). **L-DOC-CANON**: único documento de este ciclo = **PBI-0f4f0f5f**. |
| Blueprint Dedalo | `spec.md` / `plan.md` **v1.0.0** (`status: blueprint-ready`, H0–H6, `rbac_ok: true`). No abre vacíos de requisito nuevos (**L-CONTINUE-TEKTON**). |
| Modo | `branch_pr` / `feature` — hijos H1…H6 verificables. |
| `execution_id` | **3278ec5b-21ae-4fe0-af14-62a7e5ba0e07** |

## D1 — Matriz PBI × alcance estabilizado

| Afirmación (semilla / PBI) | Laudo |
|---------------------------|-------|
| URI canónica `eda://{event_family}/{event_id}`; familias solo `domain`/`telemetry`/`orchestration` | **L-1** |
| `event_id` opaco (string no vacío); sin UUID obligatorio | **L-2** |
| Eliminar `logical_rel`/`target_path` de `PublishSeal` ⇒ contrato mayor `eda.event_manager` **2.0.0**; sin dualidad productiva v1/v2 | **L-3** |
| Cutover atómico final: productores v2 → puerto v2 → consumidores v2; sin watcher huérfano | **L-4** |
| Cero `.events/` runtime (I/O, testigos FS, rutas con apariencia de fichero, fallback); excepciones genoma/instancia/docs históricos | **L-5** |
| Estado íntegro detrás del puerto; gaps 1.2 se incorporan a v2 antes de migrar consumidores | **L-6** |
| Demolición condicionada de `sddia-event-store` / `eda_port_legacy` | **L-7** |
| Seis hijos obligatorios (contrato, entregas/testigos, watcher/sweeper, periféricos, demolición, convergencia documental) | **L-CHILDREN** |
| AC-1…AC-14 = Done medible del epílogo | **L-AC** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Frontera productiva única = **`EventManagerPort` v2** / capacidad **`eda:event-manager`**. Identidad canónica = **`EventUri`**. El plano físico Redb/FS del adapter **no** es API de consumidores. |
| **L-1** | URI = `eda://{event_family}/{event_id}`. Familias admitidas: **solo** `domain`, `telemetry`, `orchestration`. `pending`/`claimed`/`processed`/`dead_letter` = **estados internos**, nunca familias URI. |
| **L-2** | `event_id` = string no vacío opaco. Tipo `EventUri { family: EventFamily, event_id: String }` con parseo/serialización/validación canónica. UUID **no** se impone sin migración/auditoría aparte. |
| **L-3** | Contrato mayor **`eda.event_manager` 2.0.0**: `PublishSeal.event_uri`; eliminación de `logical_rel` y `target_path`; schema v2, taxonomía/bindings, `version_gate` v2, suite mock/Redb/cápsula externa; rechazo explícito de consumidores v1. **Sin** dualidad productiva v1/v2. |
| **L-4** | Hijos forjables en paralelo **con** precondiciones; el runtime **no** puede quedar con watcher sin fuente. Cutover final atómico: productores v2 → puerto v2 → consumidores v2. |
| **L-5** | Al cierre: cero `.events/` en código productivo runtime; cero `read_dir`/`is_file`/escritura/borrado/creación asociados al bus; cero testigos FS; cero rutas lógicas con apariencia de fichero; cero fallback FS. Excepciones **permanentes**: `SddIA/events/` (genoma), `.SddIA/events/` (personalización de instancia). Docs/migradores históricos = eliminables, nunca runtime. |
| **L-6** | Publicación/consulta, claim/lease, ACK/NACK por suscriptor, listado de entregas, DLQ/replay, progreso, purga, consulta por familia/tipo/estado, barrido de terminales = **solo puerto**. Si 1.2 no cubre una operación, se añade a v2 **antes** de migrar consumidores. |
| **L-7** | `sddia-event-store` se elimina solo cuando: cero deps Cargo productivas/dev residuales de matrices, matrices migradas al puerto, `eda_port_legacy` vacío, suite v2 cubre Redb/mock/adaptador externo, y el gate detecta reintroducciones. |
| **L-CHILDREN** | Descomposición obligatoria en **seis hijos** (H1…H6); el epílogo parental no se declara Done sin ellos enlazados. |
| **L-SUPERSEDE-T0** | El token T0 `.events/{family}/{id}.json` (MAN-35/MAN-40) **deja de ser** sello/identificador canónico en runtime tras v2; lo sustituye **`EventUri`**. |
| **L-AUDIT-GAP** | Meta parental: cerrar gap de desacople (R1/R2/R3 + F-1…F-6 / legacy pending) → auditorías **`APTO_TOTAL`** (AC-13). |
| **L-AUDIT-DISK** | En esta pasada, los `.md` de auditoría desacople/legacy citados **no** están materializados bajo `docs/audits/`. **No** reabre L-1…L-7 ni bloquea Mayeuta; H6/Argos deben rematerializar o recuperar evidencia antes de declarar AC-13. Fuentes vigentes del *qué*: `item.refined.md` + laudos + `spec.md` §0. |
| **L-NON-GOALS** | No eliminar `SddIA/events/` ni `.SddIA/events/`; no mutar `docs/todos/pending/`; no exponer Redb fuera del host; no compat productiva con EventManager v1; no imponer UUID a `event_id` sin auditoría/migración específica. |
| **L-PROCESS** | Ciclo documental/runtime = **`feature`** / `branch_pr`; el origen PBI `refactorization` queda como metadato de forja, no como proceso de fase. |
| **L-DOC-CANON** | Documento canónico = **PBI-0f4f0f5f**; descartar `PBI-29d1e254` como fuente de este `execution_id`. |
| **L-FORGE** | Ausencia de JSON en `forge-pbi/` **no** bloquea Mayeuta; precondiciones de forge (inventario consumidores v1, ops nuevas, política purga, orden cutover, estrategia tests, `forge_key` si el runtime lo exige) quedan como **gates Dedalo/Tekton H0** antes de forja de código. |
| **L-CONTINUE-TEKTON** | Semilla `continue tekton` + blueprint `blueprint-ready`: **cero** preguntas abiertas nuevas; el *qué* no cambia; Ejecución consume `clarify.md`/`spec.md`/`plan.md` en solo lectura. Mayeuta **no** diseña H0–H6 ni emite `delegates_to`. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. Peaje dirty-worktree / paths fuera de `persist_ref` = gobernanza de proceso, **no** vacío de requisito. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |

## D3 — Hijos canónicos (orden de forja)

| Hijo | Contenido estabilizado | AC primarios |
|------|------------------------|--------------|
| **H1** | Contrato EventManager **2.0** + `EventUri` (`PublishSeal.event_uri`; eliminación `logical_rel`/`target_path`; schema, bindings, taxonomía, version gate; suite mock/Redb/externo; rechazo v1). | AC-1, AC-2, AC-3 |
| **H2** | Absorción de entregas y testigos: processing/processed/DLQ detrás del puerto; fin de escrituras en `eda_bus_topology`. | AC-5, AC-7 |
| **H3** | Watcher y sweeper sin filesystem: claim, listado, cierre y purga solo por puerto; sin doble despacho. | AC-4, AC-5, AC-6 |
| **H4** | Consumidores periféricos (`actions`, Kalma2, workspace-server, GitHub bridge) sin fallback ni emisión directa FS; retirar `runtime_db_path`/`path_is_resolved_redb` de consumidores. | AC-8, AC-9 |
| **H5** | Laboratorio y demolición: tests migrados; eliminación física de `eda_port_legacy` y `sddia-event-store`; allowlists legacy vacías. | AC-10, AC-11, AC-12 |
| **H6** | Convergencia documental: README, Constitución, contratos, procesos, auditorías → `APTO_TOTAL` (recuperar/rematerializar evidencia si falta en disco); `cumulo.paths.json` solo IDs/tablas/suscripciones lógicas; registro en `SddIA/evolution/`. | AC-13, AC-14 |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Proceso `refactorization` (PBI) vs `feature` (runtime)? | Documentar y ejecutar como **`feature`** (**L-PROCESS**). |
| 2 | ¿Cuerpo Linear MAN-43 completo en disco? | No bloquea: canal + **PBI-0f4f0f5f** + `item.refined.md` + laudos bastan (**L-TRACKER**). |
| 3 | ¿Falta `forge_key` en `.SddIA/state/forge-pbi/`? | No bloquea estabilización; sí es precondición de forja de código (**L-FORGE** / H0). |
| 4 | ¿Conviven v1 y v2 en producción? | **No** — cutover mayor sin dualidad productiva (**L-3** / **L-4**). |
| 5 | ¿El token `.events/...json` (MAN-40) sobrevive? | Solo como residuo histórico/docs; runtime canónico = **`EventUri`** (**L-SUPERSEDE-T0** / **L-5**). |
| 6 | ¿Imponer UUID a `event_id`? | **No** en este ciclo (**L-2** / **L-NON-GOALS**). |
| 7 | ¿Eliminar `SddIA/events/` o `.SddIA/events/`? | **No** — excepciones permanentes (**L-5** / **L-NON-GOALS**). |
| 8 | ¿Cuál es el documento canónico si hay refine paralelo? | **PBI-0f4f0f5f** (**L-DOC-CANON**). |
| 9 | ¿Ausencia de auditorías desacople/legacy en disco? | No bloquea el *qué*; H6/Argos recuperan evidencia antes de AC-13 (**L-AUDIT-DISK**). |
| 10 | ¿Blueprint Dedalo reabre requisitos? | **No** — `blueprint-ready` alinea forja a L-*; continuar a Tekton (**L-CONTINUE-TEKTON**). |

## D5 — Riesgos de cutover (vigilancia, no bloqueo Mayeuta)

| Riesgo | Tratamiento |
|--------|-------------|
| Consumidores externos aún en v1 | Gate `version_gate` v2 + rechazo explícito; inventario pre-forge (**L-3** / **L-FORGE**). |
| Operaciones insuficientes para sustituir sweeper | Ampliar puerto v2 **antes** de H3 (**L-6**). |
| Eventos residuales solo en `.events/` | Política de purga/migración definida pre-forge; higiene ≠ SSOT (**L-5** / **L-7**). |
| Cambio simultáneo de contratos CLI y procesos | H1 primero; H4/H6 alinean CLI/procesos tras sello v2 (**L-4**). |
| README/Constitución que aún describen `.events/` como topología | H6 obligatorio; AC-14. |
| Auditorías parentales ausentes en disco | Rematerializar/recuperar en H6 antes de AC-13 (**L-AUDIT-DISK**); no inventar `APTO_TOTAL`. |

## D6 — Veredicto

**ok** — Requisitos estabilizados y **reconfirmados** bajo semilla `continue tekton`. El *qué* permanece cerrado: `EventManagerPort`/`EventUri` v2 como única frontera; laudos **L-1…L-7**; hijos **H1…H6**; AC **1…14**; no-objetivos y riesgos de cutover documentados; sin reabrir bóveda MAN-36 ni compat v1. Blueprint Dedalo no abre vacíos. Nota explícita: auditorías desacople/legacy **no** leídas en disco esta pasada (**L-AUDIT-DISK**). Tracker **MAN-43** / documento **PBI-0f4f0f5f** / padre **MAN-40** / `execution_id` **3278ec5b-21ae-4fe0-af14-62a7e5ba0e07**.
