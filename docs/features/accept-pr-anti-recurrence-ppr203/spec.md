---
feature_name: accept-pr-anti-recurrence-ppr203
created: "2026-08-27"
process: refactorization
phase: design
agents: dedalo
base: main
scope: failsoft-sync-post-merge-accept-pr
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-anti-recurrence-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
version_spec: "1.0.0"
status: dedalo_locked
ola: A2
olas:
  - A2
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — sync/push causal post-merge_commit_hash; since 2026-08-27T12:31:30Z"
---

# Spec — ola A2 accept-pr-anti-recurrence-ppr203

## 1. Misión técnica

Extender supervivencia post-física de `accept-pr` a `"Sincronización y Limpieza"` cuando `merge_commit_hash` ya cruzó. No reabrir sello #200. Instancia = persist_ref A1.

## 2. Diagnóstico código

| Vector | Hecho |
|--------|--------|
| Sello | `accept_pr.rs` `SEAL_PHASE` + `residual_runner` inline/post-pass **cubierto**. |
| Sync | `execute_accept_pr_phase` `"Sincronización y Limpieza"`: `push` origin/main **Err → return Err**. Higiene delete solo tras push OK. |
| Residual | `mark_fail_soft_if_seal_post_merge` **ignora** sync (`phase_name != SEAL_PHASE`). |
| Agregador | failed sin `fail_soft` → `exit_code: 1` → Radamanto sample KO → n=3 rate < 0.70 → Cerbero. |
| Empiria | 4s post-merge #203; 1 KO en ventana de 3. |

## 3. Laudos Dedalo (A2)

| Ref | Decisión |
|-----|----------|
| **L-SYNC-PHASE** | Constante de fase = `"Sincronización y Limpieza"` (nombre YAML SSOT). |
| **L-FAILSOFT-SYNC** | Predicado: `physical` ∧ fase sync ∧ status failed/blocked → `report.fail_soft = true`. |
| **L-HELPERS** | `mark_fail_soft_if_sync_post_merge` + `adjudicate_sync_fail_soft_post_merge` **o** helper unificado `mark_fail_soft_if_post_physical(phase, …)` parametrizado (sello ∪ sync) **sin** cambiar semántica sello. Preferir helpers dedicados para no mezclar tests. |
| **L-INLINE-ERR** | En `residual_runner` rama Err accept-pr: llamar mark sync **además** del mark sello (no-op si fase distinta). |
| **L-RESIDUAL-SYM** | Post-bucle: adjudicar sync **y** sello (orden irrelevante; idempotente). |
| **L-SIGNAL** | Report sigue `failed` + `error` (push). Survival vía `fail_soft`. |
| **L-AGGREGATOR** | `phase_terminal.rs` prohibido. |
| **L-NO-HOLLOW** | `radamanto_batch_core.rs` prohibido. |
| **L-THRESH** | `radamanto.thresholds.json` prohibido. |
| **L-PROCESS-YAML** | `accept-pr.md` prohibido. |
| **L-NO-REOPEN-SEAL** | No mutar predicado sello salvo extract compartido de `physical`. |
| **L-NO-REOPEN-194** | `delete_branch_*` / higiene intactos. |
| **L-TESTS** | `t_a2_sync_*` nuevos. `t_a2_seal_*` / `t_a2_canon_*` sin cambio de aserción. |

## 4. Touchpoints

| Locus | Mutación |
|-------|----------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | Helpers sync + tests `t_a2_sync_*`. |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Err inline + post-pass sync. |
| `phase_terminal.rs` / `radamanto_batch_core.rs` / thresholds / `accept-pr.md` | **Prohibido.** |
| `directories.evolution` | UUID `b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e`. |
| persist_ref A2 | Cascada + `validacion.md` + archive PBI. |

## 5. Contratos

```text
physical = non_empty(trim(state.merge_commit_hash))

# INLINE Err
entry.status = failed; entry.error = <push/git>
mark_fail_soft_if_seal_post_merge(...)   # no-op si no sello
mark_fail_soft_if_sync_post_merge(...)   # no-op si no sync
# physical ∧ sync → fail_soft

# POST-PASS
adjudicate_seal_fail_soft_post_merge(...)
adjudicate_sync_fail_soft_post_merge(...)

verdict = aggregate_execution_terminal(...)
# failed + fail_soft → success / exit 0
```

Causal: sync failed ∧ !physical → exit 1. Merge/checkout failed → exit 1.

## 6. Pruebas mínimas

| ID | Caso | Aserción |
|----|------|----------|
| **T-A2-SYNC-SOFT** | hash + sync failed | `fail_soft`; agregador success / 0 |
| **T-A2-SYNC-HARD** | sin hash + sync failed | sin fail_soft; exit 1 |
| **T-A2-SYNC-IDEM** | adjudicate ×2 | un fail_soft |
| Regresión sello | `t_a2_seal_*` | intacta |
| Regresión higiene | `t_a2_canon_*` | intacta |

`cargo test -p execute-process --lib` filtro `t_a2_`.

## 7. Mapa AC

| AC | Verificación |
|----|--------------|
| AC-A2-SYNC | §5 + tests. |
| AC-A2-SEAL | Tests sello. |
| AC-TESTS | `t_a2_` verde. |
| AC-THRESH | Diff umbrales vacío. |
| AC-GIT-CLEAN | Sin instancia. |
| AC-DOC | Patrón + PBI done + validacion APTO. |

## 8. RBAC

`ecosystem-evolution` / `filesystem-ops` (engine) · `source-control` (git) · `action:execute-process` DCC.

## 9. Handoff Tekton

`plan.md` T0→T5. A2 motor **antes** de A1 instancia en host. No declarar Done sin ambos persist_ref en el mismo PR.
