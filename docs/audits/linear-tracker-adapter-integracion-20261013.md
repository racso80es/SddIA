# Informe — Cápsula Linear e integración en SddIA

Fecha de contraste: 2026-10-13.  
Alcance: `linear-tracker-adapter` v1.1.0 y el circuito que la rodea (manifiesto, procesos, bus EDA, daemon, gate de Done).

## 1. Qué es

`linear-tracker-adapter` es un adaptador GraphQL ciego. Recibe JSON por stdin y responde JSON por stdout. No decide transiciones ni interpreta el ciclo de SddIA: ejecuta operaciones contra la API de Linear.

El significado de cada estado, label y evento vive fuera de la cápsula:

- en el manifiesto de proyecto (`tracker`);
- en procesos nativos del motor;
- en las suscripciones de `SddIA/core/event-domain-subscriptions.json`.

Definición: `SddIA/tools/linear-tracker-adapter.md`.  
Implementación: `SddIA/tools/linear-tracker-adapter/` (binario nativo, cliente `ureq`).  
Contexto RBAC: `tracker-operations`.  
UUID: `8f3c2a1b-9d4e-4f5a-b6c7-1234567890ab`.

## 2. Arquitectura

| Capa | Artefacto | Rol |
|------|-----------|-----|
| I/O | `linear-tracker-adapter` | Token, GraphQL, códigos `LINEAR_*`. |
| Orquestación | `tracker-stamp`, `forge-pbi`, `refine-hu`, `tracker-sync-replay`, `tracker-linear-markdown-sync`, `tracker-markdown-apply`, `tracker-backlog-query` | Deciden cuándo crear, transicionar, comentar o volcar texto. |
| Sensor | Daemon `linear-bridge-watcher` | Polling e inyección de `Tracker_Issue_Changed`. El webhook es solo de laboratorio. No escribe markdown. |

Norma: `SddIA/norms/execution-contexts.md` §2.10 (RBAC 1.4.0).

**En alcance:** leer, listar, transicionar, comentar, crear issues y recibir webhooks firmados en laboratorio (bind local, HMAC `Linear-Signature`).

**Fuera de alcance:** borrar issues y administrar equipos o proyectos.

Los procesos de desarrollo (`feature`, `bug-fix`, `refactorization`) no llevan el contexto `tracker-operations`. Quien habla con Linear es la forja (`forge-pbi`, `refine-hu`) o un suscriptor EDA.

## 3. Configuración

Contrato: `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md` v1.3.0. El Core valida la forma. No interpreta semántica de negocio.

### 3.1 Índice Core

Ruta: `.SddIA/projects/{slug}.md`.

Campos obligatorios: `id`, `uuid`, `project_root` (absoluto), `manifest_ref`, `codex_slug`, `status` (`active` | `inactive`).

### 3.2 Manifiesto de proyecto

Ruta: `{project_root}/.SddIA/project.md`. Bloque `tracker`:

| Campo | Regla |
|-------|--------|
| `provider` | Solo `linear`. |
| `team_key` | Obligatorio si hay `provider`. Prefijo del identificador Linear (`BX-12`). |
| `project_id` | Opcional. UUID del Linear Project. Acota listados y altas. |
| `state_map` | Clave canónica → nombre de WorkflowState. Claves: `backlog`, `todo`, `in_progress`, `in_review`, `done`, `cancelled`. `todo` es opcional. |
| `labels.hu` | Default lógico `hu`. |
| `labels.pbi` | Default lógico `pbi`. |
| `labels.fix` | Default lógico `fix`. |
| `labels.kaizen` | Default lógico `kaizen`. |
| `labels.deuda` | Default lógico `deuda`. |
| `labels.spike` | Default lógico `spike`. |
| `labels.editable` | Default lógico `sddia-editable`. Opt-in para editar el cuerpo desde Linear. |
| `done_gate` | `git` \| `linear` \| `both`. Default `git`. Sin bloque `tracker`, el motor fuerza `git`. |

Los secretos (`LINEAR_*`) están prohibidos en el manifiesto. Van en la bóveda de `env_ref` (típicamente `.SddIA/.dev/.env`).

### 3.3 Variables de entorno

| Variable | Uso |
|----------|-----|
| `LINEAR_API_TOKEN` | Autenticación. |
| `SDDIA_LINEAR_API_URL` | Endpoint GraphQL. Default `https://api.linear.app/graphql`. |
| `SDDIA_LAB_MOCK_OUTBOUND` | Mock de laboratorio. |
| `SDDIA_LAB_MOCK_LINEAR_URL` | URL mock de laboratorio. |
| `LINEAR_WEBHOOK_SECRET` | HMAC del webhook de laboratorio. |
| `SDDIA_LINEAR_WEBHOOK_PORT` | Puerto del POST local. `0` = desactivado. |
| `SDDIA_LINEAR_WEBHOOK_BIND` | Bind. Default `127.0.0.1`. |
| `SDDIA_LINEAR_BRIDGE_POLL_SECONDS` | Intervalo de polling del watcher. |
| `SDDIA_TRACKER_MARKDOWN_APPLY_SKIP_GIT` | Salta la persistencia git en `tracker-markdown-apply` (`1` o `true`). |

No existe la variable `LINEAR_PROJECT`. El laudo la sustituyó por `tracker.project_id`.

### 3.4 Instancia registrada

| Pieza | Ubicación |
|-------|-----------|
| Índice | `.SddIA/projects/barcelonaxplorer.md` |
| `project_root` | `/home/racso/Proyectos/BarcelonaXplorer` |
| Manifiesto | `/home/racso/Proyectos/BarcelonaXplorer/.SddIA/project.md` |

Valores vigentes del piloto: `provider: linear`, `team_key: BX`, mapa de estados completo, labels estándar, `done_gate: git`, `env_ref: .SddIA/.dev/.env`. `project_id` no está declarado: la cápsula opera sobre el equipo `BX` entero.

El índice `.SddIA/projects` no contiene un manifiesto del Core. Entradas actuales: `barcelonaxplorer` y fixtures de laboratorio (`lab-done-gate`, `lab-done-gate-both`).

## 4. Operaciones de la cápsula

`dispatch` en `SddIA/tools/linear-tracker-adapter/src/main.rs`:

| Operación | Entrada principal | Salida |
|-----------|-------------------|--------|
| `fetch_issue` | `issue_ref` | Issue. |
| `list_issues` | `team_key`; opcionales `labels[]`, `project_id` | Lista. |
| `update_issue_state` | `issue_ref`, `state_name`, `team_key` | Transición al WorkflowState resuelto por nombre. |
| `create_issue` | `team_key`, `title`, `description`, `labels[]`; opcionales `parent_ref`, `project_id`, `state_name`, `priority` | `issue_ref`, `id`, `url`. |
| `create_comment` | Issue + cuerpo | Comentario. |
| `update_issue_description` | Issue + descripción | Descripción sustituida. No crea issues. |

Códigos de error: `LINEAR_AUTH_MISSING`, `LINEAR_AUTH_REJECTED`, `LINEAR_TRANSPORT`, `LINEAR_RATE_LIMITED`, `LINEAR_GRAPHQL_ERROR`, `LINEAR_NOT_FOUND`, `LINEAR_STATE_UNKNOWN`, `LINEAR_STATE_AMBIGUOUS`, `LINEAR_LABEL_UNKNOWN`, `LINEAR_PARENT_NOT_FOUND`.

El identificador lo asigna Linear (`TEAMKEY-n`). Las labels las envía el proceso que llama a `create_issue`.

## 5. Ciclo directo (SddIA → Linear)

### 5.1 Alta

1. **`refine-hu` v1.0.1** (`SddIA/process/refine-hu.md`). Refina el markdown de la HU. Si el manifiesto tiene `tracker` y el frontmatter no tiene `tracker_ref`, llama a `create_issue` con label `hu`. Sella y emite `HU_Refined`.
2. **`forge-pbi` v1.1.3**. Fase «Registro en tracker»: `create_issue` con label `pbi` más el tipo (`fix`, `kaizen`, `deuda`, `spike`) y `parent_ref` tomado de la HU. El sellado escribe `tracker_ref` y emite `PBI_Forged`.
3. **`refine-pbi` v1.0.1**. No llama a Linear. Refina el cuerpo, sella y emite `PBI_Refined`. El sello lo aplica `tracker-stamp`.

### 5.2 Sello (`tracker-stamp` v1.1.0)

Suscriptor táctico. Ciego al markdown. Fail-soft: un fallo de Linear emite `Tracker_Sync_Failed` y no aborta el ciclo Git.

| Evento | Efecto |
|--------|--------|
| `PBI_Forged` | Estado `backlog` + comentario con `document_id`. |
| `HU_Refined` | Transición a `todo` + comentario. |
| `PBI_Refined` | Transición a `todo` + comentario (`document_id` y versión). |
| `Work_Initiated` | `in_progress` + comentario de rama y `persist_ref`. Si la HU padre está en `backlog` o `todo`, también pasa a `in_progress`. |
| `PullRequest_Presented` | `in_review` + comentario del PR. |
| `PullRequest_Audited` | Comentario de la auditoría. Sin cambio de estado. |
| `PullRequest_Merged` | `done` + comentario. Roll-up de la HU si todos los hijos están en `done` o `cancelled` y al menos uno está `done`. Escribe proof local. |
| `Delivery_Committed` | Mismo cierre que el merge, para `trunk_direct` (sin PR). |
| `PBI_Cancelled` | `cancelled`, comentario con motivo, archivo del markdown en `todos_done`. |

Si `state_map` no declara `todo`, la transición a `todo` es un warn y un no-op. No hay fallback a otro estado.

`Work_Initiated` lo emite el motor al entrar en la primera fase `agent:tekton`.

Suscripciones: `SddIA/core/event-domain-subscriptions.json`, agente `tekton`, proceso `tracker-stamp`.

### 5.3 Reintento

`tracker-sync-replay` v1.1.0 suscribe `Tracker_Sync_Failed`. Hace `fetch_issue` y reaplica la operación solo si el ciclo avanza:

`backlog` < `todo` < `in_progress` < `in_review` < `done`.

`cancelled` es terminal. Un segundo fallo queda en el bus (dead-letter). El evento `Tracker_Sync_Failed` puede llevar `error_code: TRACKER_STATE_DIVERGED` o un código `LINEAR_*`.

### 5.4 Markdown local → descripción Linear

`tracker-linear-markdown-sync` v1.0.0 recorre los `.md` con `tracker_ref` (`sync_all`, default `true`) o acepta el par `markdown_path` + `issue_ref`. Llama a `update_issue_description` con el cuerpo íntegro, tras redactar líneas de secretos conocidos. No crea issues.

## 6. Ciclo inverso (Linear → SddIA)

1. **`linear-bridge-watcher` v1.0.0.** Polling de `list_issues` por `team_key` y `project_id`. Cursor en `.SddIA/state/linear-bridge-watcher/{project_slug}.json`. Heartbeat 60 s. Entrypoint: `SddIA/daemons/linear-bridge-watcher.sh`.
2. **Webhook de laboratorio.** POST en bind local. Firma HMAC `Linear-Signature`. Sin endpoint público.
3. **Filtro.** Solo issues con label `hu` o `pbi`.
4. **Anti-eco.** Descarta cambios que coinciden con el registro saliente `.SddIA/state/tracker-outbound/{issue_ref}.jsonl` (escrito por `tracker-stamp`, `tracker-sync-replay`, `forge-pbi` y `refine-hu`) o con comentarios marcados `sddia-sync-id`.
5. **Evento `Tracker_Issue_Changed`.** Emisor único: el watcher. `source`: `poll` | `webhook`. Payload: `correlation_id`, `occurred_at`, `project_slug`, `issue_ref`, `changes`, `source`. Prohibidos el token, el cuerpo crudo del webhook y `hash_signature`.
6. **`tracker-markdown-apply` v1.0.0.** Réplica al markdown local y commit en la rama `tracker-sync/{project_slug}` vía skill `git-manager`. Prohibido commitear en `default_branch`.
7. **Label `editable`.** El cuerpo del markdown solo se sobrescribe si el issue lleva `sddia-editable`. Sin esa label, el cuerpo es no-op. La cascada F2 (`objectives.md`, `spec.md`, `plan.md`, `implementation.md`) no es destino de esta réplica.

Definición del evento: `SddIA/events/domain/tracker-issue-changed.md`.

## 7. Gate de Done

Implementación: `SddIA/engine/execute-process/src/engine/done_gate.rs`.

| `done_gate` | Cierre |
|-------------|--------|
| `git` (default) | PR fusionado en `default_branch` + `validacion.md` APTO + PBI en `done/`. SddIA no rastrea el despliegue. |
| `linear` | El cierre lee el estado Linear `done`. El pre-push exige el proof local `.SddIA/proofs/tracker-done/{tracker_ref}.json` escrito por `tracker-stamp` al sellar `done`. Sin red en el hook. |
| `both` | Exige Git y Linear. Si Linear no responde durante el archivado, fallback a Git y emisión de `Tracker_Sync_Failed` con `LINEAR_UNAVAILABLE`. |

El validador acepta los tres valores. El default del motor y el manifiesto de BarcelonaXplorer permanecen en `git`.

Proof: `tracker_ref`, `state`, `occurred_at`, `commit_sha`, snapshot del issue. Directorio `.SddIA/proofs/tracker-done/`.

## 8. Lectura desde Kalma2

`tracker-backlog-query` v1.0.0 lista issues para Kalma2.

Inputs: `project_slug` (opcional; ausente = Core-self), `kind` (`hu` | `pbi` | `all`, default `all`), `state` canónico opcional.

Outputs: `tracker_configured`, `items`. Sin bloque `tracker` en el manifiesto, `tracker_configured` es `false`.

El bridge lo expone en `handle_backlog` (`SddIA/interfaces/kalma2-bridge/src/main.rs`), query `project_slug`, `kind`, `state`.

## 9. Estado de la forja

| Bloque | Documento | Estado |
|--------|-----------|--------|
| Cápsula y orquestación base | `Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md` | Hecho. |
| HU padre | `HU-LINEAR-SYNC-FLOW` en `docs/todos/historias/` | Particionada (v2.3.0). No se forjan PBI desde este `document_id`. |
| HU-A ciclo directo | `HU-LINEAR-DIRECT-CYCLE` | Cerrada. PR #335 (2026-10-04). PBI A-01…A-09 en `docs/todos/done/`. E2E `linear_direct_cycle_e2e`. |
| HU-B inverso y gate | `HU-LINEAR-SSOT-INVERSE` | Cerrada. PR #336–#341 (2026-10-12). PBI B-01…B-06 en `docs/todos/done/`. E2E `linear_done_gate_both_e2e` sobre fixture `done_gate: both`. |

No hay PBI Linear en `docs/todos/pending/`.

Decisiones de laudo que ya están en el repo: D-A (`create_issue`), D-B (`Work_Initiated` al entrar en Ejecución), D-C (promoción de HU desde `backlog` o `todo`), D-D (`refine-hu` / `refine-pbi`), D-E (warn sin `todo`), D-F (polling + webhook lab), D-H (cuerpo solo con `editable`), D-J (`refine-hu` crea el issue de la HU).

## 10. Pendiente y límites

1. **`done_gate: linear` no está activado en producto.** El laudo D-I lo reserva al manifiesto Core-self. Ese manifiesto no existe en `.SddIA/projects`. Escribir `linear` en BarcelonaXplorer queda fuera de ese laudo. El fixture `lab-done-gate` usa `linear` como prueba de AC-18, no como activación de producto. `lab-done-gate-project-both` permanece en `both`.
2. **`project_id` ausente** en BarcelonaXplorer. No hay acotación a un Linear Project concreto.
3. **Webhook solo de laboratorio.** El sensor de producción es el polling. No hay endpoint público.
4. **Réplica inversa acotada a rutas fijas.** `tracker-markdown-apply` no lee `docs_layout`. Recorre `docs/todos/pending`, `docs/todos/done`, `docs/todos/historias` y `Documentacion/PBI/Realizado`. En BarcelonaXplorer, `Documentacion/PBI/Pendiente` y `Documentacion/HistoriasDeUsuario/` quedan fuera. El manifiesto del piloto lo declara no operativo para el alta inversa hasta un laudo posterior.
5. **D-G (PR diario/lote).** Lo materializado es un commit en `tracker-sync/{project_slug}`. No hay empaquetador que abra un único PR al día.
6. **Permanente.** Borrar issues, administrar equipos o proyectos, y rastrear un estado «en producción». Done es consolidación en `default_branch`.

## 11. Mapa de artefactos

| Artefacto | Ruta |
|-----------|------|
| Cápsula | `SddIA/tools/linear-tracker-adapter.md` |
| Crate | `SddIA/tools/linear-tracker-adapter/` |
| Contrato de proyecto | `SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md` |
| RBAC | `SddIA/norms/execution-contexts.md` §2.10 |
| Sello | `SddIA/process/tracker-stamp.md` |
| Replay | `SddIA/process/tracker-sync-replay.md` |
| Volcado descripción | `SddIA/process/tracker-linear-markdown-sync.md` |
| Réplica inversa | `SddIA/process/tracker-markdown-apply.md` |
| Consulta Kalma2 | `SddIA/process/tracker-backlog-query.md` |
| Alta PBI | `SddIA/library/codexes/codex-software-engineering/process/forge-pbi.md` |
| Alta HU | `SddIA/process/refine-hu.md` |
| Refino PBI | `SddIA/process/refine-pbi.md` |
| Watcher | `SddIA/daemons/linear-bridge-watcher.md` |
| Runtime del sensor | `SddIA/sddia-daemon-runtime/src/linear_bridge.rs`, `linear_bridge_webhook.rs` |
| Gate | `SddIA/engine/execute-process/src/engine/done_gate.rs` |
| Binding | `SddIA/engine/execute-process/src/engine/project_binding.rs` |
| Anti-eco | `SddIA/engine/execute-process/src/engine/tracker_outbound.rs` |
| Evento fallo | `SddIA/events/domain/tracker-sync-failed.md` |
| Evento cambio | `SddIA/events/domain/tracker-issue-changed.md` |
| Suscripciones | `SddIA/core/event-domain-subscriptions.json` |
| Índice piloto | `.SddIA/projects/barcelonaxplorer.md` |
| Manifiesto piloto | `/home/racso/Proyectos/BarcelonaXplorer/.SddIA/project.md` |
