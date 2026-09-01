---
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "f8661783-55b7-4419-b659-e96369c02410"
title: "[KAIZEN] Telemetría de CI: Captura remota de colapsos y asimilación local"
format: markdown
version: "1.1.0"
created: "2026-08-30"
updated: "2026-09-01"
status: pending
refinement_status: refinado
priority: media
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-telemetry-observability
persist_ref_suggested: docs/features/kaizen-ci-telemetry-observability
depends_on: []
related:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/daemons/github-bridge-watcher.md
  - SddIA/daemons/github-bridge-watcher/src/main.rs
  - SddIA/sddia-daemon-runtime/src/github_bridge.rs
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/events/telemetry/index.md
  - SddIA/events/telemetry/daemon-heartbeat.md
  - SddIA/events/events-contract.md
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/core/cumulo.paths.json
  - SddIA/norms/external-ai-constraints.md
  - docs/todos/done/[KAIZEN] Tekton — aduana local evolution y veto de polling CI.md
  - docs/todos/done/[KAIZEN] CI — optimizar steps >1 min (verify-compiled-capsules y LanceDB).md
refinement_notes: "v1.1.0 Filtro A: UUID v4 real; purge [cite:N]; sensor Checks API no comentarios PR; Trinidad/emisores; Radamanto no consume CI_Job_Failed; Domain_Entity_Degraded ≠ Kintsugi."
---

# [KAIZEN] Telemetría de CI: Captura remota de colapsos y asimilación local

## Mandato

Erradicar la fuga entrópica de los fallos ciegos de GitHub Actions. Hoy, si un job de `.github/workflows/sddia-index-qa.yml` colapsa, el bus local no recibe estímulo: el Vértice Biológico (o Tekton, en violación de DA-6) debe inspeccionar logs remotos.

Un fallo de CI debe cruzar el umbral, convertirse en telemetría local (`CI_Job_Failed`) y, **solo tras correlación explícita con una entidad de genoma**, alimentar la actuaría de Radamanto. Sin esa correlación, el sistema materializa alerta Kaizen; no revoca RBAC a ciegas.

Complementa `PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL` (DA-6): la captura es del centinela, no de Tekton.

## 0. Dictamen de refinamiento (Filtro A)

Hallazgos sobre v1.0.0. El mandato se conserva; la línea de montaje se corrige.

| # | Afirmación v1.0.0 | Verdad objetiva | Veredicto |
|---|-------------------|-----------------|-----------|
| H1 | `uuid: 8e7f6a5b-4c3d-2e1f-0a9b-8c7d6e5f4a3b` | Nibble de versión `2`, variante `0`. No es UUID v4 RFC 4122. Patrón secuencial. | Alucinación. Sustituido por `f8661783-55b7-4419-b659-e96369c02410`. |
| H2 | Marcadores `[cite: 48]`, `[cite: 44, 45]` | No existen en el repo. Residuo de grounding externo. | Entropía. Purgados. |
| H3 | `github-bridge-watcher` parsea comentarios de PR | El binario lista PRs abiertos (`/pulls?state=open`), valida y delega `github_bridge::process_pr` → `PullRequest_Presented` (dominio). Idempotencia: `BridgeState.processed_pr_urls`. No hay cliente de issue comments. Poll default 30 s. | Inexacto. El sensor nativo es la API de Checks/Actions sobre esos mismos PRs, no un side-channel de comentarios. |
| H4 | Paso `if: failure()` + `gh pr comment` | Workflow `permissions`: `contents: read`, `actions: write`. Falta `pull-requests: write`. Triggers: `push` y `pull_request` (comentario sin PR asociado falla). Forks: `GITHUB_TOKEN` read-only. `concurrency.cancel-in-progress` en push: `if: failure()` no corre en `cancelled`. | Diseño inviable como vía primaria. |
| H5 | «Firma criptográfica o prefijo determinista» | Un prefijo `[SDDIA-CI-TELEMETRY]` no es firma. No hay clave ni sello. | Equivalencia falsa. Idempotencia = estado local del puente (`check_run_id` / `run_id+job_id`). |
| H6 | Emisión a `./.events/telemetry/` por el watcher | `events-contract.md` §6 y `telemetry/index.md`: emisores telemetría = **Solo CLI**. Excepción ya catalogada: `Daemon_Heartbeat` (centinelas, incl. `github-bridge-watcher`). `event-telemetry-subscriptions.json` no tiene clave `CI_Job_Failed`. Clase `ci-job-failed.md` **no existe**. | Trinidad violada si se emite sin forja de Clase + excepción de emisor análoga al latido. |
| H7 | «Modificar `radamanto_batch_core.rs` para suscribirse» | La suscripción vive en `event-telemetry-subscriptions.json`. El batch **solo** procesa `Raw_Execution_Finished`: exige `asset_id`, deduplica en `.SddIA/radamanto/consumed.json`, mapea entidad vía `capsule_id` \| `process_name`. `CI_Job_Failed` sin ese payload rompe el handler. | Orden invertido e incompatibilidad de contrato. |
| H8 | Cuota `max_ci_failures_per_entity` en `radamanto.thresholds.json` | El fichero v1.2.0 declara: `success_rate_min*`, `batch_min_events`, `latency_ms_p95_threshold`, `redemption_success_count`, `max_recovery_attempts`, `abrupt_drop_min_samples`, `cognitive.*`. Esa clave no existe. `directories.agents` es genoma DA-2. | Invención. Forja vía `entity-manager`. |
| H9 | Umbral roto → `Domain_Entity_Degraded` «abre PBI Kintsugi» | `Domain_Entity_Degraded` (emisor exclusivo Radamanto) → Cerbero revoca RBAC + `fix-tool-process` (tool/skill) + sello IOTA. Kintsugi (`System_Fracture_Detected` → `materialize-fracture-pbi`) es otro circuito. Jobs CI **no** son entidades de genoma. `target_entity_from_payload` sin `capsule_id`/`process_name` cae a `unknown-entity`. | Contaminación de protocolos. Un flake de `wasi-runtime-smoke` no puede revocar una entidad inventada. |
| H10 | CA1 «un test fallido» | El workflow tiene **cinco** jobs (`sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`). Colapso = job/check con `conclusion=failure`, no un test unitario aislado. Extraer el *step* colapsado no es trivial con `if: failure()` a nivel de job. | Alcance inflado. |
| H11 | Ruta `.SddIA/radamanto/stats.json` | SSOT `cumulo.paths.json` → `radamanto.stats`. | Correcto. Se conserva. |

## 1. Superficie de impacto (verificada)

| Capa | Ruta | Rol real hoy | Mutación este PBI |
|------|------|--------------|-------------------|
| Workflow | `.github/workflows/sddia-index-qa.yml` | Aduana CI. Sin emisión al bus. | **Opcional** (Ola A0): no es el sensor. No comentar PRs. |
| Puente | `github-bridge-watcher` + `sddia-daemon-runtime::github_bridge` | Oráculo DLT: PRs abiertos → `PullRequest_Presented`. Latido → `Daemon_Heartbeat`. | Sensor Checks/Actions; emisión `CI_Job_Failed`; idempotencia en `BridgeState`. |
| Clase ECST | `SddIA/events/telemetry/` (catálogo: 3 clases) | `System_Vitality_Probed`, `Daemon_Heartbeat`, `Raw_Execution_Finished` | Forja `ci-job-failed.md` vía `entity-manager`. |
| Contrato Trinidad | `events-contract.md` §6, `telemetry/index.md` | Emisor telemetría = CLI, excepción latidos | Extender excepción: centinela `github-bridge-watcher` para `CI_Job_Failed`. |
| Fan-out | `SddIA/core/event-telemetry-subscriptions.json` | `Raw_Execution_Finished` → radamanto-batch + compliance; `Daemon_Heartbeat` → heartbeat-audit | Nueva clave `CI_Job_Failed`. |
| Actuaría | `radamanto_batch_core.rs` + `radamanto.thresholds.json` | Solo `Raw_Execution_Finished` | Handler **distinto** (no reusar `asset_id` de Peaje). Umbral nuevo vía entity-manager. |
| Lab | `.SddIA/.dev/remote_pr_simulation.json` | Fixture PRs | Extender fixture de check fallido para CA sin GitHub real. |

## 2. Línea de montaje (corregida)

### Ola A0 — No hacer (rechazado)

Inyectar `if: failure()` + `gh pr comment` como bus remoto.

Motivo: H4–H5. GitHub ya persiste el colapso como Check Run / Workflow Job. El puente ya autentica contra la API. Duplicar el suceso en la conversación del PR es ruido, exige permisos nuevos y no cubre `push` sin PR ni forks.

### Ola A1 — Sensor en el puente (cruce del umbral)

En el mismo ciclo que ya lista PRs abiertos:

1. Por cada PR candidato, consultar Checks o Actions del `head.sha` (`GET /repos/{owner}/{repo}/commits/{sha}/check-runs` o `.../actions/runs?head_sha=`).
2. Filtrar `conclusion == failure` (no `cancelled`, no `skipped`).
3. Idempotencia: extender `BridgeState` con ids de check-run/job ya asimilados (paridad `processed_pr_urls`). Prohibido reacciones GitHub como sello.
4. Emitir instancia ECST `CI_Job_Failed` en `./.events/telemetry/` (`eda_fractal.telemetry`). Emisor: `github-bridge-watcher`. Patrón de escritura: el mismo que `Daemon_Heartbeat`, no `eda_bus.pending` (eso es dominio).
5. Lab: fixture bajo `.SddIA/.dev/` que simule un check fallido con `SDDIA_LAB_SIMULATE_REMOTE_PR` (o flag gemelo). Sin red.

Payload mínimo de Clase (forja A2; aquí el contrato de intención):

| Campo | Obligatorio | Notas |
|-------|:-----------:|-------|
| `repository` | sí | slug `owner/name` |
| `head_sha` | sí | 40 hex |
| `workflow_name` | sí | p.ej. `sddia-index-qa` |
| `job_name` | sí | uno de los cinco jobs |
| `conclusion` | sí | `failure` |
| `html_url` | sí | URL del job/run |
| `check_run_id` o `run_id`+`job_id` | sí | clave de idempotencia |
| `pr_url` | no | ausente en push a `main` sin PR |
| `step_name` | no | solo si la API lo aporta; no inventar |
| `entity_id` | no | **prohibido inferir**. Solo si existe tabla de correlación explícita (Ola A3). |

DA-6: Tekton no espera, no hace `gh pr checks` en bucle, no rerunea el `headSha`. El centinela opera en su tick (60 s heartbeat / 30 s poll). Fuera del veto DA-5/DA-6.

### Ola A2 — Genoma de evento (DA-2)

Vía `./sddia-run.sh --process entity-manager` (no escritura directa en `directories.events` ni `directories.agents`):

1. Clase `ci-job-failed.md` (`event_family: telemetry`, `event_type: CI_Job_Failed`).
2. Actualizar `SddIA/events/telemetry/index.md`: emisores = CLI **y** centinelas catalogados para clases de ruido periférico (`Daemon_Heartbeat`, `CI_Job_Failed`).
3. Enmendar `events-contract.md` §6: la fila «Solo CLI» deja de ser literalmente exclusiva; documentar la excepción de sensores periféricos (latido + CI remota) sin mezclar con dominio.
4. `event-telemetry-subscriptions.json` → `CI_Job_Failed`: suscriptores a definir en spec (mínimo un consumidor que selle `delivery_state`; Radamanto solo si entra Ola A3).

Sin A2, A1 es emisión huérfana (Argos/compliance tumba la instancia).

### Ola A3 — Gobernanza actuarial (gated)

**No** cablear `Domain_Entity_Degraded` en el primer merge.

Radamanto degrada entidades de genoma (`capsule_id` / `process_name`). Un job de GHA no es ninguna de las dos. Acumular fallos CI como si fueran Peaje Termodinámico contamina `stats.json` y puede revocar `unknown-entity` o un proceso inocente.

Sub-olas:

| Sub | Qué | Condición de entrada |
|-----|-----|----------------------|
| A3.1 | Acumulador **aparte** (p.ej. `.SddIA/radamanto/ci_failures.json` o bucket namespaced `ci:{workflow}:{job}` que **no** pasa por `cerbero-governance-react`) | A1+A2 verdes |
| A3.2 | Umbral en `radamanto.thresholds.json` (nombre a forjar; no asumir `max_ci_failures_per_entity`) vía entity-manager | A3.1 |
| A3.3a | Crónica **sin** mapa entidad → emitir `Kaizen_Alert_Required` (Cúmulo → PBI en `docs/todos/pending/`). Esto es lo que v1.0.0 llamó «PBI Kintsugi» de forma errónea. | A3.2 |
| A3.3b | Crónica **con** mapa explícito `job_name|step → entity_id` versionado → entonces sí `Domain_Entity_Degraded` con `reason` dedicado (no reusar `success_rate_below_threshold`) | Laudo + mapa en spec. Fuera del MVP si el mapa no existe. |

MVP de este PBI = A1 + A2 + A3.1. A3.2/A3.3 quedan en spec como olas siguientes, no como CA bloqueantes del primer PR.

## 3. Criterios de aceptación

- [ ] **CA1 (Cruce de umbral):** Un job del workflow `sddia-index-qa` con `conclusion=failure` en un PR abierto (o fixture lab equivalente) produce exactamente una instancia `CI_Job_Failed` en `./.events/telemetry/` tras un ciclo del puente. Sin comentarios en el PR. Sin intervención de Tekton (`gh pr checks` / `gh run rerun`).
- [ ] **CA2 (Idempotencia):** Dos ciclos consecutivos del puente sobre el mismo `check_run_id` (o `run_id`+`job_id`) no duplican instancias. El sello vive en `BridgeState`, no en reacciones GitHub.
- [ ] **CA3 (Trinidad):** La Clase está catalogada en `telemetry/index.md`; `emitter_agent=github-bridge-watcher` es emisor autorizado de esa Clase (no de `Raw_Execution_Finished`). La instancia no se escribe en `eda_bus.pending` ni en `./.events/domain/`.
- [ ] **CA4 (Cancelación ≠ fallo):** Un job `cancelled` (concurrency) no emite `CI_Job_Failed`.
- [ ] **CA5 (Forja):** `ci-job-failed.md` nace por `entity-manager`. Diff de genoma `events/` + índices coherentes. `radamanto.thresholds.json` **no** se edita a mano.
- [ ] **CA6 (No revocación ciega):** El MVP no emite `Domain_Entity_Degraded` ni muta ratios de entidades de genoma a partir de CI. Un flake no entra en `.SddIA/cerbero/revoked_entities.json`.
- [ ] **CA7 (Lab):** Fixture de check fallido ejecutable con `github-bridge-watcher --once` y `SDDIA_LAB_SIMULATE_REMOTE_PR` (o flag documentado) sin `GITHUB_TOKEN`.

Fuera de MVP (no bloquear el PR de A1+A2+A3.1):

- [ ] **CA8 (Kaizen crónica):** Superar el umbral A3.2 sin mapa de entidad emite `Kaizen_Alert_Required`, no Kintsugi de fractura de proceso.
- [ ] **CA9 (Degradación mapeada):** Solo con mapa job→entidad: `Domain_Entity_Degraded` con `reason` de cuota CI.

## 4. Restricciones duras

- DA-2: Clase de evento y umbrales de agente solo vía `entity-manager`. Código del daemon y `SddIA/core/event-telemetry-subscriptions.json` no están en la tabla DA-2; el evento sí.
- DA-4: ciclo `feature` con `persist_ref` `docs/features/kaizen-ci-telemetry-observability` antes de mutar genoma.
- DA-5/DA-6: el ejecutor no vigila CI remota tras el acuse del CLI; el centinela ya existía.
- No mezclar `CI_Job_Failed` (telemetría) con `PullRequest_Presented` (dominio) en el mismo envelope ni en `process_pr`.
- No usar `CI_Job_Failed` como `Raw_Execution_Finished` sintético (`asset_id` de Peaje).
- Segregación: no reabre `PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN` ni `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION`.

## 5. Laudos abiertos (no bloquean refinamiento)

| ID | Pregunta | Default si no hay laudo al despachar |
|----|----------|--------------------------------------|
| L-API | ¿Check Runs o Actions Jobs como SSOT del sensor? | Check Runs sobre `head.sha` del PR (un round-trip; el puente ya tiene el SHA vía `head.ref`). |
| L-MAP | ¿Existe mapa job/step → entidad de genoma en el MVP? | No. A3.3b aplazado. |
| L-MAIN | ¿Asimilar fallos de `push` a `main` sin PR? | Fuera de MVP. El puente hoy solo lista PRs abiertos. Extensión explícita en spec si se quiere cobertura de `main`. |
