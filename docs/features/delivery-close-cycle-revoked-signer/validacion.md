---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
process: feature
branch: feat/delivery-close-cycle-revoked-signer
global: APTO
pbi_archived: true
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
pbi_ref: docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
pr_url: https://github.com/racso80es/SddIA/pull/158
ecst_event_id: 0e3c01a4-3bf5-44ac-a3f3-7b7bae1e531b
authorization_status:
  exitCode: 0
  signer_identity_rbac: Vertice_Biologico_Relay
checks:
  AC-E1: APTO
  AC-E2: APTO
  AC-SCOPE: APTO
  AC-DOC: APTO
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  TECH_EMIT_SIGNER: APTO
  TECH_CARGO_BUILD: APTO
  RBAC_SIGNER_PRESENT: APTO
  RBAC_EMITTER_NOT_REVOKED: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/actions/emit-pr-presented-event.md
  - SddIA/actions/index.md
  - SddIA/evolution/00b9e53d-d231-45f5-9685-4d2b86b7ab63.md
  - docs/features/delivery-close-cycle-revoked-signer/
  - docs/todos/done/[ARQUITECTURA] delivery-close-cycle — revoked_entities y ECST signer (PPR #136).md
---

# Validación — delivery-close-cycle-revoked-signer

## Veredicto

**APTO** (Argos relay IDE). E1 rehabilitación local + E2 firmante ECST verificados. `pbi_archived: true` — PBI en `docs/todos/done/`.

## Evidencia

| Check | Resultado | Evidencia |
|-------|-----------|-----------|
| **AC-E1** | APTO | `delivery-close-cycle` ∉ `revoked`; remaining `bug-fix`, `emit-pr-audited-event`, `feature`; stats `healthy` |
| **AC-E2** | APTO | Smoke emit → `payload.signer_identity_rbac=Vertice_Biologico_Relay` |
| **AC-SCOPE** | APTO | `feature`/`bug-fix` siguen revoked |
| **TECH_EMIT_SIGNER** | APTO | `actions.rs` default + action.md v1.1.1 |
| **RBAC_SIGNER_PRESENT** | APTO | empiría smoke local (aduana PPR post-merge confirmará) |
| **RBAC_EMITTER_NOT_REVOKED** | APTO | instancia local rehab (gitignore — reaplicar en host CI si aplica) |
| **AC-DOC** | APTO | PBI archivado en `done/` en rama del PR |

## Residual / notas

- Instancia `.SddIA/cerbero/` no viaja en git; runbook en `execution.md`.
- `L-EM-ACTION-UPDATE`: forja action update defectuosa — seed residual fuera de alcance.
