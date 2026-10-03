---
document_id: HU-LINEAR-SYNC-FLOW
title: "HU - [OPERATIVO] Ampliación de Cápsula Linear: Sincronización de Flujos SddIA"
format: markdown
version: "2.1.0"
created: "2026-10-03"
refined: "2026-10-03"
status: "particionada"
priority: "alta"
process: "feature"
base: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
children:
  - document_id: HU-LINEAR-DIRECT-CYCLE
    hu_sequence: 1
    path: "docs/todos/historias/HU - [OPERATIVO] Linear — ciclo directo de HU y PBI.md"
    scope: "F0–F6; D-A, D-B, D-C, D-D, D-E, D-J"
    pbi_order: "PBI-LINEAR-A-01-CONTRACT … PBI-LINEAR-A-09-E2E"
  - document_id: HU-LINEAR-SSOT-INVERSE
    hu_sequence: 2
    path: "docs/todos/historias/HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done.md"
    scope: "F7–F8; D-F, D-G, D-H, D-I"
    blocked_by: HU-LINEAR-DIRECT-CYCLE
    pbi_order: "PBI-LINEAR-B-01-OUTBOUND … PBI-LINEAR-B-06-E2E"
related:
  - SddIA/core/cumulo.paths.json
---

# [OPERATIVO] Ampliación de Cápsula Linear: Sincronización de Flujos SddIA

**Documento padre, particionado el 2026-10-03 por laudo del Vértice.** El alcance, decisiones, criterios de aceptación y prerrequisitos viven en las dos HU hijas. Este documento conserva el historial de refinamiento (§0) y el contrato de partición (§1). No se forjan PBIs desde este `document_id`.

## 0. Refinamiento (afirmaciones contra el repo)

### 0.1 v1.1.0 — afirmaciones de la v1.0.0

| ID | Afirmación v1.0.0 | Realidad en el repo | Resolución |
|----|-------------------|---------------------|------------|
| R1 | «Ampliar la funcionalidad de la cápsula de Linear» | La cápsula `linear-tracker-adapter` es **ciega**: ejecuta GraphQL (`fetch_issue`, `list_issues`, `update_issue_state`, `create_comment`, `update_issue_description`). Toda decisión de transición vive en el proceso `tracker-stamp` (handler nativo) disparado por suscripciones EDA. | La HU muta sobre todo **procesos y eventos**. La cápsula gana `create_issue` (D-A). |
| R2 | «Sincronización bidireccional» | La HU base dejó la sincronización inversa y los webhooks **fuera de alcance**. Los flujos v1.0.0 solo describían SddIA → Linear. | v1.2.0 (laudo del Vértice): dirección inversa incorporada como F7. |
| R2b | (implícito) Linear como gate de Done | Laudo spike `tracker-done-gate`: **no migrar**; Git es el gate. Condicionó revisitar al E2E mock del ciclo completo. | v1.2.0 (laudo del Vértice): migración incorporada como F8. La condición del spike se mantiene como prerrequisito. |
| R3 | Variable de entorno `LINEAR_PROJECT` | Configuración **por proyecto** en `{project_root}/.SddIA/project.md` (`tracker.provider`, `team_key`, `project_id`, `state_map`, `labels.*`; `project-config-contract` 1.2.0). Env vars solo para secretos. «Ceguera espacial» = los agentes no conocen rutas absolutas ni inventan negocio. | Se usa `tracker.project_id`. No se crea `LINEAR_PROJECT`. |
| R4 | «La cápsula inyecta prefijos `{alias}-{item}`» | Linear fija el identificador `TEAMKEY-n`. Una label no es un prefijo. La cápsula no asigna ni valida labels (L13–L16, D2 de la HU base). | Taxonomía de **labels** configurables (`tracker.labels.*`). Con D-A, las asigna `create_issue` en el momento de la creación (previsto en L13). |
| R5 | Label `kaicen-{item}` | Grafía del repo: **kaizen**. | `kaizen`. |
| R6 | Label `IT-{item}` = Deuda Técnica | No existe `IT`. Convención `[DEUDA]`. | `deuda`. |
| R7 | Seis estados «homologados numéricamente» | Claves canónicas: `backlog`, `in_progress`, `in_review`, `done`, `cancelled` (sin `todo`, sin numeración). `state_map` traduce clave → `WorkflowState`. | Se añade `todo` (bump 1.3.0). Numeración eliminada. |
| R8 | Done = «en Producción» | Done = PR fusionado en `default_branch` + `validacion.md` APTO + PBI en `done/`. SddIA no rastrea despliegue. | Done = consolidado en `default_branch`. |
| R9 | «merge hacia master/main» | Rama destino = `default_branch` del manifiesto. | `default_branch`. |
| R10 | «Flujo sin PR finaliza desde In Progress» | `trunk_direct` emite `Delivery_Committed` sin `tracker_ref`; `tracker-stamp` no lo suscribe. | F5. |
| R11 | HU → In Progress «al forjar o al iniciar» | Hoy solo en `Work_Initiated` del primer PBI si la HU estaba en `backlog`. | D-C. |
| R12 | `related: SddIA/agents/cumulo.paths.json` | Ruta inexistente. SSOT: `SddIA/core/cumulo.paths.json`. | Corregido. |

### 0.2 v1.3.0 — afirmaciones del laudo `clarify.md` (D-A…D-I) contra el repo

| ID | Afirmación del laudo | Realidad en el repo | Ajuste |
|----|---------------------|---------------------|--------|
| L1 | D-A: «`forge-pbi` inyecta `tracker_ref` en el frontmatter al sellarlo» | `forge-pbi` tiene `context: [knowledge-management, filesystem-ops]` y dos fases `agent:` (Mayeuta, Argos). Para invocar `create_issue` necesita `tracker-operations` en su `context` y una fase `tool:` (doble cerrojo D4 de la HU base). | Se añade fase `tool:linear-tracker-adapter` «Registro en tracker» entre Recepción y Sellado; `forge-pbi` → 1.1.0 con `tracker-operations`. La restricción «los procesos de desarrollo no ganan `tracker-operations`» (L22–L27) sigue vigente: `forge-pbi` es forja, no desarrollo. |
| L2 | D-A: «erradicando la intervención manual como precondición» | `create_issue` de un PBI exige `parentId` = issue de la HU. Las HU en `docs/todos/historias/` no tienen proceso de forja: su issue seguiría creándose a mano. | La precondición manual solo desaparece si **`refine-hu` crea el issue de la HU** cuando el frontmatter carece de `tracker_ref` (D-J). Sin D-J, D-A solo automatiza PBIs. |
| L3 | D-A: «se amplía el contexto RBAC `tracker-operations`» | `execution-contexts.md` 1.2.0 §2.10: fuera de alcance «crear o borrar issues, administración de equipos, **webhooks**». | Bump a 1.3.0 vía `entity-manager`: entra «crear issues» (D-A) **y** «recibir webhooks firmados» (F7, D-F). Borrar issues y administrar equipos siguen excluidos. |
| L4 | D-B: «`emit-work-initiated-event` se desplaza al inicio de la fase Ejecución» | La emisión no está en los `.md` de proceso: la hace el **motor** en `workspace_init.rs` (handler nativo de la fase Inicialización). La fase Ejecución es `agent:tekton`. | Cambio de motor: el dispatcher emite `Work_Initiated` al **entrar** en la primera fase con `delegates_to: agent:tekton` (o fase marcada `emits: work-initiated`), no al cerrar init. D6 de la HU base queda superado. Payload inalterado (`branch`, `persist_ref` ya existen en ese punto). |
| L5 | D-C: «HU → `in_progress` al suscribir `Work_Initiated` del primer PBI hijo» | `maybe_promote_hu_from_backlog` solo promociona si la HU está en `backlog`. Con F1/D-D la HU estará en **`todo`** cuando arranque el primer PBI → no se promocionaría. | Condición ampliada: promocionar si HU ∈ {`backlog`, `todo`}. |
| L6 | D-D: «procesos `refine-hu` y `refine-pbi`» | No existen. `feature` ya tiene fases «Estabilización de Requisitos» (Mayeuta → `clarify.md`/`objectives.md`) y «Diseño de Blueprint» (Dédalo → `spec.md`/`plan.md`). `forge-pbi` ya tiene «Recepción» (Mayeuta expande la idea). | `refine-pbi` = refinamiento **pre-forja de código** del markdown del PBI (lo que hoy se hace a mano con una IA sobre `pending/`), distinto de las fases del proceso de desarrollo, que **no** cambian estado. `refine-hu` idem sobre `historias/`. Así `todo` se alcanza antes de `Work_Initiated` y desaparece la regresión de F3 v1.1.0. |
| L7 | D-G: «gestionado por `git-manager`» | `git-manager` es una **skill** (`SddIA/skills/git-manager.md`, 1.2.0), no una cápsula/tool. | Redacción corregida. |
| L8 | D-H: «sin este blindaje `objectives.md` o `spec.md` quedarían expuestos» | F7.3 solo sincroniza el markdown del PBI/HU resuelto por `tracker_ref`. La cascada F2 (`objectives.md`, `spec.md`, …) **nunca** es destino, con o sin label. | La label `editable` protege el **cuerpo del PBI/HU** (refinamientos del repo, §0 de este mismo documento), no la cascada. Rationale corregido. |
| L9 | D-I: «ausencia de eventos `TRACKER_STATE_DIVERGED`» | `TRACKER_STATE_DIVERGED` es un `error_code` dentro de `Tracker_Sync_Failed`, no un evento. | Redacción: «sin `Tracker_Sync_Failed` con `error_code: TRACKER_STATE_DIVERGED`». |
| L10 | D-F: «polling continuo» | Un daemon bajo `daemons-contract` opera por intervalo con `Daemon_Heartbeat`; «continuo» sugiere bucle sin espera. | Polling por intervalo configurable (default 60 s). |


## 1. Contrato de partición (laudo 2026-10-03)

| | **HU-A — `HU-LINEAR-DIRECT-CYCLE`** | **HU-B — `HU-LINEAR-SSOT-INVERSE`** |
|---|---|---|
| Objetivo | Linear refleja el ciclo completo sin intervención manual. | Linear es fuente de verdad; markdown réplica; Done leído desde Linear. |
| Alcance | F0, F1, F2, F3, F4, F4b, F5, F6. | F7, F8. |
| Decisiones | D-A, D-B, D-C, D-D, D-E, **D-J (a)**. | D-F, D-G, D-H, D-I. |
| `project-config-contract` | **1.2.0 → 1.3.0** (único bump): `state_map.todo`, labels extendidas incl. `editable` inerte, `done_gate` declarado con solo `git` aceptado. | Sin bump: habilita `linear|both` en el validador y consume `editable`. |
| `execution-contexts.md` §2.10 | **1.2.0 → 1.3.0**: crear issues. | **1.3.0 → 1.4.0**: recibir webhooks firmados. |
| Dependencia | Línea base actual. | HU-A mergeada y AC-7/AC-8 (E2E lab) verdes. |
| Riesgo | Bajo-medio. | Alto. |
| AC | AC-0a…AC-10. | AC-11…AC-23. |

Razones del laudo: HU-B depende del cierre de HU-A (condición del spike `tracker-done-gate`); HU-B reabre un laudo y toca normas `alwaysApply`; una HU única habría convivido con dos definiciones de Done incompatibles durante su propia ejecución.

## 2. Decisiones dictaminadas (resumen; detalle en las hijas)

| ID | Laudo |
|----|-------|
| D-A | (b) `create_issue` en cápsula 1.1.0; RBAC crear; `forge-pbi` 1.1.0 con fase `tool:`. |
| D-B | (a) `Work_Initiated` al entrar en la fase Ejecución (cambio de motor). |
| D-C | (a) HU → `in_progress` solo en `Work_Initiated` del primer PBI hijo; condición `backlog|todo`. |
| D-D | `refine-hu` → `HU_Refined`; `refine-pbi` → `PBI_Refined`; único trigger a `todo`. |
| D-E | (a) `warn` no-op sin `state_map.todo`; prohibido fallback. |
| D-F | (b) Polling por intervalo + webhook lab; sin endpoint público. |
| D-G | (a) Un PR diario/lote en `tracker-sync/{project_slug}` vía skill `git-manager`. |
| D-H | (a) Edición del cuerpo solo con `tracker.labels.editable`. |
| D-I | (a) `done_gate: linear` solo Core-self en el primer ciclo. |
| D-J | (a) `refine-hu` crea el issue de la HU si falta `tracker_ref`. |
