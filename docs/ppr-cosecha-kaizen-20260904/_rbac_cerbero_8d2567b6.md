---
schema: kalma2-agent-runtime-evidence/v1
phase: Certificación RBAC
agent: cerbero
process: pull-request-review
materialized_at: "2026-09-04T13:22:00Z"
execution_id: 8d2567b6-86b3-413c-adc2-54cd206c4324
correlation_id: 74a57c11-6764-4a6a-92e6-7943faa48d35
persist_ref: docs/ppr-cosecha-kaizen-20260904
sibling_race_exec: e431afdf-b388-4c8f-a857-8e0973a3cdeb
global: NO_APTO
resolution: FAIL_F4_RBAC
authorization_status:
  exitCode: 1
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: git-hook-pre-push
  origin_agent: cerbero
checks:
  F4_RBAC_GATE: NO_APTO
  RBAC_CERBERO_CERT: NO_APTO
  RBAC_PROCESS_REGISTRY: NO_APTO
  RBAC_EMITTER_NOT_REVOKED: APTO
  RBAC_SPATIAL_INTEGRITY: APTO
  RBAC_VBR_GENOME_AREA: APTO
  RBAC_SIGNER_AUTHORIZED: APTO
  RBAC_AUTHORING_KM_POLICY: APTO
  F2_DOC_GATE: APTO
  TECH_FORMAL_EXECUTE_PROCESS: APTO
  GIT_EVIDENCE_VIA_GIT_MANAGER: APTO
  GIT_EVIDENCE_SESSION_SHELL: NO_APTO
---

# Sidecar Cerbero F4 — exec 8d2567b6… / CID 74a57c11…

Ancla anti-carrera frente a sibling concurrent exec `e431afdf…` (mismo sink / mismo CID).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F4_RBAC` |
| `exitCode` | `1` |
| `F4_RBAC_GATE` | `NO_APTO` |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-08-29T05:01:52Z` (`abrupt_success_rate_drop`) |
| `RBAC_EMITTER_NOT_REVOKED` | `APTO` — emisor `git-hook-pre-push` ∉ revoked |
| `signer` | `Vertice_Biologico_Relay` × docs sink |
| `F2_DOC_GATE` (heredado) | `APTO` / `PASS_F2_DOC` (sidecar `_argos_triaje_8d2567b6.md`) |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected; sin `gitStdout` inventado |
| Evidence Bridge R1/R2 | `prosthesis_subprocess` · TECH_FORMAL APTO · GIT_EVIDENCE APTO |
| ECST | `Local_QA_Requested` · emisor `git-hook-pre-push` |

**Dictamen:** `blocked` · peaje `RBAC_PROCESS_REGISTRY` · downstream Veredicto/Cosecha/Handoff.
