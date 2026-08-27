---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
updated: "2026-08-27T12:15:00Z"
process: refactorization
phase: execution
agents: tekton
items_applied:
  - T0-failsoft-seal
  - T0-residual-sym
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
olas:
  - A1
  - A2
runtime_correlation_id: "7756d929-8baa-40a3-962c-15be0e5c0541"
---

# Execution — accept-pr-revoked-registry-rehab-ppr200

## T0 (motor A2)

| Check | Resultado |
|-------|-----------|
| Predicado sello | `merge_commit_hash` nonempty ∧ fase sello failed/blocked → `fail_soft` |
| Sin hash | sello failed permanece causal |
| Residual | Err en `residual_runner` + adjudicación post-bucle |
| Umbrales / agregador / hollow | **intactos** |

Tests: `cargo test -p execute-process --lib t_a2_` → **7/7** @ 2026-08-27 (`t_a2_seal_*` 3/3).

## T1 (instancia · fuera del PR)

Locus Cúmulo: `radamanto.revoked_entities` = `.SddIA/cerbero/revoked_entities.json`; `radamanto.stats` = `.SddIA/radamanto/stats.json`.

| Check | Resultado |
|-------|-----------|
| `revoked.accept-pr` | **ausente** |
| `permanent.accept-pr` | **ausente** |
| laterales | `refactorization`, `emit-pr-audited-event` intactos |
| stats raíz `accept-pr` | `healthy` · `recovery_attempts: 0` · `consecutive_success_count: 0` · `degraded_at: null` · `entity_type: process` · `structure_valid: true` · `rehab_laudo: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY` · `rehabilitated_at: 2026-08-27T12:00:00Z` · `samples: []` |
| fósiles #194 | `rehab_laudo` / `rehabilitated_at` 11:20Z **sustituidos** |

## T2 (documental)

Cascada `objectives`→`execution` + evolution `a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c`. Assert: **no** versionar `.SddIA/cerbero/` ni `.SddIA/radamanto/` en el PR.

## T5 (DCC)

| Campo | Valor |
|-------|--------|
| PR | https://github.com/racso80es/SddIA/pull/202 |
| ECST Presented | `1498e461-3235-483a-b210-907cca744cdd` |
| Merge | `42fff0765f1b0986f1807b89586bbef3f53c0011` @ `2026-08-27T12:11:54Z` |

## T6 (smoke handoff post-merge)

`accept-pr` @ `main` (binario post-#202) · `merge_already_done: true` · `SDDIA_LAB_SKIP_BRANCH_DELETE=1`.

| Check | Resultado |
|-------|-----------|
| `exit_code` | **0** |
| Sello | `PullRequest_Merged` `c3a80d66-…` · fase sello **executed** (no DLQ) |
| `merge_commit_hash` | `42fff0765f1b0986f1807b89586bbef3f53c0011` |
| `orphan_merge` | **false** (`Presented` `1498e461…` encontrado) |
| Cerbero post-smoke | `accept-pr` **∉** `revoked`/`permanent` |
| Radamanto post-smoke | `healthy` · 1×sample OK (`15d2c300…` · 782ms · exit 0) · **sin** re-revocación |

