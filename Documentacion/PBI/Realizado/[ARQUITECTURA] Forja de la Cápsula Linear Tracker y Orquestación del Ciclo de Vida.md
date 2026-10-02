---
document_id: HU-SDDIA-TRACKER-LINEAR-001
uuid: "26209dff-e413-4c6d-8838-5b785251356c"
legacy_document_id: PBI-SDDIA-LINEAR-CORE-001
title: "[ARQUITECTURA] Tracker de requerimientos por proyecto — cápsula Linear, backlog en Kalma2 y sello de estado HU/PBI"
format: markdown
version: "2.4.0"
type: historia
status: cerrada
priority: alta
created: "2026-10-01"
updated: "2026-10-02"
closed: "2026-10-02"
finalized: "2026-10-02"
ubicacion: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
process_candidate: feature
decisions_status: "D1–D7 y D4.1 dictaminadas por Racso (2026-10-02)"
cierre_hu:
  fase: especificacion_y_desglose_pbi
  veredicto: pbi_autonomos_en_cola_pending
  cola_ejecucion_ssot: docs/todos/pending/
  prerrequisito_linear: "LINEAR_API_TOKEN en bóveda (.dev/.env) — 2026-10-02"
related:
  - SddIA/norms/capsule-json-io.md
  - SddIA/tools/tools-contract.md
  - SddIA/norms/execution-contexts.md
  - SddIA/agents/cerbero.md
  - SddIA/agents/tekton.md
  - SddIA/agents/argos.md
  - SddIA/engine/execute-process/src/engine/policy_validator.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
  - SddIA/engine/execute-process/src/core/resolver.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/library/codexes/codex-software-engineering/process/refactorization.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md
  - SddIA/library/norms/features-documentation-pattern.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/domain/pull-request-presented.md
  - SddIA/events/domain/pull-request-audited.md
  - SddIA/events/domain/pull-request-merged.md
  - SddIA/actions/emit-pr-presented-event.md
  - SddIA/daemons/event-sweeper.md
  - SddIA/process/kalma2-interact.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/app.js
historia_madre_ref: "docs/todos/done/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
spawned_pbis:
  - document_id: PBI-ARQUITECTURA-TRACKER-OPERATIONS-CONTEXT
    uuid: "12aeb72e-f7b4-4b39-ba1a-81fc19bed0a5"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — contexto RBAC tracker-operations.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-PROJECT-TRACKER-CONTRACT
    uuid: "ee3e2a61-c905-4e63-af24-7b816e597849"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — contrato de proyecto tracker.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER
    uuid: "87ac8a3b-a7e3-4a18-892d-9474fa073dda"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — cápsula linear-tracker-adapter.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-KALMA2-BACKLOG
    uuid: "20106dc7-1881-4d97-aaf0-9bce8353c339"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — backlog Kalma2.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-TRACKER-SYNC-FAILED
    uuid: "046e786b-3d0e-412d-aa3a-b4c28105d691"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — deuda de sincronización EDA.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-WORK-INITIATED
    uuid: "98d21320-e629-4c8e-a2bd-5406298468cc"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — evento Work_Initiated.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-ARQUITECTURA-TRACKER-STAMP
    uuid: "4bc88ff4-ac6b-441c-9c92-b7a7bf018173"
    path: "docs/todos/pending/[ARQUITECTURA] Tracker — proceso tracker-stamp.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
  - document_id: PBI-SPIKE-LINEAR-DONE-GATE
    uuid: "fb35cfa0-a7a8-485c-a87e-2a6bc934e9d3"
    path: "docs/todos/pending/[SPIKE] Tracker — migración del gate de Done a Linear.md"
    process: feature
    status: pending
    dispatch: true
    execution_mode: autonomo
    especificacion_cerrada: "2026-10-02"
---

# [ARQUITECTURA] Tracker de requerimientos por proyecto — cápsula Linear, backlog en Kalma2 y sello de estado HU/PBI

## 0. Filtro A

### 0.1 Refinamiento v2.0.0 (afirmaciones de la v1.0.0 contra el repo)

| # | Afirmación v1.0.0 | Realidad en el repo | Corrección aplicada |
|---|-------------------|---------------------|---------------------|
| A1 | El documento es un PBI (`PBI-…`, `process: feature`) | Reside en `docs/todos/historias/` y abarca varias entregas (cápsula, procesos, Kalma2, contrato de proyecto). | Se reclasifica como **historia** (`HU-…`, `type: historia`, `process_candidate`). Se desglosa en PBIs (§7). Id antiguo en `legacy_document_id`. |
| A2 | Frontmatter sin `uuid` | `.cursorrules` §2 exige `uuid` v4. | Añadido. |
| A3 | Contrato en `spec.md` / `spec.json` dentro de `SddIA/tools/linear-tracker-adapter/` | `spec.json` está prohibido (`.cursorrules` §2). Patrón real: definición `SddIA/tools/{name}.md` + crate `SddIA/tools/{name}/` + artefacto en `SddIA/target/` (`tools-contract` v1.6.0 §3). | Definición `SddIA/tools/linear-tracker-adapter.md`; crate `SddIA/tools/linear-tracker-adapter/`. |
| A4 | «Rust WASI o binario nativo» | `wasm32-wasip1` no tiene sockets. Las tools HTTP existentes (`gemini-http-infer`, `send-telegram-notification`) son binarios nativos con `ureq`. | **Binario nativo** con desviación §8 documentada (precedente: `sddia-workspace-server`). |
| A5 | `related: SddIA/process/delivery-close-cycle.md` | No existe. Vive en `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md`. | Ruta corregida. |
| A6 | `delivery-close-cycle` contiene la directriz «actualizar el archivo markdown» | Falso. DCC no toca el PBI. El movimiento `pending/ → done/` está en la fase **Cierre documental en rama** de `feature` (handler `feature-pbi-archive`) y `bug-fix`. | El sello de estado se asocia a las fases y eventos reales (§4.D). |
| A7 | «Actualizar el catálogo de Cerbero» | No hay catálogo. Cerbero es Read & Block: cruza el `context` de la cápsula destino con `allowed_policies` del agente. | Se resuelve con un contexto nuevo (D4, §4.E). |
| A8 | `LINEAR_API_KEY` en «la bóveda de la instancia» | La bóveda de instancia declara **`LINEAR_API_TOKEN`** (2026-10-02). En 1×N la precedencia es SO > proyecto (`env_ref`) > instancia > global. | La cápsula lee `LINEAR_API_TOKEN`. `LINEAR_API_KEY` no es SSOT. |
| A9 | «Erradicar Git como base de datos de estado» sin más | Choca con la Definición de Done vigente (`task-closure-documental.mdc`, `feature` v1.3.0, `bug-fix` v1.4.0). | Transición en dos tiempos: espejo activo en esta HU y spike para el gate (D1). |
| A10 | `update_issue_state` acepta un estado abstracto | En Linear el estado es un `WorkflowState` con id propio **por equipo**. | Acepta `state_id` o `state_name` + `team_key`. Si el nombre es ambiguo o no existe, devuelve un error tipado. |
| A11 | La funcionalidad es global | Kalma2 ya transporta `project_slug`. El manifiesto de proyecto no tiene configuración de tracker. | Todo se resuelve **por proyecto seleccionado** (§4.A). Bump del contrato a 1.2.0. |

### 0.2 Refinamiento v2.1.0 (afirmaciones del laudo contra el repo)

| # | Afirmación del laudo | Realidad en el repo | Corrección aplicada |
|---|----------------------|---------------------|---------------------|
| L1 | D1: el gate de Done impacta en **Argos** y en **`delivery-close-cycle`** | Ninguno de los dos lo aplica. El gate es el handler `feature-pbi-archive` del motor (`execute-process/src/engine/phase_capsules.rs`): exige `validacion.md` con `global: APTO` y `pbi_archived: true`, y mueve el PBI a `docs/todos/done/`, ruta **cableada** que ignora `docs_layout.todos_done`. `pull-request-review` (Argos) produce `validacion.md` pero no comprueba `done/`. | El spike apunta al handler del motor y a `pull-request-review`. La ruta cableada queda como hallazgo del spike. |
| L2 | D1: «si cortamos eso de golpe bloquearemos todos los PRs» | Retirar el gate no bloquea PRs: los dejaría pasar sin evidencia. El bloqueo aparece si el PBI vive solo en Linear y el gate sigue exigiendo un markdown en `done/`. | Redactado así en §3: mientras dure esta HU, todo PBI con `tracker_ref` conserva también su markdown. |
| L3 | D1: «Linear pasa a ser SSOT» y a la vez «no rompemos la validación local de Git» | Las dos cosas no pueden ser ciertas a la vez: el estado que decide el merge sigue siendo el de Git. | SSOT **objetivo**: Linear. SSOT **efectivo** durante esta HU: el gate de Git. Si discrepan, gana Git y la discrepancia se emite como `Tracker_Sync_Failed` (§4.F). |
| L4 | D3: sin tracker, Kalma2 indica que no hay cápsula configurada | Correcto. Consecuencia no explícita: hoy **ningún** proyecto (ni Core-self ni BarcelonaXplorer) tiene Linear, así que el panel saldrá vacío en todos hasta configurar uno. | AC-5 y AC-6 se reescriben. Se quita también el sello en frontmatter markdown de la v2.0.0: sin tracker, el ciclo actual queda intacto (no-op). |
| L5 | D4: con `system-operations` «le damos a Tekton permisos para casi todo» | Tekton **ya tiene** `system-operations` (`tekton.md`). Usar ese contexto no le añade nada a Tekton. El riesgo real es el inverso: **cualquier** agente con `system-operations` podría llamar a Linear, y Argos (que no lo tiene) no podría. | El contexto `tracker-operations` sí aplica menor privilegio, pero sobre la **tool**: solo la invocan los agentes que declaren ese contexto. |
| L6 | D4: añadir el contexto en `execution-contexts.md` basta para Cerbero | Correcto: `policy_validator.rs` parsea los contextos desde la norma en tiempo de ejecución, no hay lista cableada. El test exige `>= 9` contextos; con el nuevo serán 10. | Sin cambio de motor para reconocer el contexto. La norma y los agentes son genoma: se mutan **vía `entity-manager`**, no a mano. |
| L7 | D4 (implícito): el puente Kalma2 invoca la cápsula para listar | El puente no es un agente: lanza binarios con `Command::new` y no pasa por Cerbero. Llamar a la tool desde el puente se saltaría el RBAC que crea D4. | El listado entra por `execute-process` (proceso de lectura) con un agente solicitante con `tracker-operations`. Qué agente lo hace es la decisión D4.1. |
| L8 | D5: el evento «queda en `pending/` (Dead-letter)» | Son buzones distintos (`cumulo.paths.json` → `eda_bus`): `./.events/pending` es la entrada de eventos de dominio; `./.events/dead-letter` recibe lo que un suscriptor no pudo procesar. | `Tracker_Sync_Failed` se emite en `eda_bus.pending`. Solo llega a `dead-letter` si su reintento también falla. |
| L9 | D5: «un barredor de eventos (`event-sweeper`) lo sincronice» | `event-sweeper` es ciego («solo inyecta eventos físicos en el bus»): purga padres con consenso de suscriptores y alerta Kaizen ante `dead-letter`. No llama a Linear ni reintenta. | El reintento lo hace un suscriptor nuevo (proceso `tracker-sync-replay`). El sweeper conserva su papel: purgar y alertar. |
| L10 | D5: «el proceso no hace un `panic!`» | Los procesos son contratos que orquesta `execute-process`; el riesgo real es que una fase devuelva `success: false` y aborte la entrega. | La fase de sello del tracker se declara fail-soft: con error emite el evento, deja `warn` en el envelope y la entrega continúa. |
| L11 | D5 (implícito): reintentar la llamada fallida | Reproducir una transición vieja puede **hacer retroceder** un issue que ya avanzó (p. ej. aplicar `in_review` después de `done`) y duplicar comentarios. | El reintento consulta el estado actual y descarta transiciones obsoletas. Los comentarios llevan una marca de idempotencia (§4.F). |
| L12 | — | Un fallo de Linear no es un colapso de proceso oficial. | No se emite `System_Fracture_Detected` ni aplica el Protocolo Kintsugi. |

### 0.3 Refinamiento v2.2.0 (laudo D2 y D4.1 contra el repo)

| # | Afirmación del laudo | Realidad en el repo / en la HU | Corrección aplicada |
|---|----------------------|--------------------------------|---------------------|
| L13 | D2: la cápsula «asigna» las labels `hu` / `pbi` **al crear** | La cápsula no tiene operación de crear: crear issues está fuera de alcance (§6). | No aplica en esta HU. Si en el futuro se crea desde SddIA, la label va en esa operación. |
| L14 | D2: la cápsula asigna las labels **al leer** | Escribir durante una lectura convierte `fetch_issue` / `list_issues` en mutaciones, y decidir qué es HU o PBI es lógica de dominio, cosa que la ceguera operativa prohíbe (§4.B). | La cápsula no asigna ni exige labels. Las labels son **precondición**: las pone quien crea el issue en Linear. El proceso pasa el filtro de label a `list_issues`. Si un issue con `tracker_ref` no tiene la label esperada, el sello emite `warn` y no lo corrige. |
| L15 | D2: filtrar por label «consume muchos menos tokens» | El listado de Kalma2 no pasa por un LLM: no consume tokens. | El beneficio real es otro: el filtro se aplica en el servidor GraphQL, así que viaja menos payload, hacen falta menos páginas y se gasta menos cuota de complejidad (rate limit) de Linear. |
| L16 | D2 (implícito): los nombres `hu` / `pbi` son fijos | Un equipo Linear puede tener ya labels con esos nombres y otro significado. | Nombres configurables en `tracker.labels.hu` / `tracker.labels.pbi` (por defecto `hu` / `pbi`). |
| L17 | D4.1: «Cerbero ve que Tekton tiene `tracker-operations`» | Para una fase que delega directamente en `tool:…`, Cerbero no lee el genoma del agente. `resolve_requester_policies` (`cerbero_di_rbac.rs`) usa `target_executor_rbac.allowed_policies`, que es un input reservado al runtime (`resolver.rs` → `RUNTIME_INJECTED`) pero que hoy nadie rellena, o en su defecto el **`context[]` del proceso**. | El permiso efectivo del listado lo da `context: [tracker-operations]` en `tracker-backlog-query`. Tekton queda como **ejecutor de registro** del proceso. Que el runtime derive `target_executor_rbac` del genoma del agente de registro es mejora de motor fuera de alcance. |
| L18 | D4.1: «Tekton ejecuta la cápsula» | Si la fase delegara en `agent:tekton`, se despertaría su LLM para una lectura determinista: latencia y coste innecesarios, en contra del espíritu «I/O pura» del propio laudo. | La fase delega en `tool:linear-tracker-adapter` y la ejecuta el runtime sin LLM, como hacen `event-bus-audit` o `telegram-gateway`. |
| L19 | D4.1: «inyectamos `tracker-operations` en Tekton» | Ya estaba dictaminado en D4. | Sin acción nueva. |
| L20 | (consecuencia de L17) | El mismo mecanismo afecta al sello de §4.D: una fase `tool:linear-tracker-adapter` dentro de `feature` (`context` sin `tracker-operations`) sería bloqueada por Cerbero. | Toda invocación de la tool se concentra en procesos con `context` que incluya `tracker-operations` (`tracker-backlog-query`, `tracker-stamp`, `tracker-sync-replay`). Ver §4.D. |
| L21 | Coreografía: «el puente invoca al CLI» | Los procesos largos devuelven `detached: true` (DA-5). Un listado para UI necesita respuesta síncrona. | `tracker-backlog-query` debe completar en la misma invocación y devolver `items` en el envelope, como `kalma2-interact`. |

### 0.4 Refinamiento v2.3.0 (laudo D6 contra el repo)

| # | Afirmación del laudo | Realidad en el repo / en la HU | Corrección aplicada |
|---|----------------------|--------------------------------|---------------------|
| L22 | Los procesos de dominio «emiten un evento ciego» | En el repo ningún proceso escribe directamente en el bus: lo hace una acción `emit-*-event` (p. ej. `emit-pr-presented-event`, `context: ecosystem-evolution`, delega en `crypto-broker` y `filesystem-manager`). | Acción nueva `emit-work-initiated-event` con `context: ecosystem-evolution`, que ya cubren `feature`, `bug-fix` y `refactorization`. Esos procesos no ganan `tracker-operations`. |
| L23 | `Work_Initiated` como nombre | No colisiona con ningún evento existente. Sigue la convención `PascalCase_Con_Guiones` (`PullRequest_Presented`, `System_Fracture_Detected`). | Evento `Work_Initiated`, definición `SddIA/events/domain/work-initiated.md` vía `event-creator`. |
| L24 | «El proceso no necesita saber si el proyecto usa Linear, Jira o Markdown» | Correcto para el emisor. Matiz: por D3, markdown no es un proveedor de tracker; sin `tracker` en el manifiesto, `tracker-stamp` no hace nada (no-op). | El evento no lleva campos específicos de proveedor. `tracker_ref` viaja como cadena opaca. |
| L25 | `tracker-stamp` «orquestado por Tekton» | Igual que en L17–L18: el runtime ejecuta la fase `tool:` sin LLM y Cerbero valida contra el `context[]` del proceso. | Tekton como ejecutor de registro; `tracker-stamp` con `context: [tracker-operations]`. |
| L26 | (implícito) Con `Work_Initiated` el resto del ciclo ya está cubierto | Los eventos de PR no identifican el PBI: `PullRequest_Presented` exige solo `branch` y `status`; `PullRequest_Merged`, `source_branch` y `merge_commit_hash`. Ninguno lleva `pbi_ref`, `tracker_ref` ni `correlation_id` en su contrato. Sin eso, `tracker-stamp` no sabe qué issue pasar a `in_review` o `done`. | Decisión D7. |
| L27 | (implícito) `Work_Initiated` debe identificar la HU | No hace falta: `tracker-stamp` obtiene la HU con `fetch_issue` sobre el PBI (`parent`, D2). | El payload lleva solo la referencia del PBI. |

## 1. Historia de usuario

**Como** Vértice Biológico (Racso) operando desde Kalma2, y como agentes orquestadores (Tekton, Argos),
**quiero** consultar el backlog (HU y PBI) del proyecto seleccionado en Linear, y que los procesos de desarrollo reflejen automáticamente el estado y los datos de entrega en esas HU y PBI mediante la cápsula ciega `linear-tracker-adapter`,
**para** que Linear sea la fuente de verdad de los requerimientos, con trazabilidad (rama, PR, SHA, veredicto) anclada en cada uno, y retirar en una fase posterior los commits documentales que hoy solo sirven para cambiar estado.

## 2. Funcionalidades esperadas

| ID | Funcionalidad | Superficie |
|----|---------------|------------|
| F1 | Leer un issue de Linear (HU o PBI) con estado, prioridad, padre/hijos y URL. | Cápsula |
| F2 | Listar issues de un equipo o proyecto Linear con filtros (tipo HU/PBI, estado) y paginación. | Cápsula |
| F3 | Transicionar el estado de un issue. | Cápsula |
| F4 | Comentar en un issue para anclar información de entrega (SHA, PR, veredicto). | Cápsula |
| F5 | **Kalma2: listar HU y PBI del proyecto seleccionado.** Sin tracker: aviso explícito. | Puente + proceso de lectura + UI |
| F6 | **Procesos de desarrollo: ajustar estado y registrar información de interés en PBI y HU** en los hitos del ciclo (espejo activo). | Procesos + suscriptores EDA |
| F7 | Configuración de tracker por proyecto en el manifiesto. | Contrato de proyecto |
| F8 | Deuda de sincronización visible y recuperable cuando Linear falla. | Evento + suscriptor de reintento |

## 3. Decisiones

| ID | Decisión | Estado | Laudo |
|----|----------|--------|-------|
| D1 | Fuente de verdad del estado | **Dictaminada** | Linear es el SSOT objetivo. Durante esta HU la cápsula mantiene Linear como **espejo activo** y el gate local de Git no se toca: todo PBI con `tracker_ref` conserva su markdown, que se archiva en `done/` con `validacion.md` APTO en el mismo PR. Si Git y Linear discrepan, gana Git. La migración del gate nace como PBI explícito **`PBI-SPIKE-LINEAR-DONE-GATE`** (§7). |
| D2 | Representación HU/PBI en Linear | **Dictaminada** | HU = issue padre con label `hu`; PBI = sub-issue (`parent` nativo) con label `pbi`. No se usan Linear Projects para HU. Las labels son precondición configurable (`tracker.labels.*`) y filtro server-side de `list_issues`. La cápsula no las asigna ni las exige (L13–L16). |
| D3 | Kalma2 sin tracker | **Dictaminada** | El puente responde `tracker_configured: false` y la UI muestra «Proyecto sin tracker configurado». No se lee markdown. |
| D4 | Contexto RBAC | **Dictaminada** | Contexto nuevo **`tracker-operations`** en `execution-contexts.md`. Se añade a `allowed_policies` de Tekton y Argos vía `entity-manager`. La tool declara `context: tracker-operations`. |
| D4.1 | Agente solicitante del listado de Kalma2 | **Dictaminada** | Tekton como ejecutor de registro; no se despiertan Cúmulo ni Argos. Coreografía física: Kalma2 → `GET /api/backlog` → puente → `execute-process` (`tracker-backlog-query`, `context: [tracker-operations]`) → Cerbero cruza el contexto del proceso con el de la tool → el runtime ejecuta `tool:linear-tracker-adapter` sin LLM → envelope JSON síncrono → puente → UI (L17, L18, L21). |
| D5 | Fallo de Linear | **Dictaminada** | Fail-soft: la entrega de código manda. El fallo se emite como evento de dominio `Tracker_Sync_Failed` en `eda_bus.pending`. Lo consume un suscriptor de reintento; si el reintento falla, el evento pasa a `dead-letter` y `event-sweeper` lanza la alerta Kaizen. |
| D6 | Sello de inicio (`in_progress`) | **Dictaminada: (a)** | Al cerrar la fase «Inicialización de Espacio de Trabajo», `feature`, `bug-fix` y `refactorization` invocan `action:emit-work-initiated-event`, que deposita `Work_Initiated` en `eda_bus.pending`. El emisor no conoce el proveedor de tracker. `tracker-stamp` (`context: [tracker-operations]`, ejecutor de registro Tekton) se suscribe y aplica el sello. Los procesos de desarrollo no ganan `tracker-operations` (L22–L27). |
| D7 | Correlación de los eventos de PR con el issue | **Dictaminada: (a)** | Campo OPTIONAL `tracker_ref` (cadena opaca) en `PullRequest_Presented`, `PullRequest_Audited` y `PullRequest_Merged`; cada emisor lo copia de los inputs del proceso. Implementación en `PBI-ARQUITECTURA-TRACKER-STAMP` y bumps ECST vía `event-creator`. |

## 4. Alcance

### 4.A Configuración por proyecto (F7)

- `project-config-contract` 1.1.0 → **1.2.0**, campos opcionales en `{project_root}/.SddIA/project.md`:
  - `tracker.provider`: `linear`. Ausente = proyecto sin tracker (D3).
  - `tracker.team_key`: clave del equipo Linear (p. ej. `BX`). Obligatoria si hay `provider`.
  - `tracker.project_id`: opcional, acota a un Linear Project.
  - `tracker.state_map`: estados canónicos SddIA → nombre de `WorkflowState` del equipo (§4.D).
  - `tracker.labels.hu` / `tracker.labels.pbi`: nombres de label en Linear (por defecto `hu` / `pbi`) (D2).
- Core-self (sin `project_slug`) usa las mismas claves en la configuración de instancia.
- `LINEAR_API_TOKEN` se resuelve SO > proyecto (`env_ref`) > instancia > global. Nunca aparece en el manifiesto ni en el envelope.

### 4.B Cápsula `linear-tracker-adapter` (F1–F4)

- Definición `SddIA/tools/linear-tracker-adapter.md` (`name`, `uuid`, `version`, `context: tracker-operations`, `io_mode: capsule-json-io`, `implementation_path_ref`), forjada vía `entity-manager`.
- Crate Rust `SddIA/tools/linear-tracker-adapter/`, binario nativo en `SddIA/target/`. Desviación §8 declarada.
- Endpoint por defecto `https://api.linear.app/graphql`, configurable con `SDDIA_LINEAR_API_URL`. Mock de laboratorio con el patrón `SDDIA_LAB_MOCK_OUTBOUND` (`sddia-io/src/outbound_lab.rs`).
- Operaciones (`request.operation`):

| Operación | Entrada mínima | Salida (`result`) |
|-----------|----------------|-------------------|
| `fetch_issue` | `issue_ref` (identificador `BX-123` o id) | `id`, `identifier`, `title`, `state`, `priority`, `labels[]`, `parent`, `children[]`, `url`, `updated_at` |
| `list_issues` | `team_key`, filtros opcionales (`labels`, `states`, `project_id`, `parent`), `first`, `after` | `items[]` (mismo esquema reducido), `page_info` |
| `update_issue_state` | `issue_ref` + (`state_id` \| `state_name` + `team_key`) | `issue_ref`, `previous_state`, `state` |
| `create_comment` | `issue_ref`, `body` (markdown) | `comment_id`, `url` |

- `list_issues` aplica `labels` como filtro GraphQL en el servidor (D2), no en la cápsula tras descargar.
- Ceguera operativa: no decide transiciones, no redacta contenido, no asigna ni valida labels, no emite eventos (eso lo hace el proceso, §4.F).
- Errores tipados sin secretos: `LINEAR_AUTH_MISSING`, `LINEAR_AUTH_REJECTED`, `LINEAR_NOT_FOUND`, `LINEAR_STATE_AMBIGUOUS`, `LINEAR_STATE_UNKNOWN`, `LINEAR_RATE_LIMITED`, `LINEAR_TRANSPORT`, `LINEAR_GRAPHQL_ERROR`.
- `exitCode: 0 ⟺ success: true`.

### 4.C Kalma2: backlog del proyecto seleccionado (F5)

- **Proceso de lectura** `tracker-backlog-query` (forjado vía `process-creator`): `context: [tracker-operations]`, ejecutor de registro Tekton (D4.1). Inputs `project_slug` (opcional; ausente = Core-self), `kind` (`hu`|`pbi`|`all`), `state`. Una única fase `delegates_to: tool:linear-tracker-adapter` (`list_issues` con el filtro de label de `kind`), ejecutada por el runtime sin LLM. Síncrono: devuelve `items` en el envelope, sin `detached`.
- **Puente** `kalma2-bridge`: ruta `GET /api/backlog?project_slug={slug}&kind=…&state=…`, que ejecuta `tracker-backlog-query` vía `execute-process`. No invoca la cápsula directamente (L7).
  - Proyecto sin `tracker`: `200` con `tracker_configured: false`, `items: []`. No se ejecuta el proceso.
  - Con tracker: `items[]` con `kind`, `id` (`identifier`), `title`, `state` canónico, `priority`, `parent_id`, `url`. El puente deriva `kind` de `labels[]` con `tracker.labels.*`. Los issues sin label `hu` ni `pbi` no se listan.
  - Fallo de Linear al listar: error visible en la UI. Es una lectura: no se emite `Tracker_Sync_Failed`.
  - Invariante O3 de la historia madre: la respuesta no contiene `project_root` ni rutas absolutas.
- **UI** `interfaces/kalma2/`: panel «Backlog» que se recarga al cambiar el selector de proyecto, con filtros por tipo y estado. Muestra «Proyecto sin tracker configurado» si procede. Solo lectura.

### 4.D Sello de estado e información de interés en PBI y HU (F6)

Estados canónicos SddIA: `backlog`, `in_progress`, `in_review`, `done`, `cancelled`. Cada proyecto los traduce en `tracker.state_map`.

| Hito (fase o evento real) | PBI | HU madre |
|---------------------------|-----|----------|
| `forge-pbi` sella el PBI (`pbi-forged`) | `backlog`; comentario con `document_id`. | Comentario: PBI hijo añadido. |
| `Work_Initiated` (init de `feature` / `bug-fix` / `refactorization` → `emit-work-initiated-event`, D6) | `in_progress`; comentario con rama y `persist_ref`. | `in_progress` si estaba en `backlog` (HU obtenida vía `parent`). |
| `PullRequest_Presented` (DCC → `emit-pr-presented-event`; correlación según D7) | `in_review`; comentario con `pr_url` y SHA de cabeza. | — |
| `PullRequest_Audited` (`pull-request-review` → `emit-pr-audited-event`; D7) | Comentario con `resolution` (`PASS` / `REJECT` / `FLAG`). | — |
| `PullRequest_Merged` (`accept-pr` → `emit-pr-merged-event`; D7) | `done`; comentario con `merge_commit_hash` y veredicto de `validacion.md`. | `done` solo si todos sus PBI hijos están `done`. |

- La decisión de transición (incluida la de la HU) la toma el proceso o agente, nunca la cápsula.
- Vínculo PBI/HU ↔ issue: campo `tracker_ref` (`BX-123`) en el frontmatter markdown. Sin `tracker_ref` o sin tracker en el proyecto, la fase es un no-op documentado y el ciclo actual queda idéntico.
- El markdown sigue moviéndose a `done/` en el PR (D1). Esta HU no escribe estado de vuelta en el markdown.
- PBI cuyo issue no lleva la label `tracker.labels.pbi`, o cuyo `parent` no es el issue de la HU: `warn` en el envelope, sin corrección automática (D2).
- Implementación (L20): un único proceso `tracker-stamp` con `context: [tracker-operations]` es el que invoca la tool. Se dispara por suscripción en `event-domain-subscriptions.json` a `pbi-forged`, `work-initiated`, `pull-request-presented`, `pull-request-audited` y `pull-request-merged`. Así `feature`, `bug-fix` y `refactorization` no ganan `tracker-operations` en su `context` y no se editan sus fases de cierre.
- Evento `Work_Initiated` (D6), payload ECST:
  - REQUIRED: `event_id`, `correlation_id`, `source_process` (`feature` | `bug-fix` | `refactorization`), `branch`, `persist_ref`, `occurred_at`.
  - OPTIONAL: `project_slug`, `pbi_ref` (ruta relativa del PBI), `tracker_ref` (cadena opaca copiada del frontmatter del PBI).
  - FORBIDDEN: campos específicos de proveedor (`team_key`, ids de `WorkflowState`, URLs de Linear).
- Emisión fail-soft: si `emit-work-initiated-event` falla, el proceso de desarrollo sigue con `warn` (coherente con D5). Sin `tracker_ref`, el evento se emite igual; `tracker-stamp` lo descarta como no-op.
- Mutaciones de procesos, eventos y suscripciones solo vía `entity-manager` / `process-creator` / `event-creator`.

### 4.E Contexto RBAC `tracker-operations` (D4)

- Nueva entrada §2.10 en `execution-contexts.md` (bump 1.1.0 → 1.2.0) vía `entity-manager`. Dominio: requerimientos en trackers externos. Alcance: leer, listar, transicionar y comentar issues. Fuera de alcance: crear o borrar issues, administrar equipos o webhooks.
- `allowed_policies` de Tekton y Argos ganan `tracker-operations` vía `entity-manager`. No se toca `system-operations`.
- Doble cerrojo (L17): en fases `tool:` el permiso efectivo es el `context[]` del proceso (`tracker-backlog-query`, `tracker-stamp`, `tracker-sync-replay`). En fases `agent:` que emitan invocaciones a la tool rige `allowed_policies` del agente. Ambos deben incluir `tracker-operations`.
- Argos lo necesita para el spike (consultar Linear antes de aprobar). En esta HU no invoca la tool.

### 4.F Deuda de sincronización (F8, D5)

- Evento de dominio nuevo `Tracker_Sync_Failed` (`SddIA/events/domain/tracker-sync-failed.md`) vía `event-creator`. Payload ECST:
  - REQUIRED: `event_id`, `correlation_id`, `source_process`, `issue_ref`, `operation`, `target_state` o `comment_kind`, `error_code`, `occurred_at`.
  - OPTIONAL: `project_slug`, `pr_url`, `commit_sha`, `attempt`.
  - Prohibido incluir la clave de API o el cuerpo de respuesta de Linear.
- Emisión: `tracker-stamp` captura el error de la cápsula, emite el evento en `eda_bus.pending`, deja `warn` en su envelope y devuelve éxito al proceso padre.
- La discrepancia de L3 (estado en Git distinto del de Linear al hacer merge) se emite con el mismo evento y `error_code: TRACKER_STATE_DIVERGED`.
- Suscriptor: proceso `tracker-sync-replay` (`context: [tracker-operations]`), registrado en `event-domain-subscriptions.json`.
  - Lee el estado actual del issue (`fetch_issue`) y solo aplica la transición si sigue avanzando el ciclo (orden de §4.D). Si está obsoleta, la descarta y lo registra.
  - Los comentarios llevan la marca `sddia-sync-id: {event_id}`; si ya existe un comentario con esa marca, no se duplica.
  - Si falla, el evento queda en `eda_bus.dead_letter` y `event-sweeper` emite la alerta Kaizen ya existente.
- No se emite `System_Fracture_Detected` (L12).

## 5. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | `SddIA/tools/linear-tracker-adapter.md` existe con `uuid`, `context: tracker-operations`, `io_mode` e `implementation_path_ref`, y figura en `tools/index.md`. | `sddia-qa verify-tools-index` |
| AC-2 | Las cuatro operaciones responden según `capsule-json-io` 2.0 contra el mock de laboratorio, incluidos los errores tipados. | Tests del crate con mock outbound |
| AC-3 | Ningún secreto en `message`, `feedback`, `result`, `error` ni en `Tracker_Sync_Failed`. Sin `LINEAR_API_TOKEN` → `LINEAR_AUTH_MISSING`, `exitCode` ≠ 0. | Test + `rg` sobre envelopes y eventos de la suite |
| AC-4 | `update_issue_state` con nombre ambiguo o inexistente falla con `LINEAR_STATE_AMBIGUOUS` / `LINEAR_STATE_UNKNOWN` sin mutar el issue. | Test con mock |
| AC-5 | Con un proyecto `provider: linear` en mock, `GET /api/backlog?project_slug={slug}` lista HU y PBI del `team_key` del manifiesto y no los de otro proyecto. | Test del puente |
| AC-6 | Para un proyecto sin `tracker` (p. ej. `barcelonaxplorer` hoy) y para Core-self sin configuración, `/api/backlog` responde `tracker_configured: false` y la UI muestra el aviso. | Test del puente + prueba manual en Kalma2 |
| AC-7 | Cero `project_root` o rutas absolutas en `/api/backlog` y en `interfaces/kalma2/`. | `rg` |
| AC-8 | Un ciclo `feature` con PBI con `tracker_ref` (mock) produce `in_progress` → `in_review` → `done` y comentarios con rama, `pr_url` y merge SHA. El markdown termina en `done/` con `validacion.md` APTO. | Suite E2E de laboratorio |
| AC-9 | La HU madre pasa a `done` solo cuando su último PBI hijo pasa a `done`. | Suite E2E |
| AC-10 | Sin `tracker_ref` o sin tracker, el ciclo actual queda idéntico y verde. | Suite Core-self existente |
| AC-11 | Con Linear caído en mock, la entrega se completa, la fase deja `warn` y aparece un `Tracker_Sync_Failed` en `eda_bus.pending`. | Test con mock caído |
| AC-12 | Con Linear restablecido, `tracker-sync-replay` aplica la transición pendiente; si el issue ya avanzó, la descarta. No hay comentarios duplicados. | Test con mock |
| AC-13 | Con el reintento también fallido, el evento acaba en `dead-letter` y `event-sweeper` emite la alerta Kaizen. | Suite EDA de laboratorio |
| AC-14 | Un proceso con `context: [tracker-operations]` supera Cerbero al invocar la tool. Un proceso sin ese contexto (p. ej. con `system-operations`) recibe `exitCode: 1`. Lo mismo para agentes con y sin la política. | Test RBAC |
| AC-15 | `policy_validator` reconoce `tracker-operations` desde la norma (test de contextos con `>= 10`). | `cargo test` en `execute-process` |
| AC-16 | `list_issues` con `kind: hu` envía el filtro de label en la query GraphQL; el mock verifica que la cápsula no descarga issues de otras labels. | Test con mock |
| AC-17 | `tracker-backlog-query` responde de forma síncrona (sin `detached`) y sin invocar ningún LLM. | Test del proceso + telemetría sin `llm:*` |
| AC-18 | Un issue con `tracker_ref` sin la label `pbi`, o con `parent` distinto de la HU, produce `warn` en `tracker-stamp` y Linear no se modifica. | Test con mock |
| AC-19 | Entradas en `SddIA/evolution/` vinculadas al `uuid` de la tool, del contexto, de los eventos, de la acción de emisión, del contrato de proyecto y de los procesos mutados. | `sddia-qa gate-evolution --range` |
| AC-20 | Al iniciar `feature`, `bug-fix` o `refactorization` aparece un `Work_Initiated` en `eda_bus.pending` sin campos de proveedor, y el `context` de esos procesos no contiene `tracker-operations`. | Suite E2E + `rg` sobre los procesos |
| AC-21 | Con `Work_Initiated` sin `tracker_ref`, o en un proyecto sin tracker, `tracker-stamp` termina en no-op y no invoca la cápsula. | Test del proceso |
| AC-22 | Cada evento de PR del ciclo se correlaciona con el issue correcto según D7, incluidos dos ciclos simultáneos sobre PBI distintos. | Suite E2E con mock |

## 6. Fuera de alcance

- Migrar el gate de Done a Linear (objeto de `PBI-SPIKE-LINEAR-DONE-GATE`).
- Leer markdown legacy desde Kalma2 (D3).
- Crear o editar issues desde Kalma2, o forjar PBIs directamente en Linear.
- Sincronización inversa (cambios hechos en Linear que vuelvan al markdown).
- Webhooks de Linear hacia SddIA.
- Otros trackers (Jira, GitHub Issues). `tracker.provider` los admite a futuro sin cambiar Kalma2.

## 7. Desglose en PBIs (forjados 2026-10-02)

`forge-pbi` exige `project_slug` registrado y escribe en el `todos_pending` del cliente (`forge_pbi.rs`). El único índice es `barcelonaxplorer`. Estos PBIs son de Core: viven en `docs/todos/pending/` de SddIA, mismo patrón que la serie Workspace 1×N.

| Orden | PBI | Cubre | Depende de |
|-------|-----|-------|------------|
| 1 | `PBI-ARQUITECTURA-TRACKER-OPERATIONS-CONTEXT` | D4 | — |
| 2 | `PBI-ARQUITECTURA-PROJECT-TRACKER-CONTRACT` | F7 | — |
| 3 | `PBI-ARQUITECTURA-LINEAR-TRACKER-ADAPTER` | F1–F4 | 1 |
| 4 | `PBI-ARQUITECTURA-KALMA2-BACKLOG` | F5 | 2, 3 |
| 5 | `PBI-ARQUITECTURA-TRACKER-SYNC-FAILED` | F8 | 3 |
| 6 | `PBI-ARQUITECTURA-WORK-INITIATED` | F6 (D6) | — |
| 6b | `PBI-ARQUITECTURA-TRACKER-STAMP` | F6 | 2, 3, 5, 6 |
| 7 | `PBI-SPIKE-LINEAR-DONE-GATE` | D1 | 6b |

### 7.1 Alcance del spike `PBI-SPIKE-LINEAR-DONE-GATE`

Investigación con entregable documental (informe + propuesta de HU), sin cambios de gate en producción.

- Cómo `pull-request-review` (Argos) consulta el estado del issue (`fetch_issue`) antes de emitir veredicto.
- Qué sustituye al handler `feature-pbi-archive` y a la exigencia `pbi_archived: true` cuando el PBI vive solo en Linear.
- Qué pasa con `validacion.md`: sigue en Git como evidencia de QA o se ancla como comentario o adjunto en Linear.
- Impacto en `task-closure-documental.mdc`, `features-documentation-pattern`, `feature`, `bug-fix`, `refactorization` y la aduana pre-push.
- Deuda detectada: `phase_capsules.rs` cablea `docs/todos/done` e ignora `docs_layout.todos_done` del proyecto.
- Plan de convivencia para proyectos sin tracker (D3 los deja fuera del panel, pero el gate debe seguir funcionando para ellos).

## 8. Prerrequisitos operativos

- `LINEAR_API_TOKEN` presente en la bóveda de instancia (2026-10-02). Precedencia SO > proyecto > instancia > global.
- Un equipo Linear de prueba con los estados declarados en `tracker.state_map`.
- Labels `hu` / `pbi` (o las configuradas) creadas en el equipo Linear; HU y PBI existentes etiquetados y con `parent` correcto (D2).

## 9. Cierre de la HU (especificación, 2026-10-02)

| Campo | Valor |
|-------|--------|
| Fase cerrada | Desglose, laudos D1–D7 y forja documental de 8 PBIs |
| Ubicación archivo | `Documentacion/PBI/Realizado/` (narrativa archivada) |
| Cola de ejecución | `docs/todos/pending/` — SSOT despachable (`paths.todos.pending`, `todos-jurisdiction`) |
| Modo PBIs | `execution_mode: autonomo`, `dispatch: true`, `especificacion_cerrada: 2026-10-02` |
| Implementación | Cada PBI arranca con `./sddia-run.sh --process feature` y su `pbi_ref`; **no** requiere reabrir esta HU |
| Done de implementación | Un PR por PBI (o cadena acordada) con `validacion.md` APTO y archivo en `docs/todos/done/` |
| Linear | `LINEAR_API_TOKEN` disponible en bóveda instancia |

`historias/` queda vacía de esta HU: la narrativa vive en `Documentacion/PBI/Realizado/`; la cola operativa no se mueve.
