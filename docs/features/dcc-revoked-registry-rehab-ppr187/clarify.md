---
feature_name: dcc-revoked-registry-rehab-ppr187
created: "2026-08-21"
purpose: Estabilización Mayeuta — PBI-PPR-187-DCC-REVOKED-REGISTRY (rehab DCC + adjudicación retroactiva EDA post-umbral físico)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
pbi_ref: docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
source_correlation_id: 4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32
source_pr_url: https://github.com/racso80es/SddIA/pull/187
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
incident_ref: "REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE — abrupt_success_rate_drop since 2026-08-20T12:04:10Z"
olas:
  - A1
  - A2
---

# Clarificación — dcc-revoked-registry-rehab-ppr187

Transcript Mayeuta. Estabiliza el **qué** y el **por qué**. Sin diseño de cápsulas, YAML de proceso ni mutación de genoma.

## D0 — Semilla y evidencia

| Vector | Hecho |
|--------|--------|
| PBI canónico | `docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md` (`document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY`; `uuid: c4a91e7b-…`; `status: abierto`; `refinement_status: dedalo_ready`) |
| Ciclo | `refactorization` · rama `refactor/dcc-revoked-registry-rehab-ppr187` · un `persist_ref` · un PR |
| Semilla operador | Rehabilitar `delivery-close-cycle` en Cerbero/Radamanto (PPR #187) y blindar anti-recurrencia `abrupt_success_rate_drop`: **A1** Yunque Rúnico + **A2** adjudicación retroactiva EDA post-umbral físico |
| Padre | #174+#177 (`persist_ref`: `docs/features/radamanto-process-threshold-rehab`) — umbral `process: 0.70` + fail-soft handoff DCC; **no** cubre block EDA fase 3 pre-`pr_url` |
| Hermano | #185 fail-soft **padre** `feature`→DCC hijo + poda hollow; **no** rehabilitó DCC ni cortó re-muerte del peaje DCC como proceso raíz |
| Check origen | `REVOKED_ENTITY_ALERT_DELIVERY_CLOSE_CYCLE` (F4/F5 · alerta no bloqueante) |
| Sighting | PPR #187 · CID `4gKBTRCyZzvEFQcbDWFnBmdC3ZjvqTJmauHiYgTWwj32` · emisor ECST `github-bridge-watcher` ∉ revoked · clave DCC revocada @ `2026-08-20T12:04:10Z` |
| Dedup gemelo | CID `34736c88-34d3-46f8-a050-75e7775d005b` — misma seed; **no** dedup contra #177 done (`since` distinto) |
| `correlation_id` runtime de esta fase | vacío en inputs |

### Estado empírico (corte estabilización 2026-08-21 · verificado en instancia)

| Clave | Cerbero | Radamanto | Nota |
|-------|---------|-----------|------|
| `delivery-close-cycle` (raíz) | **`revoked.delivery-close-cycle`** · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-20T12:04:10Z` | `status: degraded` · `recovery_attempts: 2` · `degraded_at: 2026-08-20T12:04:10Z` · `rehab_laudo: PBI-PPR-174-177-…` · `rehabilitated_at: 2026-08-16T16:37:15Z` · `structure_valid: false` · 5 samples (3 OK / 2 KO · rate 0,60) | **Vector activo.** Ausente de `permanent` |
| `entities.delivery-close-cycle` | — | `healthy` · samples lab ~75–126 ms | Fósil ontología; **no** es el revocado |
| Laterales | `revoked.bug-fix`, `revoked.refactorization`, `revoked.emit-pr-audited-event` | fuera de alcance | Prohibido rehabilitar este ciclo |

Dictamen PBI (vinculante): vector ≠ `success_rate_below_threshold` (#177). Con n=5 ≥ `abrupt_drop_min_samples: 3` y rate 0,60 < 0,70 aplica **`abrupt_success_rate_drop`**. Cadena causal: DCC cierre PR #187 → Aduana EDA `orphan_count=2` preexistentes → `exit_code: 1` → batch Radamanto degrada.

## D1 — Misión (qué / por qué)

| Decisión | Laudo |
|----------|--------|
| Objetivo | Rehabilitar `delivery-close-cycle` en Cerbero/Radamanto **y** impedir re-muerte por `abrupt_success_rate_drop` cuando el umbral físico (push/`pr_url`) ya cruzó pese a block EDA por huérfanos **preexistentes**. |
| Por qué ahora | Rehab de registro **sin** A1 absoluto + A2 reabre el vector (jurisprudencia #185 / #174+#177). Fail-soft ola 2 exige umbral físico **en el momento** de marcar; EDA es fase 3 y aún no lo tiene. |
| Efecto observable | DCC ∉ `revoked` ni `permanent`; stats raíz `healthy` con `recovery_attempts: 0` y ventana podada; report EDA puede seguir `blocked`+`argos_verdict: block` pero con `fail_soft: true` retroactivo → agregador `success` / `exit_code: 0` cuando push/PR cruzaron. |

## D2 — Decisiones de estabilización (laudos Mayeuta)

| ID | Decisión |
|----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un `persist_ref`, un PR. Prohibido despachar `bug-fix` satélite. |
| **L-WAVES** | Dos olas innegociables en el mismo ciclo: **A1** saneamiento instancia (Yunque Rúnico), **A2** adjudicación retroactiva EDA post-umbral físico. Rehab Cerbero sola = reabrir vector. |
| **L-REHAB-INST** | A1 = instancia `.SddIA/` (no genoma). Evidencia en `execution.md`. Prohibido commitear `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.delivery-close-cycle` por completo. Verificar `permanent.delivery-close-cycle` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** del bucket **raíz** `delivery-close-cycle`. No mutar fósil `entities.delivery-close-cycle`. |
| **L-RESET-ABS** | Absoluto: `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-187-DCC-REVOKED-REGISTRY`; `rehabilitated_at` ISO de intervención A1. |
| **L-SAMPLES** | Poda termodinámica obligatoria: vaciar `samples` **o** conservar solo los ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO `d7310496…` y `19391b9f…`. Sin poda, un fallo futuro re-dispara `abrupt_success_rate_drop` (p. ej. rate 1/4 < 0,70 con n≥3). |
| **L-ONTOLOGY** | Conservar `entity_type: process`. No regresionar a `tool`. |
| **L-FAILSOFT-RETRO** | Extensión de **L-FAILSOFT-OLA2**: tras el bucle de fases y **antes** de `aggregate_execution_terminal`, si `(pr_url \|\| delivery_push)` presentes **y** report `"Aduana EDA genómica"` está `blocked`/`failed` con `orphan_count > 0` y `argos_verdict == "block"`, inyectar `fail_soft: true` en ese report (adjudicación **retroactiva**). |
| **L-EDA-SIGNAL** | Prohibido debilitar `capsule_eda_genomic_audit_gate` a `pass` silencioso: Argos debe seguir registrando block / ruido de sistema. |
| **L-AGGREGATOR** | `aggregate_execution_terminal` **intacto** (jurisprudencia #185). El `fail_soft` se escribe en el report **antes** del agregador. |
| **L-CAUSAL** | Sin `fail_soft`: fallo snapshot, push, apertura PR, o block Argos **después** de umbral con deuda **introducida** por el diff del ciclo actual. EDA blocked **sin** umbral físico permanece causal (`exit_code: 1`). |
| **L-NO-HOLLOW** | **L-HOLLOW** / **L-BATCH-SKIP** (#185 A3) **no aplican** a DCC (`delivery-close-cycle` ∉ `LIFECYCLE_PROCESSES`). Prohibido tococar poda hollow / `residual_runner` bajo pretexto A3. Path EDA en `residual_runner` solo si Dedalo exige simetría de la adjudicación retroactiva (helper compartido), no poda survival. |
| **L-THRESH** | `radamanto.thresholds.json` v1.1.0 **intacto**. No reabrir `success_rate_min` ni `abrupt_drop_min_samples` sin laudo explícito. |
| **L-YAML** | No mutar YAML `{name}.md` del proceso para “inyectar” `fail_soft` estático: `fail_soft` es runtime en el JSON del `phase_report` (motor). |
| **L-OUT** | Fuera: rehab `bug-fix` / `refactorization` / `emit-pr-audited-event` / `feature`; backfill EDA de huérfanos preexistentes; merge/handoff `accept-pr` de PR #187 (ya MERGED); versionar instancia en el PR. |
| **L-DOC** | Cascada `features-documentation-pattern` + `validacion.md` APTO + `pbi_archived: true` + PBI en `docs/todos/done/` en la rama del PR. |

### Ajustes anti-alucinación (órdenes crudas → laudo)

| Orden cruda | Laudo |
|-------------|-------|
| «inyectar `fail_soft: true` en YAML de fases» | No. Runtime en `phase_report`; lo escribe el motor. |
| «solo ampliar `is_dcc_secondary_phase` con Aduana EDA» | Insuficiente solo: EDA es fase 3; umbral físico aparece en 4–5. Exige **post-pass** de adjudicación retroactiva. |
| «tocar `residual_runner` / hollow para DCC» | No aplica: hollow es de `LIFECYCLE_PROCESSES`; fallo #187 es `exit_code: 1` real. |
| «A3 poda hollow como vector #187» | Fuera de alcance. |
| «mutar agregador para tolerar blocked EDA» | Prohibido. Marcar `fail_soft` antes. |

## D3 — Matriz de aceptación (producto)

| AC | Enunciado |
|----|-----------|
| **AC-A1** | `delivery-close-cycle` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; `rehab_laudo: PBI-PPR-187-DCC-REVOKED-REGISTRY`; `rehabilitated_at`; `samples` podados; evidencia en `execution.md` (no en diff de instancia). |
| **AC-GIT-CLEAN** | `.SddIA/cerbero/` y `.SddIA/radamanto/` **no** aparecen en `git status` / diff del PR. |
| **AC-ONTO** | `entity_type: process` conservado; no regresionar a `tool`. |
| **AC-A2** | EDA blocked + huérfanos preexistentes + umbral físico cruzado → report con `fail_soft: true` retroactivo + agregador `success` / `exit_code: 0`; sin umbral físico → causal; Argos sigue `block`; agregador intacto; umbrales 1.1.0 intactos. |
| **AC-TESTS** | Unit/integración: EDA+`pr_url` → success; EDA sin push/PR → `exit_code: 1`; regresiones higiene/snapshot fail-soft existentes intactas. |
| **AC-RBAC** | En aduana PPR posterior: `RBAC_EMITTER_NOT_REVOKED: APTO` con emisor `delivery-close-cycle`. |
| **AC-DOC** | Cascada bajo `persist_ref`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## D4 — Handoff Dedalo

1. Consumir este transcript + cuerpo de `objectives.md` como `refined_requirements`.
2. `spec.md`: touchpoints A2 en `delivery_close.rs` (adjudicación retroactiva pre-agregador); opcional coherencia `is_dcc_secondary_phase` **solo** si el post-pass ya garantiza umbral; simetría path EDA en `residual_runner` si procede; **prohibido** mutar `phase_terminal.rs`, debilitar gate EDA, tocar `radamanto_batch_core` / hollow, mutar umbrales.
3. `plan.md`: un PR motor (A2) + procedimiento A1 instancia evidenciado en `execution.md` (git-clean instancia).
4. Tests de producto (qué, no cómo): EDA blocked + `pr_url` → success; EDA blocked sin umbral → causal; no regresión tests fail-soft higiene/snapshot.
5. Prohibido reabrir umbrales, rehab laterales, backfill huérfanos EDA, o castrar señal Argos en Aduana EDA.
