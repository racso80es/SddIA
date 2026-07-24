---
feature_name: delivery-close-cycle-revoked-signer
created: "2026-07-24"
process: feature
branch_name: feat/delivery-close-cycle-revoked-signer
persist_ref: docs/features/delivery-close-cycle-revoked-signer
document_id: PBI-PPR-136-DCC-REVOKED-SIGNER
execution_id: 00b9e53d-d231-45f5-9685-4d2b86b7ab63
phase: tekton
agents: tekton
items:
  - E2 emit_pr_presented signer default
  - E2 action.md 1.1.1 + index
  - E1 rehab revoked_entities instancia
  - E1 radamanto status healthy
---

# Implementation

| # | Cambio | Estado |
|---|--------|--------|
| 1 | `actions.rs` `emit_pr_presented` → `payload.signer_identity_rbac` default `Vertice_Biologico_Relay` | done |
| 2 | `emit-pr-presented-event.md` v1.1.1 + `actions/index.md` (UUID preservado) | done |
| 3 | Instancia: remove `delivery-close-cycle` de `revoked_entities.json` | done |
| 4 | Instancia: stats root `delivery-close-cycle.status=healthy` | done |
| 5 | Rebuild `execute-process` (binario local) | done |

## Laudos de forja

| ID | Laudo |
|----|-------|
| **L-EM-ACTION-UPDATE** | `entity-manager` → `action-creator`/`run_action_forge` **no** soporta update seguro (regenera UUID + stub 80 chars). Intento EM falló sello (`hash_signature_old` vacío en legado) y truncó artefacto; restaurado HEAD y mutación manual bajo topología feature. |
| **L-E1-REHAB** | Rehabilitación: rate 0.947 > 0.85; `pending_redemption`; 1 fail aislado. Sin exención latency. `feature`/`bug-fix` intactos. |
