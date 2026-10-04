---
document_id: HU-LINEAR-DIRECT-CYCLE
parent_hu: HU-LINEAR-SYNC-FLOW
title: "HU - [OPERATIVO] Linear — ciclo directo de HU y PBI"
format: markdown
version: "1.2.0"
created: "2026-10-03"
status: done
closed: "2026-10-04"
merged_pr: 335
hu_sequence: 1
hu_sequence_total: 2
successor_hu: HU-LINEAR-SSOT-INVERSE
pbi_done: 9
pbi_total: 9
persist_ref: docs/features/linear-hu-a-09-e2e-lab
pr_url: "https://github.com/racso80es/SddIA/pull/335"
priority: "alta"
process: "feature"
base: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
blocks: HU-LINEAR-SSOT-INVERSE
related:
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/process/tracker-stamp.md
  - SddIA/process/tracker-sync-replay.md
  - SddIA/norms/execution-contexts.md
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - SddIA/library/codexes/codex-software-engineering/events/pbi-forged.md
  - SddIA/library/codexes/codex-software-engineering/events/delivery-committed.md
  - SddIA/events/domain/tracker-sync-failed.md
  - SddIA/core/cumulo.paths.json
---

# [OPERATIVO] Linear — ciclo directo de HU y PBI

Partición A de `HU-LINEAR-SYNC-FLOW` (§8 del documento padre, laudo 2026-10-03). Historial de refinamiento (R1–R12, L1–L10) en el padre.

## Orden

**HU 1 de 2 — cerrada.** Sucesora: `HU-LINEAR-SSOT-INVERSE` (HU 2 de 2), desbloqueada tras merge [#335](https://github.com/racso80es/SddIA/pull/335). Archivo en `docs/todos/done/historias/`.

| Orden | PBI | Entrega | Estado |
|------:|-----|---------|--------|
| 01 | `PBI-LINEAR-A-01-CONTRACT` | Contrato 1.3.0: `todo`, labels, `done_gate` solo `git`. | done |
| 02 | `PBI-LINEAR-A-02-CREATE-ISSUE` | Cápsula `create_issue` 1.1.0. | done |
| 03 | `PBI-LINEAR-A-03-RBAC-CREATE` | RBAC §2.10 → 1.3.0 (crear issues). | done |
| 04 | `PBI-LINEAR-A-04-EVENTS` | `PBI_Refined`, `HU_Refined`, `PBI_Cancelled`, `Delivery_Committed` 1.1.0. | done |
| 05 | `PBI-LINEAR-A-05-WORK-INITIATED` | Motor: `Work_Initiated` al entrar en Ejecución. | done |
| 06 | `PBI-LINEAR-A-06-REFINE` | Procesos `refine-hu` y `refine-pbi`. | done |
| 07 | `PBI-LINEAR-A-07-FORGE-PBI` | `forge-pbi` 1.1.0 registra el issue. | done |
| 08 | `PBI-LINEAR-A-08-STAMP` | `tracker-stamp` / `tracker-sync-replay`: `todo`, `cancelled`, `trunk_direct`. | done |
| 09 | `PBI-LINEAR-A-09-E2E` | E2E lab. Compuerta de la HU 2. | done |

Los nueve PBIs están en `docs/todos/done/`. Validación de cierre: `docs/features/linear-hu-a-09-e2e-lab/validacion.md` (`global: APTO`, `pbi_archived: true`).

## 1. Historia de usuario

**Como** Vértice Biológico operando desde Kalma2 y como agentes orquestadores,
**quiero** que cada HU y PBI nazca en Linear desde la forja y recorra el ciclo `backlog → todo → in_progress → in_review → done` (o `cancelled`) de forma determinista a partir de eventos de dominio, incluido `trunk_direct`,
**para** que Linear refleje el estado operativo real sin intervención manual, con Git como gate de Done (sin cambios en esta HU).

## 2. Línea base (no es alcance)

| Hito | Evento | PBI | HU madre |
|------|--------|-----|----------|
| `forge-pbi` sella el PBI | `PBI_Forged` | `backlog` + comentario `document_id` | — |
| Init `feature` / `bug-fix` / `refactorization` (motor, `workspace_init.rs`) | `Work_Initiated` | `in_progress` + comentario rama/`persist_ref` | `in_progress` si estaba en `backlog` |
| `delivery-close-cycle` abre PR | `PullRequest_Presented` | `in_review` + comentario `pr_url` | — |
| `pull-request-review` | `PullRequest_Audited` | comentario `resolution` | — |
| `accept-pr` fusiona | `PullRequest_Merged` | `done` + comentario merge SHA | `done` si todos los hijos `done` |

Fail-soft D5: error de Linear → `Tracker_Sync_Failed` → `tracker-sync-replay`.

## 3. Alcance

### F0 — Creación de issues desde la forja (D-A, D-J)

- Cápsula `linear-tracker-adapter` 1.0.0 → **1.1.0**: operación `create_issue`. Request: `team_key`, `title`, `description`, `labels[]` (nombres), `parent_ref` (opcional), `project_id` (opcional), `state_name` (opcional), `priority` (opcional). La cápsula resuelve `team_key → teamId`, nombres de label → `labelIds`, `parent_ref → parentId`, `state_name → stateId` (mecánica de lookup de `update_issue_state`; no decide negocio). Respuesta: `issue_ref` (`TEAMKEY-n`), `id`, `url`. Errores tipados nuevos: `LINEAR_LABEL_UNKNOWN`, `LINEAR_PARENT_NOT_FOUND`.
- `execution-contexts.md` 1.2.0 → **1.3.0** §2.10: el alcance gana «crear issues». Borrar issues, administrar equipos y **webhooks** siguen excluidos (webhooks → HU-B, 1.4.0).
- `forge-pbi` 1.0.0 → **1.1.0**: `context` gana `tracker-operations`; nueva fase `tool:linear-tracker-adapter` «Registro en tracker» entre Recepción y Sellado: `create_issue` con `labels = [tracker.labels.pbi, tracker.labels.{tipo}]`, `parent_ref = tracker_ref` de la HU (`historia_ref`), `state_name = state_map.backlog`, `project_id = tracker.project_id`. El `issue_ref` se inyecta como `tracker_ref` en el frontmatter que sella Argos. Sin `tracker` en el manifiesto → fase no-op. Linear caído → fail-soft D5: PBI sellado sin `tracker_ref`, `Tracker_Sync_Failed` con `operation: create_issue`. Sin `tracker_ref` en la HU madre → `LINEAR_PARENT_NOT_FOUND` tratado igual (fail-soft).
- `tracker-stamp` en `PBI_Forged`: la transición a `backlog` pasa a ser idempotente (el issue nace en `backlog`); se conserva el comentario `document_id`.
- **D-J (a):** `refine-hu` (F4) crea el issue de la HU (`labels = [tracker.labels.hu]`, sin `parent_ref`) cuando el frontmatter carece de `tracker_ref`, y escribe `tracker_ref` en el markdown de `historias/`. Con esto la cadena HU → PBI es íntegramente automática.

### F1 — Estado canónico `todo` (D-E)

- `project-config-contract` 1.2.0 → **1.3.0** (único bump; HU-B no vuelve a subirlo):
  - `tracker.state_map` admite `todo` (opcional).
  - `tracker.labels` extendida (F2), incluida `editable` **declarada pero inerte** en esta HU.
  - `tracker.done_gate ∈ {git, linear, both}` **declarado**; el validador de esta HU solo acepta `git` (default). `linear|both` → error de validación hasta HU-B.
- Semántica: `backlog` = pendiente de refinar; `todo` = refinado, listo para forjar código.
- Sin `todo` en `state_map`: `warn` no-op. Prohibido inventar transición o hacer fallback a `backlog`.
- `tracker-sync-replay` incorpora `todo` al orden del ciclo (`backlog < todo < in_progress < in_review < done`; `cancelled` terminal).

### F2 — Taxonomía de labels configurable

| Clave | Default | Uso |
|-------|---------|-----|
| `hu` | `hu` | HU (issue padre). Existente. |
| `pbi` | `pbi` | Todo PBI. Existente. |
| `fix` | `fix` | PBI con `process: bug-fix`. |
| `kaizen` | `kaizen` | PBI de mejora / evolución. |
| `deuda` | `deuda` | PBI de deuda técnica. |
| `spike` | `spike` | PBI de investigación. |
| `editable` | `sddia-editable` | Declarada aquí; consumida en HU-B (F7). |

- Las labels de tipo son **adicionales** a `pbi`: el preflight de `tracker-stamp` sigue exigiendo `tracker.labels.pbi`.
- Las asigna `create_issue` (F0). Issues preexistentes sin label → `warn` sin corrección (D2 de la HU base).
- `tracker-backlog-query` expone `items[].type_label` (opcional).

### F3 — Transición PBI `backlog → todo` (D-D)

- Nuevo proceso **`refine-pbi`** (vía `process-creator`; `context: [knowledge-management, filesystem-ops]`): inputs `pbi_ref`, `project_slug`. Fase Mayeuta refina el markdown del PBI en `todos_pending` (no toca `done/` ni cascada F2); fase Argos sella `status: refinado`, `refined: {fecha}` y emite **`PBI_Refined`** (vía `event-creator`). Payload ECST REQUIRED: `event_id`, `correlation_id`, `source_process: refine-pbi`, `pbi_ref`, `occurred_at`; OPTIONAL: `project_slug`, `tracker_ref`; FORBIDDEN: campos de proveedor.
- `tracker-stamp` suscribe `pbi-refined` → PBI a `todo` + comentario «PBI refinado: {document_id} v{version}».
- Las fases «Estabilización de Requisitos» y «Diseño de Blueprint» de `feature`/`bug-fix`/`refactorization` **no** emiten estado.

### F4 — Transiciones de HU (D-C, D-D, D-J)

- Nuevo proceso **`refine-hu`** (vía `process-creator`; `context: [knowledge-management, filesystem-ops, tracker-operations]`): inputs `hu_ref`, `project_slug`. Fases: Mayeuta refina el markdown en `historias/`; `tool:linear-tracker-adapter` crea el issue si falta `tracker_ref` (D-J); Argos sella `status: refinada` y `tracker_ref`, y emite **`HU_Refined`** (REQUIRED: `event_id`, `correlation_id`, `source_process: refine-hu`, `hu_ref`, `occurred_at`; OPTIONAL: `project_slug`, `tracker_ref`).
- `tracker-stamp` suscribe `hu-refined` → HU a `todo`.
- `backlog|todo → in_progress`: en `Work_Initiated` del primer PBI hijo. `maybe_promote_hu_from_backlog` amplía la condición de `backlog` a `backlog|todo` (L5). Forjar PBIs es planificación; no cambia estado de la HU (D-C).
- `→ done`: existente. Con F6, los hijos `cancelled` se ignoran en la evaluación.

### F4b — Semántica estricta de `in_progress` (D-B)

- Motor (`execute-process`): `Work_Initiated` deja de emitirse al cerrar «Inicialización de Espacio de Trabajo» (`workspace_init.rs`) y se emite al **entrar** en la primera fase con `delegates_to: agent:tekton` de `feature` / `bug-fix` / `refactorization` (L4). Payload idéntico.
- D6 de la HU base queda superado; registro en `SddIA/evolution/`.
- AC-21 de la HU base (no-op sin `tracker_ref`) se conserva.

### F5 — Cierre en `trunk_direct`

- `Delivery_Committed` 1.0.0 → **1.1.0** (vía `event-creator`): OPTIONAL `tracker_ref`, `pbi_ref`, `persist_ref`, `commit_sha`.
- `delivery-close-cycle` copia `tracker_ref` desde los inputs (mecánica D7).
- `tracker-stamp` suscribe `delivery-committed` → PBI `done` + comentario `commit_sha`; evaluación de la HU madre idéntica a `PullRequest_Merged`.

### F6 — Estado `cancelled`

- Nuevo evento **`PBI_Cancelled`** (vía `event-creator`); emisores: `task-queue-manager`, acción explícita desde Kalma2. (HU-B añade `tracker-markdown-apply` como emisor.) REQUIRED: `tracker_ref` o `pbi_ref`, `reason`.
- `tracker-stamp` → PBI a `cancelled` + comentario `reason`. La HU madre no cambia automáticamente.
- El markdown se mueve a `todos_done` con `status: cancelado`.

## 4. Decisiones (dictaminadas 2026-10-03)

| ID | Cuestión | Laudo |
|----|----------|-------|
| D-A | Creación de issues | **(b)** `create_issue` en cápsula 1.1.0; RBAC §2.10 → 1.3.0 (crear); `forge-pbi` 1.1.0 con fase `tool:` y `tracker-operations`. |
| D-B | Orden `todo` / `in_progress` | **(a)** `Work_Initiated` al entrar en la fase Ejecución (cambio de motor). |
| D-C | HU → `in_progress` | **(a)** Solo en `Work_Initiated` del primer PBI hijo; condición `backlog|todo`. |
| D-D | HU/PBI → `todo` | Procesos `refine-hu` → `HU_Refined`, `refine-pbi` → `PBI_Refined`; suscripción en `tracker-stamp` = único trigger a `todo`. |
| D-E | `state_map.todo` ausente | **(a)** `warn` no-op; prohibido fallback. |
| D-J | Issue de la HU | **(a)** `refine-hu` crea el issue si falta `tracker_ref`, label `hu`, guarda `tracker_ref` en `historias/`. |

## 5. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-0a | `create_issue` con labels, `parent_ref` y `state_name` devuelve `issue_ref`; label desconocida → `LINEAR_LABEL_UNKNOWN` sin mutación; `parent_ref` inexistente → `LINEAR_PARENT_NOT_FOUND`; sin `LINEAR_API_TOKEN` → `LINEAR_AUTH_MISSING`. | Test cápsula mock |
| AC-0b | `forge-pbi` con tracker sella el PBI con `tracker_ref` del issue creado (labels `pbi` + tipo, `parent` = HU, `project_id`); sin tracker → fase no-op; Linear caído → PBI sellado sin `tracker_ref` + `Tracker_Sync_Failed` `operation: create_issue`. | Test proceso mock |
| AC-0c | `refine-hu` sobre HU sin `tracker_ref` crea el issue con label `hu` y escribe `tracker_ref` en `historias/`; con `tracker_ref` presente no invoca `create_issue`. | Test proceso mock |
| AC-1 | Manifiesto 1.3.0 con `state_map.todo`, labels extendidas y `done_gate: git` valida; `done_gate: linear|both` → error de validación; manifiesto 1.2.0 sigue validando. | Test contrato |
| AC-2 | `PBI_Refined` con `tracker_ref` → `update_issue_state(todo)` + comentario; sin `tracker_ref`/sin tracker → no-op sin invocar cápsula; `state_map` sin `todo` → `warn` no-op. | Test `tracker-stamp` mock |
| AC-2b | `HU_Refined` → HU a `todo`. | Test `tracker-stamp` mock |
| AC-3 | `Delivery_Committed` 1.1.0 con `tracker_ref` → PBI `done` + comentario `commit_sha`; HU `done` si todos los hijos `done`. | Test `tracker-stamp` mock |
| AC-4 | `PBI_Cancelled` → PBI `cancelled`; HU con un hijo `cancelled` y resto `done` pasa a `done`. | Test `tracker-stamp` mock |
| AC-4b | `Work_Initiated` del primer PBI con HU en `todo` → HU `in_progress`; HU ya en `in_progress` → sin llamada de transición. | Test `tracker-stamp` mock |
| AC-4c | `Work_Initiated` se emite al entrar en la fase Ejecución y **no** al cerrar Inicialización; payload idéntico. | Test motor |
| AC-5 | Issue con label `fix` pero sin `pbi` → `warn` (preflight AC-18 de la HU base), Linear intacto. | Test mock |
| AC-6 | `tracker-sync-replay` descarta transición a `todo` si el issue ya está en `in_progress` o posterior. | Test mock |
| AC-7 | E2E lab `feature` + `branch_pr`: `refine-hu → forge-pbi → refine-pbi → backlog → todo → in_progress → in_review → done`, con comentarios de forja, refinamiento, rama, PR y SHA. HU: `backlog → todo → in_progress → done`. | Suite lab |
| AC-8 | E2E lab `trunk_direct`: `backlog → todo → in_progress → done`. | Suite lab |
| AC-9 | Ningún envelope ni evento contiene `LINEAR_API_TOKEN` ni cuerpo de respuesta de Linear. | Test existente ampliado |
| AC-10 | Sin `tracker` en el manifiesto el ciclo actual queda idéntico y verde. | Suite Core-self |

AC-7 y AC-8 en verde son la **condición de desbloqueo** de `HU-LINEAR-SSOT-INVERSE`.

## 6. Fuera de alcance

- Sincronización inversa, daemon sensor, webhooks, `done_gate: linear|both` → `HU-LINEAR-SSOT-INVERSE`.
- Borrar issues o administrar equipos/proyectos Linear.
- Creación de HU/PBI desde Linear hacia el repo.
- Numeración de estados o semántica de despliegue/producción.
- Otros proveedores (`tracker.provider` ≠ `linear`).
- Mutación manual de genoma: todo bump pasa por `entity-manager` / `event-creator` / `process-creator`.

## 7. Prerrequisitos

- Equipo Linear de prueba con `WorkflowState` para los seis estados canónicos y labels `hu`, `pbi`, `fix`, `kaizen`, `deuda`, `spike`, `sddia-editable`.
- `SddIA/evolution/`: superación de D6 de la HU base (emisión de `Work_Initiated`).

## 8. Estado de cierre (2026-10-04)

| Hito | Evidencia |
|------|-----------|
| Implementación 01–08 | PBIs en `docs/todos/done/`; features `linear-hu-a-01-contract` … `linear-hu-a-08-stamp`. |
| AC-7…AC-10 | `cargo test -p execute-process linear_direct_cycle_e2e`; `sddia-qa run-linear-direct-cycle-e2e-lab`. |
| Documental | `docs/features/linear-hu-a-09-e2e-lab/`; evolution `68e6ea3a-05c9-4236-85ad-66d452c19a93`. |
| Cierre | Merge [PR #335](https://github.com/racso80es/SddIA/pull/335); `HU-LINEAR-SSOT-INVERSE` habilitada para forja. |
