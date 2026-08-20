---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-feature-cerbero + fail-soft-padre-DCC + poda-telemetria-hueca
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
  - A2
  - A3
correlation_id: ""
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
---

# Spec — feature-revoked-registry-rehab

## 1. Misión técnica

Rehabilitar el proceso `feature` en Cerbero **y** cortar re-muerte: fail-soft del padre cuando el DCC hijo ya cruzó umbral físico; poda de samples huecos en el batch de supervivencia Radamanto. Un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia 2026-08-20)

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `permanent.feature` · `entity_type: process` · `reason: max_recovery_attempts_exceeded` · `since: 2026-08-19T07:59:05Z`. Ausente de `revoked`. |
| Radamanto bucket raíz `feature` | `deprecated` · `recovery_attempts: 4` · `degraded_at: 2026-08-13T06:08:38Z` · ventana 11 samples · rate 0,727 (pasa umbral process 0.70). |
| Causalidad de muerte | Peaje binario de 7 fases + DCC anidado sin `fail_soft` en padre + samples lab (40 ms) inflando rate. No es `success_rate_below_threshold`. |
| Padre `feature` hoy | `capsule_feature_invoke_delivery_close` usa `invoke_process` → cualquier `success ≠ true` del hijo = `Err`. `residual_runner` rama `feature`\|`bug-fix`: `Err` → `failed` **sin** `fail_soft`. |
| DCC ya Kintsugi | `mark_fail_soft_if_secondary` + cola higiene/impacto/telemetría post-`pr_url`/`delivery_push`. El padre **no** hereda esa cicatriz. |
| Telemetría | `derive_cycle_phase` vive en PEC. Payload `Raw_Execution_Finished` **no** porta `cycle_phase` ni `lab_hollow`. Batch empuja todo a `samples`. |
| Agregador | `aggregate_execution_terminal` ya trata `simulated`/`skipped`/`awaiting*` y `fail_soft` como neutrales. **No mutar.** |
| Umbrales | `radamanto.thresholds.json` **1.1.0** intactos (`process: 0.70`, `max_recovery_attempts: 3`). |
| Laterales | `revoked.bug-fix`, `revoked.emit-pr-audited-event` — fuera. Fósiles stats `entities.feature` / `process:feature` — no mutar. |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un PR. Prohibido despachar `bug-fix` satélite. |
| **L-WAVES** | A1 instancia + A2 motor + A3 motor en el mismo ciclo. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = `.SddIA/` no genoma. Evidencia en `execution.md`. Prohibido versionar `radamanto.revoked_entities` / `radamanto.stats` (`cumulo.paths.json` → `radamanto.*`) como cierre del PR. |
| **L-CERBERO** | Borrar clave `permanent.feature`. Verificar `revoked.feature` ausente. Cerbero no tiene estado `healthy`. |
| **L-STATS** | Reset **solo** bucket raíz `feature`. Absoluto: `healthy`, `recovery_attempts: 0`, `consecutive_success_count: 0`, `degraded_at: null`, `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY`, `rehabilitated_at` ISO. Ventana vacía o ≤ últimos éxitos runtime (`duration_ms` real, `exit_code: 0`). |
| **L-ONTOLOGY** | Conservar `entity_type: process`. No regresionar a `tool`. |
| **L-AGGREGATOR** | `phase_terminal::aggregate_execution_terminal` **intacto**. |
| **L-FAILSOFT-PADRE** | Kintsugi en runner de `feature` (cápsula + residual), no en el agregador. |
| **L-INVOKE-FULL** | Sustituir `invoke_process` por `invoke_process_full` en `capsule_feature_invoke_delivery_close`. Inspeccionar envelope completo. Propagar `pr_url` / `delivery_push` / `snapshot_commit_hash` al state padre **aunque** `success ≠ true`. |
| **L-DCC-DATA** | Añadir `delivery_push` al `data` del envelope DCC (hoy copia `pr_url` pero no `delivery_push`). Predicado padre simétrico a `mark_fail_soft_if_secondary`. |
| **L-SOFT-OK** | Si umbral físico hijo **y** fallo secundario: la cápsula retorna `Ok` con `status: failed`, `fail_soft: true`, `handler: feature-delivery-close`. No `Err`. El residual **copia** el flag; no lo inventa en el `Err` path. |
| **L-CAUSAL-ERR** | Si el hijo falla **sin** umbral físico, o el fallo es causal (git/init padre; snapshot; push; apertura PR; Argos `block`; agentes 2–5 reales; archivo PBI runtime): `Err` o `failed` **sin** `fail_soft`. |
| **L-SYMMETRY** | `execute_feature_phase` es compartido `feature`\|`bug-fix`. El parche de invocación DCC aplica por simetría de código. **No** rehab Cerbero/stats de `bug-fix`. Residual: si la cápsula ya marca `fail_soft` en `Ok`, no duplicar lógica en el `Err` de `bug-fix`. |
| **L-PRED-REUSE** | Predicado secundario = reutilizar semántica de `mark_fail_soft_if_secondary` + `soft_err` DCC (timeout / telemetry / receipt / validación) + `thermodynamic_toll.telemetry_io_failed`. Extraer helper puro testeable (p. ej. `feature_parent_dcc_fail_soft_eligible`) para no copiar strings sueltos. |
| **L-TELEMETRY** | `Raw_Execution_Finished.payload` porta `cycle_phase` (misma `derive_cycle_phase`; fallo → `"failed"`). Porta `lab_hollow: true` cuando aplique. PEC **sigue**; no castrar emisión. |
| **L-HOLLOW** | Hueco = `cycle_phase` ∈ {`initialized`, `awaiting_agents`} **o** `lab_hollow: true`. `lab_hollow` si en el mismo run hay lab-skip de cierre (`SDDIA_LAB_SKIP_PBI_ARCHIVE` y/o `SDDIA_LAB_SKIP_DELIVERY_CLOSE`) **o** sello explícito. No hueco: `completed` sin skips de cierre; `failed` real (`exit_code: 1`); fase `skipped` aislada dentro de `completed`. |
| **L-BATCH-SKIP** | En `process_telemetry_file_inner`, **antes** de `samples.push`: si hueco → `mark_consumed` + `stamp_fractal_delivery_state(..., "skipped")` + return `ok: true, skipped: survival_hollow`. **No** mutar `recovery_attempts` / `success_rate` / `status`. |
| **L-FILTER-SCOPE** | Filtro A3 en peaje compartido: aplica a `LIFECYCLE_PROCESSES` (`feature`, `bug-fix`, `refactorization`). No reabre rehab `bug-fix`. |
| **L-THRESH** | Umbrales 1.1.0 intactos. Prohibido tocar `directories.agents` → `radamanto.thresholds.json` / `radamanto.instructions.json`. |
| **L-GENOME** | Motor = parche `SddIA/engine/execute-process/` (no `{name}.md` de process salvo nota mínima vía `entity-manager` si Argos lo exige). A1 = instancia. |
| **L-DOC** | Cascada patrón + PBI a `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` en la misma rama. |

## 4. Touchpoints

| Locus (Cúmulo / repo) | Mutación |
|-----------------------|----------|
| `SddIA/engine/execute-process/.../phase_capsules.rs` | `capsule_feature_invoke_delivery_close`: `invoke_process_full`; merge state; `Ok`+`fail_soft` vs `Err` causal. Tests unitarios del predicado. |
| `SddIA/engine/execute-process/.../delivery_close.rs` | Incluir `delivery_push` en `data` del envelope. **No** cambiar `mark_fail_soft_if_secondary` salvo extracción de helper compartido. |
| `SddIA/engine/execute-process/.../residual_runner.rs` | Rama `feature`\|`bug-fix`: si `Ok` ya trae `fail_soft`, preservar. `Err` permanece causal (sin fail_soft implícito). |
| `SddIA/engine/execute-process/.../thermodynamic.rs` | Copiar `cycle_phase` (+ `lab_hollow`) al payload REF. PEC intacto en emisión. |
| `SddIA/engine/execute-process/.../radamanto_batch_core.rs` | Gate hueco pre-push samples. |
| `SddIA/engine/execute-process/.../phase_terminal.rs` | **Prohibido mutar.** |
| `directories.agents` → `radamanto.thresholds.json` | **Prohibido mutar.** |
| `radamanto.revoked_entities` / `radamanto.stats` (instancia) | A1 solo; evidencia `execution.md`. |
| `directories.evolution` | Entrada breve UUID ciclo `c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f`. |
| `persist_ref` | `implementation.md` / `execution.md` / `validacion.md` + archive PBI. |

Prohibido mutar YAML de `feature.md` / `bug-fix.md` / `delivery-close-cycle.md` salvo nota documental mínima vía cadena autorizada.

## 5. Contratos de comportamiento

### 5.1 Umbral físico (hijo DCC)

```text
physical = non_empty(data.pr_url) OR data.delivery_push is present
```

`delivery_push` debe existir en `data` post-L-DCC-DATA. Fallback de inspección: fase de publicación remota `executed` en `execution_report.phases` **solo** si `data.delivery_push` aún no está (defensa; no es el camino feliz).

### 5.2 Fail-soft padre (A2)

```text
envelope = invoke_process_full(..., "delivery-close-cycle", ...)
merge padre.state ← envelope.data {pr_url, delivery_push, snapshot_commit_hash, event_id, ...}

if envelope.success:
  return Ok(executed, delivery_close=data)

if physical AND secondary_failure(envelope):
  return Ok(status=failed, fail_soft=true, handler=feature-delivery-close, delivery_close=data)

return Err(causal)  # residual → failed sin fail_soft → agregador colapsa exit_code
```

`secondary_failure`:

- algún `execution_report.phases[]` con `fail_soft: true`, **o**
- `data.thermodynamic_toll.telemetry_io_failed == true`, **o**
- error/fase en cola higiene/impacto / timeout / telemetry / receipt (misma semántica DCC ola 2),
- **y** ninguna fase causal del hijo en `failed`/`blocked` **sin** `fail_soft` (snapshot, push, apertura PR, Argos block).

Agregador padre: `failed`+`fail_soft` → `success` global / `exit_code` 0.

### 5.3 Causales (siguen abortando `feature`)

- Fase 1 `workspace-init` (git/rama).
- Snapshot / push / apertura PR del DCC hijo (sin umbral físico alcanzado).
- Argos `block` / `argos_verdict: block`.
- Fases agente 2–5 con runtime vivo y agente **falla**. `simulated` ya es neutral; **no** convertirlo en `fail_soft`.
- Fase 6 `feature-pbi-archive` en runtime. Skip lab permanece `skipped`.

### 5.4 Poda supervivencia (A3)

```text
cycle_phase = derive_cycle_phase(process, phase_reports)  # éxito
            | "failed"                                   # !success (lifecycle)
lab_hollow  = env_lab_skip_close OR payload.lab_hollow
hollow      = cycle_phase ∈ {initialized, awaiting_agents} OR lab_hollow

REF.payload += { cycle_phase?, lab_hollow? }
PEC.payload.cycle_phase  # sin cambio de emisión

batch:
  if hollow: consumed + delivery_state skipped; no samples.push; no degrade/restore
  else: camino actual
```

`lab_hollow` se sella en el emisor (thermodynamic), no solo en el batch, para que el evento sea auditable.

## 6. Criterios técnicos (mapa AC)

| AC | Verificación |
|----|--------------|
| **AC-A1** | `feature` ∉ `permanent` ni `revoked`; stats raíz `healthy`; `recovery_attempts: 0`; `rehab_laudo`; `rehabilitated_at`; ventana recortada; evidencia en `execution.md` (paths instancia **no** en diff). |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | Hijo post-`pr_url`/`delivery_push` + cola secundaria → padre `fail_soft` + éxito operativo; git/snapshot/push/PR/Argos/agentes reales causales; agregador sin diff funcional. |
| **AC-A3** | REF porta `cycle_phase` (y `lab_hollow` si aplica); batch no empuja huecos ni muta contadores; PEC se emite. |
| **AC-THRESH** | JSON umbrales 1.1.0 bit-idéntico en el PR (sin tocar). |
| **AC-DOC** | Cascada; PBI en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## 7. Pruebas mínimas (producto — qué, no cómo)

| Caso | Aserción |
|------|----------|
| Hijo envelope `success:false`, `data.pr_url` set, fase higiene `failed`+`fail_soft` | Padre fase 7 `fail_soft: true`; `aggregate_execution_terminal` → `success: true` |
| Hijo `success:false`, sin `pr_url` ni `delivery_push`, snapshot failed | Padre `exit_code: 1`; sin `fail_soft` en fase 7 |
| Hijo `telemetry_io_failed` post-push | Padre `fail_soft`; no decapita |
| REF lifecycle `simulated` → `cycle_phase: initialized` | Batch `skipped: survival_hollow`; `samples` sin el asset |
| REF `lab_hollow: true` con `cycle_phase: completed` (lab-skip cierre) | Idem: no entra a `samples` |
| REF `exit_code: 1`, `cycle_phase: failed` | **Sí** entra a `samples` (el fuego cuenta) |
| PEC emitido en run hueco | Evento orquestación presente; batch no ingiere supervivencia |
| Umbrales fixture | `version == 1.1.0`; `process == 0.70`; `max_recovery_attempts == 3` |

Tests viven en crate `execute-process` (`--lib`). Preferir helpers puros sobre spawn de orquestador anidado cuando el predicado lo permita.

## 8. Fuera de alcance

- Residual Kalma2 Shell / `git-manager` (dedup #136).
- Lab IMAP/Telegram vivo.
- Rehab `bug-fix` / `emit-pr-audited-event` (Cerbero/stats).
- Reabrir umbrales / `success_rate` / `max_recovery_attempts`.
- Mutar `aggregate_execution_terminal` para «tolerar simulated».
- Troceo EDA de `feature` (faro Kaizen; Filtro C).
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR.

## 9. Handoff Tekton

Ejecutar `plan.md` T0→T5. Git solo `skill:git-manager`. No declarar A1 APTO sin evidencia de instancia en `execution.md`. No tocar umbrales. No inventar `pr_url` de hijo para forzar fail_soft.
