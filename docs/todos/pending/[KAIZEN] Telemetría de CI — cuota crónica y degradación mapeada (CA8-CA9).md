---
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
title: "[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8/CA9)"
format: markdown
version: "1.0.0"
created: "2026-09-01"
updated: "2026-09-01"
status: pending
refinement_status: refinado
priority: media
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-telemetry-chronic-quota
persist_ref_suggested: docs/features/kaizen-ci-telemetry-chronic-quota
parent_pbi: docs/todos/done/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
parent_document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
parent_uuid: "f8661783-55b7-4419-b659-e96369c02410"
parent_pr: "https://github.com/racso80es/SddIA/pull/249"
depends_on:
  - PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
related:
  - docs/todos/done/[KAIZEN] Telemetría de CI - Captura remota de colapsos y asimilación local.md
  - docs/features/kaizen-ci-telemetry-observability/spec.md
  - SddIA/events/telemetry/ci-job-failed.md
  - SddIA/events/domain/kaizen-alert-required.md
  - SddIA/events/domain/domain-entity-degraded.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/agents/radamanto.md
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/cerbero_governance_react_core.rs
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/event-telemetry-subscriptions.json
  - SddIA/core/cumulo.paths.json
  - SddIA/norms/external-ai-constraints.md
refinement_notes: "Nace refinado. Extrae CA8/CA9 del padre (A3.2/A3.3). Filtro A: no reusar contrato DIA de Kaizen_Alert_Required; no inventar max_ci_failures_per_entity; Domain_Entity_Degraded exige success_rate; mapa job→entidad vacío hasta laudo."
---

# [KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8/CA9)

Residual de `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY` (`f8661783-55b7-4419-b659-e96369c02410`, PR #249, merge `26e3366`). El padre cerró A1+A2+A3.1 (CA1–CA7). Este PBI es A3.2 + A3.3.

## Mandato

El ledger `.SddIA/radamanto/ci_failures.json` acumula `CI_Job_Failed` y **no actúa**. Un job que falla de forma crónica debe:

1. **Sin mapa job→entidad:** cruzar un umbral forjado y materializar deuda Kaizen (no fractura de proceso, no revocación RBAC).
2. **Con mapa versionado y laudo:** degradar **solo** la entidad mapeada, con `reason` de cuota CI, sin contaminar `stats.json` de Peaje.

Complementa DA-6: Tekton sigue sin vigilar CI. Actúa Radamanto sobre el ledger ya escrito.

## 0. Dictamen (Filtro A) — qué **no** copiar del padre literal

Los CA8/CA9 del padre son intención correcta y contrato **inexacto**. Verdad en `main` post-#249:

| # | Afirmación (padre §3 / A3.3) | Verdad objetiva | Veredicto |
|---|------------------------------|-----------------|-----------|
| H1 | «Umbral A3.2» / clave `max_ci_failures_per_entity` | `radamanto.thresholds.json` v1.2.0: `success_rate_min*`, `batch_min_events`, `latency_ms_p95_threshold`, `redemption_success_count`, `max_recovery_attempts`, `abrupt_drop_min_samples`, `cognitive.*`. Esa clave **no existe**. `directories.agents` = DA-2. | Inventar el nombre es el mismo error H8 del padre. Forjar objeto namespaced vía `entity-manager`. Propuesta (no hecho): `ci_failures.per_job_limit` + `ci_failures.window` (forma exacta = spec). |
| H2 | Superar umbral → `Kaizen_Alert_Required` | Clase `kaizen-alert-required.md`: familia **domain**, contexto DIA. REQUIRED: `review_id`, `alert_justification`, `implicated_files`. Emisores: `pull-request-review`, `emit-kaizen-alert-required-event`. Cúmulo → `materialize-kaizen-alert-doc` → `PENDING_AUDIT_DOC_{hash8}.md`. | Reusar la Clase rellenando `review_id` sintético es contaminación DIA. **Prohibido.** CA8 no es paridad documental de un PR. |
| H3 | Cúmulo «abre PBI en `docs/todos/pending/`» ante esa alerta | El handler DIA no escribe `[KAIZEN] *.md`. Los PBI Kintsugi salen de `System_Fracture_Detected` → `materialize-fracture-pbi` (circuito distinto; padre H9). | CA8 ≠ Kintsugi. CA8 ≠ `PENDING_AUDIT_DOC_*`. Hace falta **Clase + acción** nuevas (o extensión explícita de ambas, vía EM), no un atajo de evento. |
| H4 | `Domain_Entity_Degraded` con `reason` de cuota CI | REQUIRED: `entity_type`, `entity_id`, `reason`, `success_rate`, `recovery_attempt`. Emisor exclusivo: Radamanto. Fan-out: Cerbero **revoca** `entity_id`. Un job GHA no es entidad de genoma. | Meter `success_rate` inventado (p.ej. `0`) miente al contrato de Peaje. Extender payload (DA-2) o documentar en spec cómo se deriva la tasa **desde el ledger CI** sin tocar `stats.json`. |
| H5 | Mapa `job_name\|step → entity_id` | No existe en repo. `CI_Job_Failed` FORBIDDEN: `entity_id`, `process_name`, `asset_id`. Cinco jobs de `sddia-index-qa` ≠ cinco entidades. | Sin laudo de pares, CA9 es revocación ciega (niega CA6 del padre). Mapa vacío = siempre rama CA8. |
| H6 | Un tick de `radamanto-batch` por cada `CI_Job_Failed` | `process_ci_job_failed`: dedup `check_run_id`, append, stamp success. **Cero** lectura de umbral. | A3.2 se evalúa **después** del append, en la misma rama, sin pasar por `entity_bucket`. |

## 1. Superficie (post-#249)

| Capa | Estado actual | Mutación este PBI |
|------|---------------|-------------------|
| Ledger | `.SddIA/radamanto/ci_failures.json` (`cumulo.paths.json` → `radamanto.ci_failures`). Filas: `check_run_id`, `job_name`, `workflow_name`, `head_sha`, `html_url`, `repository`, `timestamp`, `event_id`. | Contar por clave de cuota; sello de alerta ya emitida (idempotencia). No mezclar con `stats.json`. |
| Umbrales | `SddIA/agents/radamanto.thresholds.json` v1.2.0 | Bloque `ci_failures` vía `entity-manager` (update agente / config). Prohibido Write IDE. |
| Clase alerta crónica | No existe | Forjar evento **nuevo** (default L-ALERT). No clonar DIA. |
| Cúmulo | `Kaizen_Alert_Required` → `PENDING_AUDIT_DOC_*` | Acción nueva (o rama explícita) que materialice `[KAIZEN] CI crónica — …` en `pending/`, consultando **pending y done** (lección fan-out fractura). |
| `Domain_Entity_Degraded` | Peaje + Cerbero | Solo Ola B3, con mapa no vacío y payload honesto. |
| Sensor puente | A1 cerrado | **Fuera.** No reabrir `github-bridge-watcher` salvo bug de ledger. |

## 2. Línea de montaje

### Ola B1 — Umbral (A3.2)

Vía `./sddia-run.sh --process entity-manager`:

1. Añadir configuración de cuota CI a `radamanto.thresholds.json` (nombre forjado en spec; no asumir `max_ci_failures_per_entity`).
2. En `process_ci_job_failed`, **tras** append exitoso: agregar por `job_name` (default L-KEY). Si `count < limit` → return como hoy.
3. Tests: bajo umbral no emite dominio; al cruzar, una sola emisión.

Default de conteo (L-WINDOW, laudo abre): número de filas del ledger con el mismo `job_name`. Ventana temporal / compactación = spec, no este seed.

### Ola B2 — CA8 (A3.3a, sin mapa)

Si umbral cruzado **y** `job_name` **no** está en el mapa (mapa ausente o vacío ≡ no está):

1. Emitir instancia de la **Clase nueva** (familia `domain`, emisor `radamanto` / `radamanto-batch`) hacia `./.events/domain/` (no `telemetry/`, el `CI_Job_Failed` ya es el ruido periférico).
2. Payload: identidad de cuota (`job_name`, `workflow_name`, `count`, `limit`, `html_url` representativa). FORBIDDEN: `entity_id` inferido, `asset_id`, `review_id` falso.
3. Fan-out → Cúmulo materializa PBI Kaizen en `docs/todos/pending/` (no `System_Fracture_Detected`, no `PENDING_AUDIT_DOC_*`).
4. Idempotencia: segundo `CI_Job_Failed` del mismo `job_name` bajo la misma ventana **no** reemite ni recrea PBI si ya existe en pending **o** done (mismo `document_id` derivado).
5. Cerbero no se suscribe a esta Clase.

### Ola B3 — CA9 (A3.3b, gated)

Condición de entrada: spec con mapa versionado **con al menos un par laudoado**. Artefacto propuesto (ruta final = spec, DA-2 si cae en `SddIA/core/` o agentes): tabla `job_name` → `{ entity_type, entity_id }`.

| Job (`sddia-index-qa`) | Par en este seed |
|------------------------|------------------|
| `sddia-index-integrity` | **Ninguno.** No laudar. |
| `eda-iota-smoke-simulate` | **Ninguno.** |
| `wasi-runtime-smoke` | **Ninguno.** |
| `eda-bus-e2e-smoke` | **Ninguno.** |
| `eda-iota-physical` | **Ninguno.** |
| cualquier otro `workflow_name` | **Ninguno.** |

Hasta laudo: mapa = `{}` → 100 % tráfico B2. Implementar el **cable** (lookup + emisión `Domain_Entity_Degraded`) en el mismo ciclo solo si el mapa tiene entradas; si no, el lookup vacío es el test de CA9-negativo (no Degraded).

Si hay par laudoado:

1. `Domain_Entity_Degraded` con `entity_id`/`entity_type` del mapa. `reason` dedicado (propuesta: `ci_failure_quota_exceeded`), **no** `success_rate_below_threshold`.
2. `success_rate` / `recovery_attempt`: o bien extensión de Clase (OPTIONAL cuando `reason` es cuota CI) vía EM, o bien derivación documentada `1 - count/window` **sin** escribir `stats.json`. Elegir en spec; prohibido `success_rate: 0` opaco.
3. Cerbero revocará esa entidad. Eso es el coste del laudo, no un efecto colateral «a ver qué pasa».
4. Prohibido degradar `unknown-entity`. Prohibido Kintsugi.

MVP de **este** PBI = B1 + B2 + lookup vacío (CA9-negativo). B3 con pares reales **no** bloquea el primer PR.

## 3. Criterios de aceptación

- [ ] **CA8 (Kaizen crónica):** Tras ≥ `limit` filas distintas (`check_run_id`) del mismo `job_name` en el ledger, **sin** entrada de mapa, existe exactamente una instancia de la Clase nueva en `./.events/domain/` y un PBI `[KAIZEN]` en `docs/todos/pending/` (o skip idempotente si ya está en pending/done). Cero `System_Fracture_Detected`. Cero `PENDING_AUDIT_DOC_*` atribuible a este flujo. Cero mutación de `.SddIA/cerbero/revoked_entities.json`. Cero escritura en `.SddIA/radamanto/stats.json`.
- [ ] **CA8-IDEM:** Un `CI_Job_Failed` adicional del mismo `job_name` no duplica evento ni PBI.
- [ ] **CA8-FORJA:** Umbral y Clase (y acción Cúmulo si aplica) nacen por `entity-manager`. Diff `radamanto.thresholds.json` no es Write IDE.
- [ ] **CA9-NEG (mapa vacío):** Fixture de cuota superada + mapa `{}` → camino CA8. Cero `Domain_Entity_Degraded`.
- [ ] **CA9 (Degradación mapeada):** **Solo** con par laudoado en spec: `Domain_Entity_Degraded` con ese `entity_id` y `reason` de cuota CI. Cerbero puede revocar **esa** entidad. Jobs no mapeados siguen CA8. Fuera del primer PR si L-MAP sigue vacío.

## 4. Restricciones duras

- DA-2: eventos, umbrales de agente, acciones Cúmulo, índices de suscripción de **dominio** si la Clase es nueva: `entity-manager`. `radamanto_batch_core.rs` no está en la tabla DA-2.
- DA-4: ciclo `feature` con `persist_ref` `docs/features/kaizen-ci-telemetry-chronic-quota` antes de mutar genoma.
- DA-5/DA-6: sin `gh pr checks` / `gh run rerun`. El estímulo es el ledger local.
- Trinidad: no convertir `CI_Job_Failed` en `Raw_Execution_Finished`. No emitir alerta crónica a `eda_fractal.telemetry`.
- No reabrir sensor Check Runs ni CA1–CA7 del padre.
- No usar `Kaizen_Alert_Required` v1.0.0 tal cual (H2).
- Segregación: no reabre `PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN`, `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION`, ni rehab `revoked_entities` de PPR.

## 5. Laudos (no bloquean el refinamiento; sí el despacho de B3)

| ID | Pregunta | Default si no hay laudo al despachar |
|----|----------|--------------------------------------|
| L-ALERT | ¿Clase nueva vs extender `Kaizen_Alert_Required` (`alert_kind=ci_chronic`, `review_id` opcional, Cúmulo bifurcado)? | **Clase nueva.** Extender DIA mezcla semánticas y obliga a `review_id` fantasma. |
| L-WINDOW | ¿Cuota = filas eternas del ledger, ventana temporal, o últimas N? | Filas actuales del ledger por `job_name`. Compactación = otro PBI. |
| L-KEY | ¿Agregar por `job_name` o `workflow_name`+`job_name`? | `job_name` (los cinco jobs de `sddia-index-qa` no colisionan). |
| L-LIMIT | ¿Valor inicial de `per_job_limit`? | `3` (lab/tests). Producción = spec. |
| L-MAP | ¿Pares job→entidad en el primer PR? | **Ninguno.** CA9-NEG sí; CA9 positivo espera laudo explícito por par. |
| L-DEGRADED-RATE | ¿`success_rate` sintético o Clase extendida? | Extender Clase (OPTIONAL condicionado a `reason`) si B3 entra; no `0` opaco. |

## 6. Fuera

- Asimilar `push` a `main` sin PR (L-MAIN del padre).
- Comentarios GitHub / `if: failure()`.
- Umbrales de Peaje (`success_rate_min`, `abrupt_drop_*`) aplicados al ledger CI.
- Auto-laudo de mapas «obvios» (un smoke WASI no es el tool WASI).
