---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
process: refactorization
phase: execution
agents: tekton
items:
  - T0-failsoft-seal
  - T0-residual-sym
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
olas:
  - A1
  - A2
runtime_correlation_id: "7756d929-8baa-40a3-962c-15be0e5c0541"
---

# Implementation — accept-pr-revoked-registry-rehab-ppr200

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `accept_pr_physical_threshold_crossed`; `mark_fail_soft_if_seal_post_merge`; `adjudicate_seal_fail_soft_post_merge`; tests `t_a2_seal_*` |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Err sello → fail_soft si hash; post-bucle `adjudicate_seal_fail_soft_post_merge` |
| `SddIA/engine/execute-process/src/engine/phase_terminal.rs` | **Intacto** |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | **Intacto** |
| `SddIA/agents/radamanto.thresholds.json` | **Intacto** (1.1.0) |
| `.SddIA/cerbero/revoked_entities.json` | A1: `accept-pr` ausente (no PR) |
| `.SddIA/radamanto/stats.json` | A1: bucket raíz healthy + laudo #200 (no PR) |
| `SddIA/evolution/a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c.md` | Registro UUID ciclo |

## Tests (T0)

`cargo test -p execute-process --lib t_a2_` — **7/7 OK** @ 2026-08-27 (`t_a2_seal_*` 3/3).

## Predicado A2 (L-FAILSOFT-SEAL)

```text
physical = non_empty(merge_commit_hash)
∧ phase_name == "Sello Criptográfico de Fusión"
∧ status ∈ {failed, blocked}
→ report.fail_soft = true
```

## Fuera de esta entrega

- Rehab laterales Cerbero (`refactorization`, `emit-pr-audited-event`).
- Mutación de instancia en el diff git.
- T5 DCC (apertura PR) — fase posterior.
