---
schema: kalma2-agent-runtime-evidence/v1
phase: Veredicto y bloqueo
agent: argos
process: pull-request-review
materialized_at: "2026-09-04T12:56:00Z"
execution_id: 6362eb00-556c-4f34-96a1-d012c3541a06
correlation_id: 9c9cd653-dabe-4fe2-a54d-17f868cd427e
persist_ref: docs/ppr-cosecha-kaizen-20260904
---

# Sidecar Argos F5 — CID 9c9cd653…

Ancla anti-carrera frente a sibling concurrent `7293fada…` / exec `e21fc03d…` (mismo sink).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F5_VERDICT` |
| `global` | `NO_APTO` |
| `F5_VERDICT_GATE` | `NO_APTO` |
| `F4_RBAC_GATE` | `NO_APTO` — heredado Cerbero @ 12:52:00Z |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-08-29T05:01:52Z` |
| `F2_DOC_GATE` (heredado) | `APTO` / `PASS_F2_DOC` @ 12:42:00Z |
| `F3_TECH_GATE` | `NO_APTO` — sin Triaje técnico este CID |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `native_state` / `idempotent-hit`) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected |
| `BRANCH_WORKTREE_SYNC` | `NO_APTO` — `.git/HEAD` → `refs/heads/main` ≠ inject |

**Dictamen:** `blocked` · peaje F4→F5 · downstream Cosecha Kaizen (dedup) → Handoff **prohibido**.
