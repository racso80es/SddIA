---
feature_name: accept-pr-anti-recurrence-ppr203
created: "2026-08-27"
process: refactorization
phase: execution
agents: tekton
items:
  - T0-failsoft-sync
  - T0-residual-sym
  - T0-unit-tests
  - T2-evolution
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-anti-recurrence-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
ola: A2
olas:
  - A2
---

# Implementation — ola A2 accept-pr-anti-recurrence-ppr203

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | `SYNC_PHASE`; `mark_fail_soft_if_sync_post_merge`; `adjudicate_sync_fail_soft_post_merge`; tests `t_a2_sync_*` |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Err accept-pr → mark sync + sello; post-bucle adjudicación sync |
| `SddIA/engine/execute-process/src/engine/phase_terminal.rs` | **Intacto** |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | **Intacto** |
| `SddIA/agents/radamanto.thresholds.json` | **Intacto** (1.1.0) |
| `SddIA/evolution/b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e.md` | Registro UUID ciclo |

## Tests (T0)

`cargo test -p execute-process --lib t_a2_` — **10/10 OK** @ 2026-08-27 (`t_a2_sync_*` 3/3; `t_a2_seal_*` / `t_a2_canon_*` sin regresión).

## Predicado A2-sync (L-FAILSOFT-SYNC)

```text
physical = non_empty(merge_commit_hash)
∧ phase_name == "Sincronización y Limpieza"
∧ status ∈ {failed, blocked}
→ report.fail_soft = true
```

## Fuera de esta entrega

- Instancia Cerbero/Radamanto (ola A1 · persist_ref hermano).
- T5 DCC (apertura PR) — fase posterior.
