---
feature_name: accept-pr-anti-recurrence-ppr203
created: "2026-08-27"
purpose: Estabilización Mayeuta — ola A2 PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY (anti-recurrencia motor accept-pr post-#200)
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-anti-recurrence-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
pbi_alias: docs/todos/pending/PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY.md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — re-revoked 4s post-merge 120d741… despite A2 seal fail_soft #200"
ola: A2
olas:
  - A2
runtime_execution_id: "d96ec0f4-99ef-44d3-9d93-6fa9009a72cc"
---

# Clarificación — ola A2 accept-pr-anti-recurrence-ppr203

Transcript Mayeuta. **Qué / por qué** del gap motor. Instancia = ola A1.

## D0 — Semilla

| Vector | Hecho |
|--------|--------|
| #200 A2 | `mark_fail_soft_if_seal_post_merge` + adjudicación residual **ya en main**. Tests `t_a2_seal_*`. |
| Empiria #203 | Merge `120d741…` @ `12:31:26Z`. Revoke @ `12:31:30Z`. Sample KO `5d6f7cb3-…` exit **1** · 766ms. Ventana n=3 (OK/KO/OK) rate **0,667** < 0,70. |
| Smoke #202 | `execution.md` ppr202 declara accept-pr `exit_code: 0` + Merged `4afbf976-…`. El KO **no** es el sello si ese run fue 0. |

Dictamen causal (Filtro A):

1. Umbral 1.1.0: **un** exit 1 en n=3 revoca. No tocar umbrales.
2. Fail_soft **solo** cubre fase `"Sello Criptográfico de Fusión"`. `"Sincronización y Limpieza"` (`push` origin/main) es **Err causal** aunque `merge_commit_hash` ya cruzó — hueco simétrico a DCC `mark_fail_soft_if_secondary` post-`pr_url`.
3. Higiene `delete_branch` ya es soft (#194); **push no**.
4. Telemetría Radamanto usa `exit_code` del veredicto. Si agregador = 1, el sample es KO. A2 debe hacer survival post-física = `exit_code: 0` **o** el sample no entra a `success_rate` (hollow). Preferir fail_soft + agregador 0 (agregador intacto).

## D1 — Misión

Cortar re-muerte: fail_soft **post-umbral físico** para fases no-sello que no anulan la fusión (push/sync). Conservar sello #200. Causal duro si **no** hay `merge_commit_hash`.

## D2 — Laudos Mayeuta (A2)

| ID | Decisión |
|----|----------|
| **L-NO-REOPEN-SEAL** | No reescribir predicado sello #200. Tests `t_a2_seal_*` / `t_a2_canon_*` intactos. |
| **L-POSTPHYS-SYNC** | `physical = non_empty(merge_commit_hash)` ∧ fase `"Sincronización y Limpieza"` ∧ `status ∈ {failed, blocked}` → `fail_soft: true` **antes** del agregador. Error de push **visible**. |
| **L-PHYSICAL** | Sin hash → push/sync KO permanece `exit_code: 1`. Merge soberano KO permanece causal. |
| **L-INLINE-ERR** | Rama Err `execute_accept_pr_phase` para sync: marcar soft si physical (simetría sello). |
| **L-FAILSOFT-RETRO** | Extender adjudicación post-bucle a sync (helper dedicado o ampliar `adjudicate_*` **sin** romper sello). Idempotente. |
| **L-AGGREGATOR** | `phase_terminal.rs` intacto. |
| **L-THRESH** | Umbrales 1.1.0 bit-idénticos. |
| **L-HOLLOW** | `radamanto_batch_core` / `is_survival_hollow` **prohibido** salvo evidencia de que fail_soft success aún graba exit 1 (entonces Dedalo reabre con laudo). Hipótesis primaria = agregador 1 por push causal. |
| **L-PROCESS-YAML** | `accept-pr.md` intacto. |
| **L-NO-REOPEN-194** | Payload `delete_branch` / handoff #194 intactos. |
| **L-DOC** | Cascada este persist_ref + Argos + archive PBI canónico + DCC (cierre del ciclo). |

## D3 — Criterios (A2)

| ID | Criterio |
|----|----------|
| AC-A2-SYNC | Sync/push KO + hash → `fail_soft` + agregador success / exit 0; sin hash → causal. |
| AC-A2-SEAL | Regresión sello #200 verde. |
| AC-TESTS | Producto A2-sync + no regresión sello/higiene. |
| AC-THRESH | Umbrales intactos. |
| AC-DOC | `validacion.md` APTO · `pbi_archived: true` · PBI canónico en `done/` · branch coherente. |
