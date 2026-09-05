---
schema: kalma2-agent-runtime-evidence/v1
phase: Certificación RBAC
agent: cerbero
process: pull-request-review
materialized_at: "2026-09-04T13:41:00Z"
execution_id: 0b826e3b-ed99-4313-aa30-b2cc3c970d3f
correlation_id: DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm
persist_ref: docs/ppr-cosecha-kaizen-20260904
sibling_race_cid: e4c9970f-9e15-40fe-857f-07c44c1bada5
sibling_race_exec: d50a40ba-d3dc-4e8e-bc54-c3a13eb60800
global: NO_APTO
resolution: FAIL_F4_RBAC
authorization_status:
  exitCode: 1
  signer_identity_rbac: Vertice_Biologico_Relay
  emitter_agent: github-bridge-watcher
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

# Sidecar Cerbero F4 — CID DK5Qu… · exec 0b826e3b…

Ancla anti-carrera frente a sibling concurrent `e4c9970f…` / exec `d50a40ba…` (mismo PR #255 / sink).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F4_RBAC` |
| `exitCode` | `1` |
| `F4_RBAC_GATE` | `NO_APTO` |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-08-29T05:01:52Z` (`abrupt_success_rate_drop`) |
| `RBAC_EMITTER_NOT_REVOKED` | `APTO` — emisor `github-bridge-watcher` ∉ revoked |
| `signer` | `Vertice_Biologico_Relay` × docs sink |
| `F2_DOC_GATE` (heredado) | `APTO` / `PASS_F2_DOC` (sidecar `_argos_triaje_0b826e3b.md`) |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected; sin `gitStdout` inventado |
| Evidence Bridge R1/R2 | `prosthesis_subprocess` · TECH_FORMAL APTO · GIT_EVIDENCE APTO |
| ECST | `PullRequest_Presented` · emisor `github-bridge-watcher` · origin `jules` · PR #255 |

**Dictamen:** `blocked` · peaje `RBAC_PROCESS_REGISTRY` · reclaim `validacion.md` post-carrera sibling `d50a40ba` · downstream Veredicto/Cosecha/Handoff.
