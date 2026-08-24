---
feature_name: dcc-revoked-registry-rehab-ppr187
created: "2026-08-21"
process: refactorization
phase: execution
agents: tekton
items_applied:
  - T0-adjudicate-eda-fail-soft-post-physical
  - T0-residual-sym
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/dcc-revoked-registry-rehab-ppr187
persist_ref: docs/features/dcc-revoked-registry-rehab-ppr187
pbi_ref: docs/todos/pending/[ARQUITECTURA] delivery-close-cycle — rehabilitación revoked_entities (PPR #187).md
document_id: PBI-PPR-187-DCC-REVOKED-REGISTRY
uuid: c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14
olas:
  - A1
  - A2
---

# Implementation — dcc-revoked-registry-rehab-ppr187

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | `adjudicate_eda_fail_soft_post_physical` (`pub(crate)`); invocado en `run` tras bucle / antes de `aggregate_execution_terminal`. Tests §7. **No** ampliar `is_dcc_secondary_phase`. |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Tras bucle, si `process_name == "delivery-close-cycle"`, mismo helper (**L-RESIDUAL-SYM**). |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | **Intacto** (gate EDA no debilitado). |
| `SddIA/engine/execute-process/src/engine/phase_terminal.rs` | **Intacto.** |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | **Intacto** (hollow fuera). |
| `SddIA/agents/radamanto.thresholds.json` | **Intacto** (1.1.0). |
| `.SddIA/cerbero/revoked_entities.json` | A1: borrado `revoked.delivery-close-cycle` (no PR). |
| `.SddIA/radamanto/stats.json` | A1: bucket raíz DCC healthy + laudo + poda KO (no PR). |
| `SddIA/evolution/c4a91e7b-2f68-4d3a-a8e1-5b7c9d0e2f14.md` | Registro UUID ciclo + PBI-187. |

YAML `delivery-close-cycle.md` intacto.

## Predicado A2 (L-PRED-EDA)

```text
physical = non_empty(pr_url) ∨ delivery_push present
∧ phase_name == "Aduana EDA genómica"
∧ status ∈ {blocked, failed}
∧ orphan_count > 0
∧ argos_verdict == "block"
→ report.fail_soft = true  (retroactivo; Argos block preservado)
```

## Tests (T0)

`cargo test -p execute-process delivery_close` — **14/14 OK** @ 2026-08-24.

## Fuera de esta entrega

- Rehab Cerbero laterales (`bug-fix`, `refactorization`, `emit-pr-audited-event`).
- Backfill EDA huérfanos preexistentes.
- Mutación de instancia en el diff git.
- T5 DCC (apertura PR) — fase posterior en rama.
