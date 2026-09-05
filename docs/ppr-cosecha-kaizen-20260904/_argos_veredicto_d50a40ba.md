---
schema: kalma2-agent-runtime-evidence/v1
phase: Veredicto y bloqueo
agent: argos
process: pull-request-review
materialized_at: "2026-09-04T13:43:30Z"
execution_id: d50a40ba-d3dc-4e8e-bc54-c3a13eb60800
correlation_id: e4c9970f-9e15-40fe-857f-07c44c1bada5
persist_ref: docs/ppr-cosecha-kaizen-20260904
sibling_race_exec: 0b826e3b-ed99-4313-aa30-b2cc3c970d3f
sibling_race_cid: DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm
global: NO_APTO
resolution: FAIL_F5_VERDICT
---

# Sidecar Argos F5 — CID e4c9970f… · exec d50a40ba…

Ancla anti-carrera frente a sibling concurrent `0b826e3b…` / CID `DK5Qu…` (mismo sink / PR #255 / branch `docs/local-ledger-20260904`).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F5_VERDICT` |
| `global` | `NO_APTO` |
| `F5_VERDICT_GATE` | `NO_APTO` |
| `F4_RBAC_GATE` | `NO_APTO` — heredado Cerbero @ 13:40:00Z (`_rbac_cerbero_d50a40ba.md`) |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-08-29T05:01:52Z` |
| `RBAC_EMITTER_NOT_REVOKED` | `NO_APTO` L-OUT — emisor `delivery-close-cycle` ∈ revoked |
| `F2_DOC_GATE` (heredado) | `APTO` / `PASS_F2_DOC` (`_argos_triaje_d50a40ba.md`) |
| `F3_TECH_GATE` | `NO_APTO` — sin Triaje técnico este CID |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `native_state` / `idempotent-hit`) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — sin `gitStdout` físico |
| `BRANCH_WORKTREE_SYNC` | `NO_APTO` — inject `docs/local-ledger-20260904` ≠ FS `.git/HEAD` → `main` |

**Dictamen:** `blocked` · peaje F4→F5 · downstream Cosecha Kaizen (dedup) → Handoff **prohibido**.
