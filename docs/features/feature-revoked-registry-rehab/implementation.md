---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
process: refactorization
phase: execution
agents: tekton
items:
  - a2-fail-soft-padre-dcc
  - a3-poda-telemetria-hueca
  - a1-instance-rehab
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
olas:
  - A1
  - A2
  - A3
---

# Implementation — feature-revoked-registry-rehab

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | `delivery_push` en `data` del envelope DCC (L-DCC-DATA). Semántica `mark_fail_soft_if_secondary` intacta. |
| `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | `invoke_process_full`; merge `pr_url`/`delivery_push`/snapshot; `feature_dcc_parent_fail_soft`; `Ok`+`fail_soft` vs `Err` causal; veto fases causales hijas; fallback físico `Publicación remota` executed. |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Sin mutación: rama `feature`\|`bug-fix` ya copia `fail_soft` del `Ok`; `Err` permanece causal. |
| `SddIA/engine/execute-process/src/engine/thermodynamic.rs` | REF: `cycle_phase` vía `survival_cycle_phase`; `lab_hollow` solo lab-skip de cierre (env o `reason` en phase_reports). PEC sin castrar. |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | Gate `is_survival_hollow` pre-`samples.push`; `skipped: survival_hollow`. Test umbrales 1.1.0 lectura. |
| `SddIA/engine/execute-process/src/engine/phase_terminal.rs` | **Intacto.** |
| `SddIA/agents/radamanto.thresholds.json` | **Intacto** (1.1.0 / process 0.70 / max_recovery_attempts 3). |
| `.SddIA/cerbero/revoked_entities.json` | A1: `permanent.feature` ausente (no PR). |
| `.SddIA/radamanto/stats.json` | A1: raíz `feature` healthy + laudo (no PR). |
| `SddIA/evolution/c041bfd2-3be0-4956-83ec-be28fadee390.md` | Registro UUID ciclo + PBI-185. |

YAML `feature.md` / `bug-fix.md` / `delivery-close-cycle.md` intacto.

## Predicado padre (A2)

```text
!success ∧ physical(pr_url|delivery_push|fase Publicación remota executed)
∧ ¬causal_hard_fail(hijo)
∧ (telemetry_io_failed ∨ phase.fail_soft ∨ error cola higiene/impacto/timeout/telemetry/receipt)
→ Ok(status=failed, fail_soft=true, handler=feature-delivery-close)
```

## Fuera de esta entrega

- Rehab Cerbero/stats `bug-fix` / `emit-pr-audited-event`.
- Troceo EDA de `feature`.
- Mutación de instancia en el diff git.
- T4 Argos / T5 DCC (fases posteriores).
