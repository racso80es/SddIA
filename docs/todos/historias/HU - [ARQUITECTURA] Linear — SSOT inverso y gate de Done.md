---
document_id: HU-LINEAR-SSOT-INVERSE
parent_hu: HU-LINEAR-SYNC-FLOW
title: "HU - [ARQUITECTURA] Linear — SSOT inverso y gate de Done"
format: markdown
version: "1.1.0"
created: "2026-10-03"
status: "bloqueada"
hu_sequence: 2
hu_sequence_total: 2
predecessor_hu: HU-LINEAR-DIRECT-CYCLE
blocked_by: HU-LINEAR-DIRECT-CYCLE
unblock_condition: "HU-LINEAR-DIRECT-CYCLE mergeada en default_branch y AC-7/AC-8 (E2E lab) en verde"
priority: "alta"
process: "feature"
base: "Documentacion/PBI/Realizado/[ARQUITECTURA] Forja de la Cápsula Linear Tracker y Orquestación del Ciclo de Vida.md"
supersedes_laudo: "docs/features/tracker-done-gate/spec.md (laudo «no migrar», reabierto por el Vértice 2026-10-03)"
related:
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/process/tracker-stamp.md
  - SddIA/process/tracker-sync-replay.md
  - SddIA/process/tracker-linear-markdown-sync.md
  - SddIA/norms/execution-contexts.md
  - SddIA/skills/git-manager.md
  - SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md
  - SddIA/library/codexes/codex-software-engineering/process/accept-pr.md
  - SddIA/events/domain/tracker-sync-failed.md
  - SddIA/daemons/daemons-contract.md
  - SddIA/daemons/github-bridge-watcher.md
  - docs/features/spike-linear-done-gate/informe.md
  - docs/features/tracker-done-gate/spec.md
  - SddIA/core/cumulo.paths.json
---

# [ARQUITECTURA] Linear — SSOT inverso y gate de Done

Partición B de `HU-LINEAR-SYNC-FLOW` (§8 del documento padre, laudo 2026-10-03). Historial de refinamiento en el padre.

## Orden

**HU 2 de 2.** Predecesora: `HU-LINEAR-DIRECT-CYCLE` (HU 1 de 2). Esta HU no se forja hasta `PBI-LINEAR-A-09-E2E` en verde y mergeado.

| Orden | PBI | Entrega |
|------:|-----|---------|
| 01 | `PBI-LINEAR-B-01-OUTBOUND` | Registro saliente anti-eco. |
| 02 | `PBI-LINEAR-B-02-WATCHER` | Daemon polling + evento `Tracker_Issue_Changed`. |
| 03 | `PBI-LINEAR-B-03-WEBHOOK` | RBAC §2.10 → 1.4.0 y webhook lab. |
| 04 | `PBI-LINEAR-B-04-MARKDOWN-APPLY` | Proceso `tracker-markdown-apply`. |
| 05 | `PBI-LINEAR-B-05-DONE-GATE` | `done_gate: linear\|both`, handler, pre-push, normas. |
| 06 | `PBI-LINEAR-B-06-E2E` | E2E Core-self `done_gate: both`. Activación según F8.3. |

PBIs en `docs/todos/pending/`, todos con `blocked_by` que incluye `PBI-LINEAR-A-09-E2E` de forma transitiva.

## 1. Historia de usuario

**Como** Vértice Biológico y como agentes orquestadores,
**quiero** que los cambios hechos en Linear (estado, título, descripción opt-in, prioridad, padre, cancelación) vuelvan al markdown local como eventos de dominio, y que el criterio de Done del motor pueda leerse desde Linear,
**para** que Linear sea la fuente de verdad de los requerimientos (D1 de la HU base, fase «SSOT objetivo») y el markdown quede como réplica trazable, sin commits documentales cuyo único fin sea cambiar estado.

## 2. Herencia de HU-A (ya disponible al desbloquearse)

- `project-config-contract` **1.3.0**: `done_gate` declarado (solo `git` aceptado), `tracker.labels.editable` declarada inerte. **Esta HU no vuelve a subir el contrato**: habilita `linear|both` en el validador y consume `editable`.
- `execution-contexts.md` **1.3.0** §2.10 (crear issues). Esta HU lo sube a **1.4.0** añadiendo «recibir webhooks firmados».
- Evento `PBI_Cancelled`; esta HU añade `tracker-markdown-apply` como emisor autorizado.
- `tracker-stamp` sella `done` en `PullRequest_Merged` / `Delivery_Committed`.

## 3. Alcance

### F7 — Sincronización inversa Linear → markdown (D-F, D-G, D-H)

Hoy no existe escritura hacia el markdown: `tracker-linear-markdown-sync` vuelca markdown → descripción Linear.

**F7.1 Sensor `linear-bridge-watcher`** (daemon, `daemons-contract` v1.0.0, vía `daemon-creator`; patrón `github-bridge-watcher`; `context: tracker-operations`):
- Jurisdicción aislada, ceguera lógica: solo materializa eventos físicos en el bus. No decide negocio ni toca markdown.
- Polling por intervalo (default 60 s) de `list_issues` por `team_key` + `project_id` con cursor `updated_at` persistido en `.SddIA/state/linear-bridge-watcher/{project_slug}.json`. `Daemon_Heartbeat` cada 60 s.
- Webhook **solo laboratorio** (túnel): endpoint HTTP local; valida `Linear-Signature` (HMAC-SHA256 con `LINEAR_WEBHOOK_SECRET` desde `env_ref`, nunca en el manifiesto). Sin endpoint público (D-F). Requiere RBAC §2.10 1.4.0.
- Filtro: solo issues con label `tracker.labels.hu` o `tracker.labels.pbi`.
- Anti-eco: descarta cambios que coincidan con operaciones salientes registradas por `tracker-stamp` / `tracker-sync-replay` / `forge-pbi` / `refine-hu` (`.SddIA/state/tracker-outbound/{issue_ref}.jsonl`, ventana configurable) o con comentarios marcados `sddia-sync-id`.

**F7.2 Evento `Tracker_Issue_Changed`** (vía `event-creator`, familia `domain`, contexto `tracker-operations`):
- REQUIRED: `event_id`, `correlation_id`, `occurred_at`, `project_slug`, `issue_ref`, `changes[]` (`{field, from, to}`, `field ∈ {state, title, description, priority, labels, parent, cancelled}`), `source` (`poll` | `webhook`).
- OPTIONAL: `actor`, `linear_updated_at`.
- FORBIDDEN: token, secreto de webhook, cuerpo bruto del webhook.

**F7.3 Suscriptor `tracker-markdown-apply`** (proceso, `context: [tracker-operations, filesystem-ops, source-control]`, handler nativo, sin LLM):
- Resuelve el markdown por `tracker_ref` (índice inverso sobre `docs_layout.todos_pending|todos_done` + `historias/`). Sin markdown → `warn` no-op (no se crean PBI desde Linear).

| Campo Linear | Efecto en markdown | Notas |
|---|---|---|
| `state → cancelled` | `status: cancelado`; mover a `todos_done` | Emite `PBI_Cancelled`. |
| `state → todo/backlog/in_progress/in_review` | `tracker_state: {canónico}` | Solo espejo; no mueve ficheros ni cambia `status`. |
| `state → done` | `tracker_state: done` | Con `done_gate: git`, no archiva. Con `linear`/`both`, archiva en `todos_done` (F8.2). |
| `title` | frontmatter `title` | El fichero no se renombra. |
| `description` | cuerpo del PBI/HU **solo** con label `tracker.labels.editable` | Sin la label: `warn` no-op. La cascada F2 (`objectives.md`, `spec.md`, …) nunca es destino. |
| `priority` | frontmatter `priority` | Linear 0–4 → `sin-prioridad|urgente|alta|media|baja`. |
| `parent` | frontmatter `historia_ref` | Solo si existe HU local con ese `tracker_ref`. |

- Persistencia: commit en rama `tracker-sync/{project_slug}` mediante la skill `git-manager`; un PR agrupado por día o por lote de N cambios (D-G). Prohibido push a `default_branch` (pre-push y `task-closure-documental`).
- Idempotencia: `tracker_sync_last_event` en frontmatter; eventos anteriores se descartan.
- Conflicto: markdown modificado localmente tras `linear_updated_at` → gana repo, `Tracker_Sync_Failed` con `error_code: TRACKER_STATE_DIVERGED`, fichero intacto.

### F8 — Migración del gate de Done a Linear (D-I)

Reabre el laudo `tracker-done-gate`. El spike localizó la deuda en `feature-pbi-archive` (`phase_capsules.rs`, tres ramas) y en el pre-push. La HU no elimina el gate: cambia su **fuente**.

**F8.1 Criterio de Done (motor):**

```text
Done = PR fusionado en default_branch (o Delivery_Committed en trunk_direct)
     + issue tracker_ref en estado canónico `done` (fetch_issue en vivo)
     + validacion.md con global: APTO en el PR
```

- Validador del contrato acepta `done_gate ∈ {git, linear, both}` (declarado en 1.3.0 por HU-A). Default `git`. Proyecto sin `tracker` → `git` forzado.
- Con `linear`: `pbi_archived: true` y el movimiento a `done/` dejan de ser gate; pasan a **efecto** de F7.3.

**F8.2 Genoma (vía `entity-manager` / `process-creator` / `norm-creator`):**
- `feature-pbi-archive` lee `done_gate`. En `linear`/`both` consulta `fetch_issue` (`accept-pr` gana `tracker-operations` en fase «Sincronización y Limpieza»). Linear caído → fail-soft: `Tracker_Sync_Failed` + fallback `git` con `warn`.
- Pre-push: `PBI_DONE_PRESENT` lee la evidencia local `.SddIA/proofs/tracker-done/{tracker_ref}.json` escrita por `tracker-stamp` al sellar `done`; sin red. Sin evidencia y `done_gate: linear` → bloqueo citando el `tracker_ref`.
- Norma `task-closure-documental` y `features-documentation-pattern` 1.2.0 → 1.3.0: el paso «mover PBI en el mismo PR» pasa a condicional (`done_gate: git|both`). Bump vía `norm-creator`; la regla `.cursor/rules/task-closure-documental.mdc` se regenera desde la norma, no se edita a mano.
- `SddIA/evolution/`: entrada vinculando el UUID del laudo reabierto.

**F8.3 Orden de activación (obligatorio):**
1. Desbloqueo: HU-A mergeada, AC-7/AC-8 verdes (condición del spike).
2. F7 mergeado con `done_gate: git`.
3. Core-self en `done_gate: both` ≥ 1 ciclo completo sin `Tracker_Sync_Failed` con `error_code: TRACKER_STATE_DIVERGED`.
4. Laudo del Vértice → `done_gate: linear` **solo en Core-self** (D-I). Expansión a otros proyectos = laudo posterior.

## 4. Decisiones (dictaminadas 2026-10-03)

| ID | Cuestión | Laudo |
|----|----------|-------|
| D-F | Transporte inverso | **(b)** Polling por intervalo + webhook lab (túnel). Sin endpoint público. |
| D-G | Persistencia inversa | **(a)** Un PR diario/lote en `tracker-sync/{project_slug}` vía skill `git-manager`. Push a `default_branch` vetado. |
| D-H | Edición del cuerpo desde Linear | **(a)** Solo con `tracker.labels.editable` (default `sddia-editable`). Protege el cuerpo del PBI/HU; la cascada F2 nunca es destino. |
| D-I | `done_gate: linear` primer ciclo | **(a)** Solo Core-self; expansión tras laudo sin `TRACKER_STATE_DIVERGED`. |

## 5. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-11 | `linear-bridge-watcher` polling con fixture detecta `Backlog → Todo` y materializa `Tracker_Issue_Changed` (`source: poll`); issue sin label `hu`/`pbi` no genera evento. | Test daemon mock |
| AC-12 | Cambio originado por `tracker-stamp`/`forge-pbi`/`refine-hu` (registro saliente o `sddia-sync-id`) **no** produce `Tracker_Issue_Changed`. | Test daemon mock |
| AC-13 | Webhook lab con firma inválida → descartado; firma válida → evento `source: webhook`; payload bruto ausente del evento. | Test daemon mock |
| AC-14 | `tracker-markdown-apply`: `cancelled` → mueve a `todos_done` + `PBI_Cancelled`; `title` → frontmatter sin renombrar; `description` sin label `editable` → `warn` no-op; con label → cuerpo actualizado. | Test proceso mock |
| AC-15 | Markdown modificado tras `linear_updated_at` → gana repo, `Tracker_Sync_Failed` `TRACKER_STATE_DIVERGED`, fichero intacto. | Test proceso mock |
| AC-16 | Cambios inversos en rama `tracker-sync/{project_slug}` vía skill `git-manager`; nunca en `default_branch`. | Test proceso mock |
| AC-17 | `done_gate: git` → `feature-pbi-archive` y pre-push idénticos a hoy. | Suite Core-self |
| AC-18 | `done_gate: linear`, issue `done` → `feature-pbi-archive` acepta sin `pbi_archived`/PBI en `done/`; issue `in_review` → rechaza citando `tracker_ref`. | Test handler mock |
| AC-19 | `done_gate: both`, Linear caído → fallback `git` con `warn` + `Tracker_Sync_Failed`; entrega no bloqueada. | Test handler mock |
| AC-20 | Pre-push con `done_gate: linear` lee `.SddIA/proofs/tracker-done/{tracker_ref}.json`; sin red. | Test hook |
| AC-21 | E2E lab Core-self `done_gate: both`: merge → `done` en Linear → `Tracker_Issue_Changed` → PBI archivado por `tracker-markdown-apply`, sin movimiento manual en el PR de código. | Suite lab |
| AC-22 | Ningún envelope ni evento contiene `LINEAR_WEBHOOK_SECRET`. | Test |
| AC-23 | Manifiesto 1.3.0 con `done_gate: linear|both` valida tras esta HU (sin bump de `contract_version`). | Test contrato |

## 6. Fuera de alcance

- Webhook productivo con endpoint público permanente (D-F).
- Renombrado de ficheros markdown a partir de títulos de Linear.
- Edición desde Linear de la cascada F2.
- Creación de HU/PBI nuevos desde Linear hacia el repo.
- Eliminar el gate de Done: F8 cambia su fuente, no lo suprime.
- `done_gate: linear` fuera de Core-self (D-I).
- Mutación manual de genoma: todo bump pasa por `entity-manager` / `event-creator` / `process-creator` / `daemon-creator` / `norm-creator`.

## 7. Prerrequisitos

- `HU-LINEAR-DIRECT-CYCLE` mergeada; AC-7/AC-8 verdes.
- `LINEAR_WEBHOOK_SECRET` en `env_ref` del proyecto y túnel de laboratorio.
- `SddIA/evolution/`: reapertura del laudo `tracker-done-gate` (UUID de `docs/features/tracker-done-gate/spec.md` o del PBI `[ARQUITECTURA] Tracker — gate Done según spike`).
