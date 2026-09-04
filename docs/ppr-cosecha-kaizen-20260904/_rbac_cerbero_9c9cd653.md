---
schema: kalma2-agent-runtime-evidence/v1
phase: Certificación RBAC
agent: cerbero
process: pull-request-review
materialized_at: "2026-09-04T12:52:00Z"
execution_id: 6362eb00-556c-4f34-96a1-d012c3541a06
correlation_id: 9c9cd653-dabe-4fe2-a54d-17f868cd427e
persist_ref: docs/ppr-cosecha-kaizen-20260904
---

# Sidecar Cerbero F4 — CID 9c9cd653…

Ancla anti-carrera frente a sibling concurrent `7293fada…` / exec `e21fc03d…` (mismo sink).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F4_RBAC` |
| `exitCode` | `1` |
| `F4_RBAC_GATE` | `NO_APTO` |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-08-29T05:01:52Z` |
| `RBAC_EMITTER_NOT_REVOKED` | `NO_APTO` L-OUT — emisor `delivery-close-cycle` ∈ revoked |
| `signer` | `Vertice_Biologico_Relay` |
| `F2_DOC_GATE` (heredado) | `APTO` / `PASS_F2_DOC` |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected; sin `gitStdout` inventado |
| Evidence Bridge R1/R2 | `prosthesis_subprocess` · TECH_FORMAL APTO · GIT_EVIDENCE APTO |

**Dictamen:** `blocked` · peaje `RBAC_PROCESS_REGISTRY` · downstream Veredicto/Cosecha/Handoff.
