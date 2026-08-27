---
feature_name: accept-pr-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
phase: execution
agents: tekton
items:
  - T0-delete-branch-hygiene
  - T0-handoff-status
  - T0-frozen-1.1.0
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
olas:
  - A1
  - A2
  - A3
satellite_fix_pbi: docs/todos/done/[FIX] accept-pr delete_branch payload vs git-manager.md
---

# Implementation — accept-pr-revoked-registry-rehab-ppr194

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `delete_branch_local_payload` / `delete_branch_remote_payload` (bool+force); dos invokes; push causal antes de delete; `hygiene_failure.operations[]` + flags local/remoto; tests T-A2-* |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | `accept_pr_handoff_status` ∈ pending/consumed/blocked/skipped; `true` solo `consumed`; invoke Err → `blocked` sin inventar merge; `f5_handoff_when_merge_absent`; tests T-A3-* |
| `SddIA/norms/skill-io-git-manager-frozen.md` | SemVer **1.1.0**: `delete_branch` / `merge` / `get_last_commit` / `diff_name_only`; homónimo `remote` |
| `SddIA/skills/git-manager.md` | Enum inputs alineado frozen 1.1.0 |
| `SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md` | Outputs handoff boolean + status + `block_reason` |
| `.SddIA/cerbero/revoked_entities.json` | A1: `accept-pr` ausente (no PR) |
| `.SddIA/radamanto/stats.json` | A1: bucket raíz healthy + laudo (no PR) |
| `SddIA/evolution/7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d.md` | Registro UUID ciclo |

## Tests

`cargo test -p execute-process --lib t_a2_` → **4/4**. `t_a3_` → **3/3**. `handoff_skips` → **1/1**. @ 2026-08-27.

## Genoma

Mutación frozen/skill/process YAML materializada en rama (Kalma2 join + Tekton). `accept-pr.md` **intacto**. `radamanto.thresholds.json` **intacto**.
