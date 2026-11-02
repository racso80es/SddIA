---

feature_name: aut-62-ppr-qa-gate
created: "2026-11-02"
process: bug-fix
phase: execution
agents: tekton
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-62-ppr-qa-gate
pbi_ref: AUT-62
document_id: PBI-c4293e98
uuid: 69228e55-be56-4578-8bf3-1207b4eb31a2
tasks:
  - T0-baseline
  - T1-instance-rehab
  - T2-evolution
  - T3-ppr-verify
  - T4-gate-pre-pr
execution_id: "9554806d-2d2e-4944-89b7-25e2f81cc27a"
---
# Execution — aut-62-ppr-qa-gate

## Baseline (T0)

**Cerbero** — `revoked.pull-request-review`:

```json
{
  "entity_type": "process",
  "reason": "abrupt_success_rate_drop",
  "since": "2026-10-02T18:42:59Z"
}
```

**Radamanto** — 2 samples `exit_code: 1` (~207–228 s), `status: healthy`, `structure_valid: false`.

**Umbrales** — `SddIA/agents/radamanto.thresholds.json` v1.3.0 sin edición (L-THRESH).

## T1 — Después (instancia)

| Check | Resultado |
|-------|-----------|
| `revoked.pull-request-review` | **ausente** |
| `permanent.pull-request-review` | **ausente** |
| stats PPR | `healthy`, `recovery_attempts: 0`, `samples: []`, `rehab_laudo: AUT-62 / PBI-c4293e98`, `rehabilitated_at: 2026-11-02T07:16:44Z` |

## T2

Evolution `69228e55-be56-4578-8bf3-1207b4eb31a2` + `Evolution_log.md`.

## T3 / T4

| Paso | Resultado |
|------|-----------|
| PPR lab (`aut62-verify-ppr`) | `success: true`, `exitCode: 0` · `F4_RBAC_GATE: APTO` · `F5_VERDICT: APTO` · `delivery_state: success` · sin `FAIL_F4_RBAC` por PPR revocado |
| Cerbero post-T3 | `pull-request-review` ∉ `revoked` / `permanent` |
| Gate pre-PR QA (predicado) | `validate_attestation_for_branch_head` + atestación `qa-attestation/1.0` al `tree_sha` de HEAD (ver validacion.md) |
