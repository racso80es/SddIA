---
feature_name: ppr-revoked-registry-rehab-ppr190
created: "2026-08-26"
process: refactorization
phase: execution
agents: tekton
items_applied:
  - T0-ppr-cycle-phase
  - T0-detached-child-hollow
  - T0-unit-tests
  - T1-instance-rehab
  - T2-evolution
branch_name: refactor/ppr-revoked-registry-rehab-ppr190
persist_ref: docs/features/ppr-revoked-registry-rehab-ppr190
pbi_ref: docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
document_id: PBI-PPR-190-REVOKED-REGISTRY
uuid: e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21
olas:
  - A1
  - A2
---

# Execution — ppr-revoked-registry-rehab-ppr190

## T0 / T1 (motor)

- `thermodynamic.rs`: PPR en `LIFECYCLE_PROCESSES`; `detached_child` cuando `SDDIA_DETACHED_EXECUTION_ID` activo.
- `radamanto_batch_core.rs`: hollow para `detached_child` KO y `detach: true`.
- Tests: `ppr_detached_child_failure_is_hollow`, `derive_ppr_simulated_is_initialized`, regresiones hollow intactas.

## T1 (instancia · fuera del PR)

| Check | Resultado |
|-------|-----------|
| `permanent.pull-request-review` | **ausente** |
| `revoked.pull-request-review` | **ausente** |
| Laterales (`accept-pr`, `bug-fix`, `refactorization`, `emit-pr-audited-event`) | intactos |
| stats raíz `pull-request-review` | `healthy` · `recovery_attempts: 0` · `rehab_laudo: PBI-PPR-190-REVOKED-REGISTRY` · `rehabilitated_at: 2026-08-26T18:02:03Z` · 3 samples OK (poda 9 KO pre-rehab) |

## T2

Evolution `e2b9a4f1-7c83-4d5e-9a16-0f8b3c5d7e21` + cascada documental.

## Pendiente runtime

T5 DCC (apertura PR) — fase posterior.
