---
document_id: PBI-KAIZEN-CI-TELEMETRY-CHRONIC-QUOTA
uuid: "166c91f9-7378-4766-b6fe-ff5e7eee382f"
title: "[KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8/CA9)"
format: markdown
version: "1.2.0"
created: "2026-09-01"
updated: "2026-09-05"
status: done
refinement_status: implemented
persist_ref: docs/features/kaizen-ci-telemetry-chronic-quota
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
  - docs/features/kaizen-ci-telemetry-observability/validacion.md
  - SddIA/events/telemetry/ci-job-failed.md
  - SddIA/events/domain/kaizen-alert-required.md
  - SddIA/events/domain/domain-entity-degraded.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/fractal_bus.rs
  - SddIA/engine/execute-process/src/engine/cerbero_governance_react_core.rs
  - SddIA/engine/execute-process/src/engine/fix_tool_process_core.rs
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/ecst_validation.rs
  - SddIA/actions/materialize-kaizen-alert-doc.md
  - SddIA/actions/materialize-fracture-pbi.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/cumulo.paths.json
  - SddIA/norms/external-ai-constraints.md
refinement_notes: "v1.2.0 Filtro A (segunda pasada sobre v1.1.0): 1) Colisión de clave ci_failures en load_radamanto_config (path string ≠ bloque de umbral); 2) Fórmula success_rate (failures/window o limit/samples) es incoherente — el ledger solo guarda fallos; 3) Rechazada la vía OPTIONAL sobre Domain_Entity_Degraded (debilita Peaje); 4) Fan-out real de Domain_Entity_Degraded = Cerbero + Dedalo + IOTA; resolve_entity_type default tool; 5) OID de merge 26e3366 no consta en validacion.md del padre; 6) Contrato de retorno process_ci_job_failed y test thresholds_110_process_intact; 7) Acción Cúmulo en ecosystem-evolution (Cúmulo no declara quality-assurance); 8) Sello actuarial solo tras emisión OK; 9) L-RESET: sello no reabre PBI tras done/."
---

# [KAIZEN] Telemetría de CI — cuota crónica y degradación mapeada (CA8/CA9)

Residual de `PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY` (`f8661783-55b7-4419-b659-e96369c02410`, [PR #249](https://github.com/racso80es/SddIA/pull/249)). Acta del padre: `docs/features/kaizen-ci-telemetry-observability/validacion.md` (`global: APTO`, CA1–CA7). **No afirmar OID de merge**: no está en ese acta. El padre cerró A1+A2+A3.1. Este PBI aborda A3.2 + A3.3a; A3.3b queda como cable negativo (CA9-NEG) hasta laudo de mapa.

## Mandato

El ledger `.SddIA/radamanto/ci_failures.json` acumula `CI_Job_Failed` y **no evalúa cuota**. Un job crónico debe:

1. **Sin mapa job→entidad (default de partida):** al superar `per_job_limit` filas distintas (`check_run_id`) para ese `job_name`, Radamanto emite `CI_Chronic_Failure_Detected` y Cúmulo materializa deuda Kaizen en `docs/todos/pending/`. No es Kintsugi. No es DIA. No es revocación RBAC.
2. **Con mapa versionado y laudo (gated B3, fuera del primer PR si el mapa sigue `{}`):** degradar **únicamente** la entidad mapeada vía `Domain_Entity_Degraded` con `reason: ci_failure_quota_exceeded`, sin escribir `stats.json`.

Complementa DA-6: Tekton no vigila CI. El estímulo es el ledger local ya persistido por `radamanto-batch`.

---

## 0. Dictamen (Filtro A)

Los CA8/CA9 del padre y la v1.1.0 de este PBI conservan la intención. Varias hipótesis de implementación **no resisten el Core post-#249**.

| # | Afirmación previa | Verdad objetiva | Veredicto |
|---|-------------------|-----------------|-----------|
| **H1** | Umbral vía `entity-manager` / clave `max_ci_failures_per_entity` | `radamanto.thresholds.json` v1.2.0 no tiene esa clave. `entity-manager` piloto: 10 clases (`process`, `agent`, `skill`, `tool`, `action`, `norm`, `codex`, `event`, `suite`, `daemon`), todas `{name}.md`. `agent-creator` forja `{agent}.md`, no JSON companion. Precedente real: bloque `cognitive` (telemetría cognitiva) se incorporó al mismo JSON en ciclo feature, no por un creator de JSON. | **Alucinación de creator.** Mutar el JSON bajo DA-4. `SddIA/agents/` está en DA-2 y en `GENOME_PREFIXES` del pre-commit (gate de huérfanos, no de creator). `{radamanto}.md` sí es DA-2 vía `agent-creator`. |
| **H2** | Superar umbral → `Kaizen_Alert_Required` (A3.3a del padre) | Clase DIA: REQUIRED `review_id`, `alert_justification`, `implicated_files`. Acción `materialize-kaizen-alert-doc` → `PENDING_AUDIT_DOC_{hash8}.md`. | **Prohibido reusar.** CA8 no es paridad documental. Clase nueva: `CI_Chronic_Failure_Detected`. |
| **H3** | Reusar acción Cúmulo existente / «solo DIA y Fractura» | Cúmulo tiene más acciones (`sync-entity-index`, etc.). Los **materializadores de deuda** sí son dos: DIA y Kintsugi. | Acción nueva `materialize-ci-chronic-failure-pbi`. Handler nativo paridad `materialize_fracture_pbi` / `materialize_kaizen_alert_doc`. `CONSUMER_SKIP_FORGE_ACTIONS`. |
| **H4** | `OPTIONAL` condicionado a `reason`, o tasa `(1.0 - failures/window)` / `(1.0 - limit/samples)` | `ecst_validation.rs` parsea `### REQUIRED` / `### OPTIONAL` sin condicionales. El ledger **solo almacena fallos**; no hay éxitos ni ventana de intentos. `1 - failures/count(failures)` = 0 siempre. `1 - limit/samples` **sube** al acumular más fallos. | **Fórmulas incoherentes.** No mover `success_rate` a OPTIONAL (debilita el Peaje). En B3: `success_rate: 0.0` (sentinela documentada), `recovery_attempt: 0`, `reason: ci_failure_quota_exceeded`. |
| **H5** | Mapa ad-hoc job→entidad | No existe. Cinco jobs de `sddia-index-qa` (`sddia-index-integrity`, `eda-iota-smoke-simulate`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`); único workflow bajo `.github/workflows/`. Cubren suites, no una entidad. | Mapa en `thresholds.ci_failures.job_entity_map: {}`. Sin laudo, 100 % CA8. Mapear a ciegas viola CA6 del padre. |
| **H6** | Evaluar cuota en `entity_bucket` / `stats.json` | `process_ci_job_failed` se despacha **antes** de exigir `asset_id`. Append + `write_json_atomic`. Return `{ ok: true, kind: "ci_job_failed", check_run_id }`. Dedup por `check_run_id`. No toca `stats.json`. | Cuota sobre filas del ledger, en esa rama. **Prohibido** cambiar el contrato de retorno: añadir `status`, no sustituir `ok`/`check_run_id` (test `ci_job_failed_writes_ledger_not_stats`). |
| **H7** | Idempotencia difusa Radamanto/Cúmulo | Jurisdicción, no incapacidad de I/O. Radamanto no debe escanear `docs/todos/`. Cúmulo no debe leer el ledger privado. | Radamanto sella `alerts` en el ledger **después** de `write_fractal_event` OK. Cúmulo deduplica por `document_id` en `pending/` y `done/`. |
| **H8** | «Defaults en `load_radamanto_config`» para el bloque de cuota (v1.1.0) | `cfg["ci_failures"]` **ya es la ruta** `".SddIA/radamanto/ci_failures.json"`. Tras el merge, `cfg["thresholds"]` es el **objeto** JSON (no la ruta). El default in-code del objeto (líneas 80–88 de `fractal_bus.rs`) **no** incluye `ci_failures` ni `cognitive`. | **Colisión de clave.** Cuota = `cfg["thresholds"]["ci_failures"]`. Ledger path = `cfg["ci_failures"]`. Añadir default anidado `{ per_job_limit: 3, job_entity_map: {} }` en el objeto umbrales in-code (el test `ci_job_failed_writes_ledger_not_stats` no copia el JSON). |
| **H9** | «Solo Cerbero reacciona a `Domain_Entity_Degraded`» (v1.1.0) | `event-domain-subscriptions.json`: Cerbero `cerbero-governance-react` + Dedalo `fix-tool-process` (`entity_type=tool`) + Radamanto `iota-immutable-publisher`. `resolve_entity_type` cae a `"tool"` si el id no es proceso ni `tipo:id`. | Fan-out triple. Prohibido reusar `governance_payload` sin `entity_type` del mapa. Entidad inexistente → fallback CA8, no `unknown-entity` ni tool fantasma. |
| **H10** | Acción Cúmulo `context: quality-assurance` (v1.1.0) | Cúmulo `allowed_policies`: `knowledge-management`, `ecosystem-evolution`. **No** declara `quality-assurance`. `materialize-fracture-pbi` usa `ecosystem-evolution`. `materialize-kaizen-alert-doc` usa `quality-assurance` (precedente DIA, no copiar). | Acción nueva: `ecosystem-evolution`. Clase evento: `quality-assurance` (emisor Radamanto sí declara esa política). |
| **H11** | Fan-out síncrono Cúmulo desde `radamanto-batch` | `emit_domain_and_route` escribe fractal domain; `route-domain` solo si `SDDIA_LAB_ROUTE_SYNC`. Producción: watcher. | CA8 parte emisión (Radamanto) y materialización (handler Cúmulo + test de acción). Lab con flag para E2E. |

---

## 1. Superficie de Afectación

| Componente | Estado en `main` post-#249 | Mutación este PBI |
|------------|----------------------------|-------------------|
| **Ledger** | `{ "failures": [ ... ] }` vía `radamanto.ci_failures`. Dedup `check_run_id`. | Añadir `"alerts": { "<job_name>": { "emitted_at", "count_at_alert", "event_type" } }`. Cero `stats.json`. |
| **Umbrales** | `radamanto.thresholds.json` v1.2.0 | v1.3.0, bloque anidado `ci_failures`. Parche DA-4 del JSON (no EM). Actualizar test `thresholds_110_process_intact` (`version == "1.2.0"` → `"1.3.0"`). Default in-code en `fractal_bus.rs`. |
| **Agente Radamanto** | `radamanto.md` inputs: batch «exclusivamente» `Raw_Execution_Finished`; outputs sin crónica CI. Deriva residual del padre. | Update vía `entity-manager` (`entity_class: agent`): inputs admiten ledger `CI_Job_Failed`; outputs + `CI_Chronic_Failure_Detected`. |
| **Clase** | Inexistente. | Forja EM `event`: `ci-chronic-failure-detected.md`. Factory ya cita `event-domain-subscriptions.json` (no el legado `event-subscriptions.json` de la Clase DIA). |
| **Acción Cúmulo** | Materializadores de deuda: DIA + fractura. | Forja EM `action`: `materialize-ci-chronic-failure-pbi.md`. Módulo Rust + `mod.rs` + `actions::try_run_native`. `CONSUMER_SKIP_FORGE_ACTIONS`. |
| **Suscripciones** | Sin clave crónica CI. `CI_Job_Failed` → `radamanto-batch` (no tocar en este PBI). | `CI_Chronic_Failure_Detected` → Cúmulo / `materialize-ci-chronic-failure-pbi`. `SddIA/core/` no es DA-2. |
| **Engine** | `process_ci_job_failed` appendea y retorna. | Tras append: contar por `job_name`; lookup mapa; emitir; sello post-OK. Reusar `build_domain_event` + `emit_domain_and_route`. |
| **Cerbero / Dedalo / IOTA** | Suscritos a `Domain_Entity_Degraded`. No conocen la clase crónica. | **No** suscribirlos a `CI_Chronic_Failure_Detected`. B3 (mapa no vacío): asumen el fan-out completo de Degraded. |

---

## 2. Línea de Montaje

### Ola B1 — Umbral y conteo (A3.2)

1. **`radamanto.thresholds.json` v1.3.0** (parche DA-4, no EM):
   ```json
   "ci_failures": {
     "per_job_limit": 3,
     "job_entity_map": {}
   }
   ```
2. **`load_radamanto_config`** (`fractal_bus.rs`): el default del **objeto** `thresholds` incorpora el mismo bloque. Lectura en motor: `thresholds["ci_failures"]["per_job_limit"]` con fallback `3`. **Prohibido** leer `cfg["ci_failures"]` como objeto de cuota.
3. **`process_ci_job_failed`** (tras append atómico, sobre filas ya deduplicadas por `check_run_id`):
   - `count < per_job_limit` → return existente + `"status": "accumulated"`.
   - `count >= per_job_limit` y sello presente → `"status": "alert_skipped"`.
   - `count >= per_job_limit` sin sello → lookup `job_entity_map[job_name]`. Ausente/`{}` → emitir `CI_Chronic_Failure_Detected`. Presente y entidad verificable → rama B3. Presente e inválida → CA8 (no Degraded).
   - Sello **solo** si `write_fractal_event` OK. Si la emisión falla, no sellar (reintento en el siguiente fallo).
4. **Tests:** conteo 1–2 = cero domain; 3 = una emisión; 4 = skip. Ledger `stats.json` intacto. Contrato `ok: true` + `kind` + `check_run_id` conservado.

### Ola B2 — Materialización CA8 (A3.3a)

Cuando la cuota se cruza y no hay par válido en el mapa:

1. **Forja EM**
   - Evento `ci-chronic-failure-detected`:
     - `event_type: CI_Chronic_Failure_Detected`
     - `event_family: domain`
     - `event_context: quality-assurance`
     - `emitter_agents: ["radamanto"]`
     - REQUIRED: `job_name`, `workflow_name`, `failure_count`, `quota_limit`, `sample_check_run_id`, `sample_html_url`, `repository`, `head_sha`
     - OPTIONAL: `run_id`, `step_name`
     - FORBIDDEN: `entity_id`, `asset_id`, `review_id`, `process_name`
   - Acción `materialize-ci-chronic-failure-pbi`:
     - `action_context: ecosystem-evolution`
     - `capabilities`: `ci-chronic-pbi-materialization`, `delegate-filesystem-manager`, `cumulo-debt-ledger`
     - Nombre no interseca `actions-contract` §2bis.
2. **Suscripción** en `event-domain-subscriptions.json`:
   ```json
   "CI_Chronic_Failure_Detected": [
     {
       "agent": "cumulo",
       "action": "materialize-ci-chronic-failure-pbi",
       "intent": "Materializar PBI Kaizen por fallo crónico de CI en docs/todos/pending/."
     }
   ]
   ```
   Añadir la acción a `CONSUMER_SKIP_FORGE_ACTIONS`.
3. **Handler nativo** `engine::materialize_ci_chronic_failure_pbi::run`:
   - `document_id = "PBI-KAIZEN-CI-CHRONIC-" + slug(job_name).to_uppercase()`
   - Ruta: `docs/todos/pending/[KAIZEN] CI crónica — {slug}.md`
   - Idempotencia: si ese `document_id` existe en `pending/` o `done/` → `{ success: true, status: "already_open_or_done" }` sin reescribir.
   - YAML PBI: `document_id`, `uuid` v4, `title`, `format: markdown`, `version: "1.0.0"`, `status: pending`, `priority: media`, `process: feature`, `type: kaizen`.
   - Cuerpo: tabla `job_name`, `workflow_name`, `failure_count`, `quota_limit`, `head_sha`, `html_url`.
4. Cero suscriptores Cerbero/Dedalo/IOTA a esta Clase. Cero `PENDING_AUDIT_DOC_*`. Cero `System_Fracture_Detected`. Cero `Kaizen_Alert_Required`.

### Ola B3 — CA9 (A3.3b, gated)

Entrada: spec con al menos un par laudoado. **Este PBI entrega el lookup.** El mapa del primer PR es `{}`.

1. **CA9-NEG (MVP):** cuota superada + mapa vacío → rama CA8. Cero `Domain_Entity_Degraded`.
2. **Activación posterior (no bloquear el primer PR):**
   ```json
   "job_entity_map": {
     "nombre-de-job": { "entity_type": "process", "entity_id": "id-real-del-genoma" }
   }
   ```
   - `entity_type` del mapa es SSOT. No usar el default `"tool"` de `resolve_entity_type`.
   - Verificar que la entidad existe en genoma. Si no: CA8.
   - Payload Degraded: `reason: ci_failure_quota_exceeded`, `success_rate: 0.0`, `recovery_attempt: 0`. Cero escritura a `stats.json`.
   - Fan-out inevitable: Cerbero revoca; si `entity_type=tool`, Dedalo abre sandbox; IOTA sella. Laudar pares `tool`/`skill` solo si esa reparación es el objetivo. Prohibido `unknown-entity`.

---

## 3. Criterios de Aceptación

- [x] **CA8 (Kaizen crónica sin mapa):** ≥ `per_job_limit` filas distintas (`check_run_id`) para un `job_name`, mapa vacío o sin par válido → exactamente un `CI_Chronic_Failure_Detected` en `./.events/domain/`. Handler Cúmulo materializa `[KAIZEN] CI crónica — …` en `docs/todos/pending/` (unidad del handler; E2E con `SDDIA_LAB_ROUTE_SYNC` o watcher de lab). Cero `System_Fracture_Detected`. Cero `Kaizen_Alert_Required`. Cero `PENDING_AUDIT_DOC_*`. Cero mutación de `revoked_entities.json` y de `stats.json`.
- [x] **CA8-IDEM:**
  - Actuarial: fallo ulterior del mismo `job_name` no reemite mientras exista sello. Sello ausente si la emisión falló.
  - Documental: `document_id` ya en `pending/` o `done/` → no-op exitoso.
- [x] **CA8-FORJA:** Clase y acción vía `entity-manager`. `radamanto.md` residual (agent-creator update regenera UUID). `radamanto.thresholds.json` v1.3.0 por parche DA-4 (no EM). `sddia-qa audit-eda-coverage --scan` → `orphan_count: 0`.
- [x] **CA8-FILTRO-C:** acción en `CONSUMER_SKIP_FORGE_ACTIONS`. Perfil `consumer` omite la forja.
- [x] **CA8-CONTRACT:** test `ci_job_failed_writes_ledger_not_stats` sigue verde (`ok`, `kind`, dedup). Test de versión de umbrales apunta a `1.3.0`.
- [x] **CA9-NEG:** fixture de cuota superada + `job_entity_map: {}` → CA8, cero `Domain_Entity_Degraded`.
- [ ] **CA9 (gated, no bloquea el primer PR):** par laudoado → Degraded de esa entidad, `reason: ci_failure_quota_exceeded`, ECST válido, `success_rate: 0.0`. Jobs no mapeados siguen CA8.

---

## 4. Restricciones Duras

- **DA-1 / DA-2:** Clase evento, acción Cúmulo y update de `radamanto.md` vía `./sddia-run.sh --process entity-manager`. Prohibido Write IDE en `SddIA/events/`, `SddIA/actions/`, `SddIA/agents/*.md`.
- **JSON companion:** `radamanto.thresholds.json` no tiene creator. Parche en rama feature con `persist_ref` activo (DA-4). No inventar `entity-manager` sobre JSON.
- **DA-4:** ciclo `feature`, rama `feat/kaizen-ci-telemetry-chronic-quota`, `persist_ref` `docs/features/kaizen-ci-telemetry-chronic-quota` **antes** de mutar genoma o motor.
- **DA-5 / DA-6:** sin `gh pr checks` ni `gh run rerun`. Estímulo = ledger local.
- **Trinidad:** no convertir `CI_Job_Failed` en `Raw_Execution_Finished`. No inyectar `asset_id`. `CI_Chronic_Failure_Detected` → `eda_fractal.domain` (`./.events/domain/`).
- **Anti-contaminación:** no reusar `Kaizen_Alert_Required` ni `System_Fracture_Detected`.
- **Segregación:** no reabre `PBI-KAIZEN-CI-STEP-RUNTIME-GT-1MIN`, `PBI-KAIZEN-CI-WORKFLOW-OPTIMIZATION`, ni rehabilitación de `revoked_entities`. No mutar `github-bridge-watcher` ni CA1–CA7.

---

## 5. Laudos

| ID | Pregunta | Resolución |
|----|----------|------------|
| **L-ALERT** | ¿Clase nueva vs `Kaizen_Alert_Required`? | Clase nueva `CI_Chronic_Failure_Detected`. |
| **L-ACTION** | ¿Acción nueva vs bifurcar DIA? | Acción nueva. Produce PBI Kaizen, no `PENDING_AUDIT_DOC_*`. |
| **L-ACTION-CTX** | ¿Context de la acción? | `ecosystem-evolution` (políticas de Cúmulo + paridad fractura). Evento: `quality-assurance`. |
| **L-WINDOW** | ¿Cuota? | Conteo de filas del ledger por `job_name` (ya dedup `check_run_id`). Sin ventana temporal. Purga = PBI operativo posterior. |
| **L-KEY** | ¿Agregación? | `job_name`. En este repo los cinco jobs son únicos. Un segundo workflow homónimo colisionaría; fuera de alcance. |
| **L-LIMIT** | ¿Límite? | `3` (`ci_failures.per_job_limit`). |
| **L-MAP** | ¿Pares en el primer PR? | Ninguno (`{}`). Cable CA9-NEG. |
| **L-DEGRADED-RATE** | ¿ECST en B3? | `success_rate: 0.0`, `recovery_attempt: 0`. El ledger no tiene éxitos. Prohibido OPTIONAL sobre la Clase Peaje. |
| **L-DEGRADED-FANOUT** | ¿Quién reacciona en B3? | Los tres suscriptores actuales. No filtrar por `reason` en este PBI. Evitar mapear `tool`/`skill` salvo laudo explícito de reparación. |
| **L-RESET** | ¿Reabrir tras `done/`? | No en este PBI. El sello actuarial impide reemisión aunque el PBI esté archivado. Reset = purga/compactación futura del ledger. |
| **L-THRESH-PATH** | ¿Dónde vive la cuota en runtime? | `cfg["thresholds"]["ci_failures"]`. `cfg["ci_failures"]` es solo la ruta del ledger. |

---

## 6. Fuera de Alcance

- Centinela `github-bridge-watcher` y CA1–CA7 del PR #249.
- Pushes a `main` sin PR.
- Comentarios GitHub / `gh run rerun` / `if: failure()`.
- Aplicar umbrales de Peaje (`success_rate_min`, `abrupt_drop_*`) a CI.
- Mapeo automático de jobs-suite (`sddia-index-integrity` ≠ un tool).
- Compactación del ledger y reapertura post-`done/` (L-RESET).
- Filtrar Dedalo/IOTA por `reason` (L-DEGRADED-FANOUT).
- Mutar `event-telemetry-subscriptions.json` (el intent «no Domain_Entity_Degraded» sigue siendo verdadero con mapa vacío).
