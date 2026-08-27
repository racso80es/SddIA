---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-accept-pr-cerbero + failsoft-seal-post-merge
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
version_spec: "1.0.0"
status: dedalo_locked
olas:
  - A1
  - A2
source_correlation_id: "7c215675-2ad2-436a-9749-ff635c52c8b3"
source_pr_url: https://github.com/racso80es/SddIA/pull/200
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — abrupt_success_rate_drop since 2026-08-27T11:31:15Z"
---

# Spec — accept-pr-revoked-registry-rehab-ppr200

## 1. Misión técnica

Rehabilitar `accept-pr` tras **re-revocación** post-#194 (`abrupt_success_rate_drop` since `2026-08-27T11:31:15Z`, PPR #200) **y** cortar re-muerte: A1 Yunque Rúnico + A2 fail_soft del sello `PullRequest_Merged` cuando `merge_commit_hash` ya cruzó. Jurisprudencia DCC #187 (fail_soft post-umbral físico). Un `persist_ref`, un PR.

Consumir `objectives.md` + `clarify.md` como `refined_requirements` (Mayeuta).

## 2. Diagnóstico (evidencia código + instancia)

| Vector | Hecho |
|--------|--------|
| Cerbero instancia | `revoked.accept-pr` · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-08-27T11:31:15Z`. Ausente de `permanent`. |
| Radamanto bucket raíz | `degraded` · `recovery_attempts: 1` · samples mixtos (rate 0,50 < `process: 0.70`) · n≥`abrupt_drop_min_samples: 3` · `rehab_laudo` fósil #194 @ `11:20:00Z`. |
| Cadena causal | Merge soberano #194 (`6528d115…` @ `11:31:11Z`) **cruzó** → dead-letter `PullRequest_Merged` (`c24d84a7…`) → `exit_code: 1` → Cerbero re-revoca @ `11:31:15Z` (**4s**). |
| A2 #194 | Higiene `delete_branch` + handoff truth ya en `main`; **no** cubre supervivencia del sello EDA post-fusión. |
| Cuello A2 | Fase `"Sello Criptográfico de Fusión"` en `accept_pr.rs` falla vía `Err` (emit nativo KO / DLQ). Sin `fail_soft`, agregador tumba survival pese a `merge_commit_hash` ya en state. |
| Path residual | `residual_runner` arma Err del sello **sin** marcar soft; post-bucle **sin** adjudicación accept-pr (DCC #187 sí tiene post-pass). |
| Umbral físico | Presencia non-empty de `state.merge_commit_hash` (fusión soberana materializada). |
| Agregador | `aggregate_execution_terminal`: `failed`/`blocked` sin `fail_soft` → `exit_code: 1`. **Prohibido mutar.** |
| Umbrales | `directories.agents` → `radamanto.thresholds.json` **1.1.0** intactos. |
| Laterales | `revoked.refactorization`, `revoked.emit-pr-audited-event` — fuera. |
| SSOT proceso | `process_domain_roots` → `accept-pr.md` § sello — **intacto** (fail_soft es runtime, no YAML). |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-UNIFY** | Un ciclo `refactorization`, un PR. Prohibido `bug-fix` satélite. |
| **L-WAVES** | A1 instancia + A2 motor innegociables. Rehab Cerbero sola = reabrir vector (empiria 4s). |
| **L-REHAB-INST** | A1 = instancia Cúmulo (`radamanto.revoked_entities`, `radamanto.stats`). Evidencia en `execution.md`. Prohibido versionar esas rutas en el diff del PR. |
| **L-CERBERO** | Eliminar nodo `revoked.accept-pr`. Assert `permanent.accept-pr` ausente. Cerbero **no** tiene estado `healthy`. |
| **L-STATS** | Reset **solo** bucket raíz `accept-pr`. |
| **L-RESET-ABS** | `status: healthy`; `recovery_attempts: 0`; `consecutive_success_count: 0`; `degraded_at: null`; `rehab_laudo: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY`; `rehabilitated_at` ISO UTC de A1. Limpiar residuales #194. |
| **L-SAMPLES** | Vaciar `samples` **o** ≤3 últimos OK runtime (`exit_code: 0`). Eliminar KO de la ventana (p. ej. `80965a19…` / `6141727e…` si presentes). Sin poda, un fallo futuro reabre `abrupt_success_rate_drop`. |
| **L-ONTOLOGY** | Conservar `entity_type: process`. |
| **L-FAILSOFT-SEAL** | Predicado: `physical = non_empty(merge_commit_hash)` ∧ `phase_name == "Sello Criptográfico de Fusión"` ∧ `status ∈ {failed, blocked}` → `report.fail_soft = true` **antes** de `aggregate_execution_terminal`. |
| **L-PHYSICAL** | Umbral = presencia de `merge_commit_hash`. Sin hash → sello KO permanece causal (`exit_code: 1`). |
| **L-INLINE-ERR** | En `residual_runner`, rama Err de `execute_accept_pr_phase` para la fase sello: invocar `mark_fail_soft_if_seal_post_merge` al construir el report (path empírico DLQ = Err). |
| **L-FAILSOFT-RETRO** | Helper `adjudicate_seal_fail_soft_post_merge(phase_reports, state)` post-bucle / pre-agregador (simetría `adjudicate_eda_fail_soft_post_physical` #187). Idempotente. |
| **L-RESIDUAL-SYM** | Cablear **ambos** puntos (Err inline + post-pass). Cobertura parcial = no cierre Dedalo frente a `c24d84a7…`. |
| **L-SIGNAL** | Report sigue `failed`/`blocked` + error auditable (DLQ visible). Solo survival global vía `fail_soft`. |
| **L-AGGREGATOR** | `phase_terminal.rs` **intacto**. |
| **L-NO-HOLLOW** | Prohibido tocar `radamanto_batch_core` / hollow / `LIFECYCLE_PROCESSES`. |
| **L-THRESH** | Umbrales 1.1.0 bit-idénticos. |
| **L-PROCESS-YAML** | `accept-pr.md` **intacto**. Prohibido `fail_soft` estático en YAML. |
| **L-NO-REOPEN-194** | No reabrir A2 payload / A3 handoff #194 sin regresión empírica nueva. |
| **L-DOC** | Cascada patrón + PBI → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` en la misma rama. |

## 4. Touchpoints

| Locus (Cúmulo / repo) | Mutación |
|-----------------------|----------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `accept_pr_physical_threshold_crossed`; `mark_fail_soft_if_seal_post_merge`; `adjudicate_seal_fail_soft_post_merge`; tests `t_a2_seal_*`. **No** mutar higiene/payload #194. |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Err sello → `mark_fail_soft_if_seal_post_merge`; post-bucle si `process_name == "accept-pr"` → `adjudicate_seal_fail_soft_post_merge`. |
| `phase_terminal.rs` | **Prohibido.** |
| `radamanto_batch_core.rs` | **Prohibido.** |
| `directories.agents` → `radamanto.thresholds.json` | **Prohibido.** |
| `process_domain_roots` → `accept-pr.md` | **Prohibido** (salvo nota mínima vía `entity-manager` si Argos lo exige). |
| `radamanto.revoked_entities` / `radamanto.stats` (instancia) | A1 solo; evidencia `execution.md`; fuera del diff. |
| `directories.evolution` | Entrada UUID ciclo `a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c`. |
| `persist_ref` | Cascada + archive PBI + `validacion.md`. |

## 5. Contratos de comportamiento

### 5.1 Umbral físico

```text
physical = non_empty(trim(state.merge_commit_hash))
```

### 5.2 Fail_soft sello (A2) — cobertura punta a punta

```text
# (1) INLINE — residual_runner Err arm (path empírico DLQ PullRequest_Merged)
entry.status = failed; entry.error = <emit/DLQ>
mark_fail_soft_if_seal_post_merge(&mut entry, phase_name, state)
# si physical ∧ phase == "Sello Criptográfico de Fusión" → fail_soft: true
# bucle residual NO aborta → "Sincronización y Limpieza" sigue

# (2) POST-PASS — tras for phase in phases, antes de aggregate
adjudicate_seal_fail_soft_post_merge(&mut phase_reports, &state)
# idempotente sobre reports sello failed/blocked + physical

verdict = aggregate_execution_terminal(&phase_reports, &state)
# failed + fail_soft → success / exit_code 0 (agregador intacto)
```

Escenario empírico a cubrir: merge hash presente + sello Err/DLQ (`c24d84a7…` @ `11:31:11Z`) → survival; telemetría no degrada por ese sample.

### 5.3 Causal duro (sin fail_soft)

```text
sello failed/blocked + !physical     → exit_code 1
merge/push soberano failed           → exit_code 1 (fases previas)
error de sello siempre visible       → no silenciar DLQ
```

### 5.4 A1 instancia (Yunque)

```text
DELETE revoked["accept-pr"]
ASSERT permanent["accept-pr"] absent
stats["accept-pr"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  rehab_laudo: "PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY",
  rehabilitated_at: <ISO A1>,
  samples: [] | ≤3 últimos OK runtime
}
# laterales revoked intactos; no git-add de .SddIA/cerbero|radamanto
```

## 6. Criterios técnicos (mapa AC)

| AC | Verificación |
|----|--------------|
| **AC-A1** | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; laudo #200 + timestamp; samples podados; evidencia en `execution.md` (instancia **no** en diff). |
| **AC-GIT-CLEAN** | Diff PR sin `.SddIA/cerbero/` ni `.SddIA/radamanto/`. |
| **AC-ONTO** | `entity_type: process` conservado. |
| **AC-A2** | Sello KO + hash → `fail_soft` + agregador success / `exit_code: 0`; sin hash → causal; error visible; agregador sin diff. |
| **AC-TESTS** | Unit §7 verde; regresiones #194 (`t_a2_canon_*` / higiene) intactas si el motor se toca. |
| **AC-THRESH** | Umbrales 1.1.0 bit-idénticos. |
| **AC-DOC** | Cascada; PBI en `done/`; `validacion.md` `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## 7. Pruebas mínimas (producto — qué, no cómo)

| ID | Caso | Aserción |
|----|------|----------|
| **T-A2-SEAL-SOFT** | hash presente + sello `failed` (error DLQ) → mark | `fail_soft: true`; `aggregate_execution_terminal` → `success: true` / `status_code: 0` |
| **T-A2-SEAL-HARD** | sin hash + sello `failed` → mark | sin `fail_soft`; agregador → `exit_code: 1` |
| **T-A2-SEAL-IDEM** | `adjudicate_seal_fail_soft_post_merge` ×2 | un solo `fail_soft`; success |
| Regresión #194 | `t_a2_canon_*` / higiene payload | intacta |

Tests en crate `execute-process` (`--lib`, filtro `t_a2_`). Preferir helpers puros + fixtures JSON; no exigir spawn orquestador para el predicado.

## 8. Fuera de alcance

- Rehab `refactorization` / `emit-pr-audited-event` / `bug-fix`.
- Reabrir A2 payload `delete_branch` / A3 handoff #194 sin evidencia nueva.
- Mutar umbrales / agregador / hollow / YAML `accept-pr.md`.
- Silenciar dead-letter del sello.
- Versionar instancia Cerbero/Radamanto en el PR.
- Escribir semillas/TODOs bajo `docs/todos/` (Cúmulo/Kaizen).

## 9. Viabilidad RBAC (Dedalo)

`target_executor_rbac` del proceso `refactorization`: `ecosystem-evolution`, `filesystem-ops`, `source-control`.

| Delegación | Contexto cápsula | Cruce |
|------------|------------------|-------|
| Motor `execute-process` (Tekton FS) | filesystem-ops / ecosystem-evolution | OK |
| `skill:git-manager` | source-control | OK |
| Instancia A1 (FS local) | filesystem-ops | OK (no genoma) |
| `action:execute-process` → `delivery-close-cycle` | cierre T5 | OK |

Ninguna fase exige política fuera del pack. Genoma `{name}.md` **no** se forja en este ciclo.

## 10. Handoff Tekton

Ejecutar `plan.md` T0→T5. Git solo `skill:git-manager`. No declarar A1 APTO sin evidencia de instancia en `execution.md`. Verificar cobertura **inline Err + post-pass** frente a DLQ empírico. No mutar umbrales ni agregador. No reabrir #194.
