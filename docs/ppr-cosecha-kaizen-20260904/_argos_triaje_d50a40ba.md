---
schema: kalma2-agent-runtime-evidence/v1
phase: Triaje documental
agent: argos
process: pull-request-review
materialized_at: "2026-09-04T13:36:00Z"
execution_id: d50a40ba-d3dc-4e8e-bc54-c3a13eb60800
correlation_id: e4c9970f-9e15-40fe-857f-07c44c1bada5
sibling_race_exec: 0b826e3b-ed99-4313-aa30-b2cc3c970d3f
sibling_race_cid: DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm
persist_ref: docs/ppr-cosecha-kaizen-20260904
---

# Sidecar Argos F2 — CID e4c9970f… · exec d50a40ba…

Ancla anti-carrera frente a sibling concurrent `0b826e3b…` / CID `DK5Qu…` (mismo PR #255 / branch `docs/local-ledger-20260904`).

| Campo | Valor |
|-------|-------|
| `resolution` | `PASS_F2_DOC` |
| `global` | `APTO` |
| `F2_DOC_GATE` | `APTO` |
| `F3`/`F4`/`F5` | `pending` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `prosthesis_subprocess`; notes none) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected |
| `BRANCH_WORKTREE_SYNC` | `NO_APTO` — inject `docs/local-ledger-20260904` ≠ FS `.git/HEAD` → `main` |
| `PERSIST_REF_RESOLVED` | `APTO` → `docs/ppr-cosecha-kaizen-20260904` (DCC ledger; isomorfo ausente) |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `event_type` | `PullRequest_Presented` · emisor `delivery-close-cycle` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/255` |
| `delivery_state` | `pending_downstream_phases` |

**Dictamen:** `ok` · `PASS_F2_DOC` · downstream Triaje técnico → Cerbero F4 (PPR ∈ revoked peaje).
