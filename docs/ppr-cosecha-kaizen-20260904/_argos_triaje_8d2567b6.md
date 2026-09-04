---
schema: kalma2-agent-runtime-evidence/v1
phase: Triaje documental
agent: argos
process: pull-request-review
materialized_at: "2026-09-04T13:16:00Z"
execution_id: 8d2567b6-86b3-413c-adc2-54cd206c4324
correlation_id: 74a57c11-6764-4a6a-92e6-7943faa48d35
persist_ref: docs/ppr-cosecha-kaizen-20260904
---

# Sidecar Argos F2 — CID 74a57c11… · exec 8d2567b6…

Ancla anti-carrera frente a sibling concurrent `e431afdf…` (mismo CID / sink).

| Campo | Valor |
|-------|-------|
| `resolution` | `PASS_F2_DOC` |
| `global` | `APTO` |
| `F2_DOC_GATE` | `APTO` |
| `F3`/`F4`/`F5` | `pending` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `prosthesis_subprocess`; notes none) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected |
| `BRANCH_WORKTREE_SYNC` | `APTO` — inject `HEAD` ≡ FS `.git/HEAD` → `feat/gemini-http-infer-live-activation` |
| `WORKTREE_NE_FEATURE_BRANCH_DOCS` | L-OUT — docs branch `docs/ppr-cosecha-kaizen-20260904` ≠ worktree (no castra F2) |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `event_type` | `Local_QA_Requested` · emisor `git-hook-pre-push` |
| `delivery_state` | `pending_downstream_phases` |

**Dictamen:** `ok` · `PASS_F2_DOC` · downstream Triaje técnico → Cerbero F4 (PPR ∈ revoked peaje).
