---
schema: kalma2-agent-runtime-evidence/v1
phase: Veredicto y bloqueo
agent: argos
process: pull-request-review
materialized_at: "2026-09-08T12:45:00Z"
execution_id: 158fac27-fb74-4e52-815e-3e5ac3435926
correlation_id: CH1qptjxLgm5YhkSRLwhbJM9mCnKJuVUgGBZbiWaN8KB
persist_ref: docs/ingest-kintsugi-pbis-20260908
sibling_veredicto_exec: bce7c6f9-32a1-4e8c-9f08-d994110147d9
sibling_veredicto_cid: a7641b9f-c6f7-409d-80fc-c2ff9981e4ae
global: NO_APTO
resolution: FAIL_F2_DOC
---

# Sidecar Argos F5 — CID CH1qptjx… · exec 158fac27…

Ancla anti-carrera frente a sibling Veredicto `bce7c6f9…` / CID `a7641b9f…` (mismo PR #271 / branch inject `docs/ingest-kintsugi-pbis-20260908`).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F2_DOC` |
| `global` | `NO_APTO` |
| `F5_VERDICT_GATE` | `NO_APTO` |
| `F2_DOC_GATE` | `NO_APTO` — cascada ausente (canónico este exec) |
| `F3_TECH_GATE` | `NO_APTO` — no bloqueante (proxy TECH_FORMAL) |
| `F4_RBAC_GATE` | `NO_APTO` — `pull-request-review` ∈ revoked since `2026-09-05T13:34:54Z` |
| `RBAC_PROCESS_REGISTRY` | `NO_APTO` |
| `RBAC_EMITTER_NOT_REVOKED` | `APTO` — emisor `github-bridge-watcher` |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `native_state` / `idempotent-hit`) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — sin `gitStdout` físico |
| `BRANCH_WORKTREE_SYNC` | `NO_APTO` — inject `docs/ingest-kintsugi-pbis-20260908` ≠ FS `.git/HEAD` → `main` |
| `delivery_state` | `failed` |
| `accept_pr_handoff` | `false` / `blocked` |

**Dictamen:** `blocked` · `FAIL_F2_DOC` (+F4 co) · downstream Cosecha Kaizen (dedup) → Handoff **prohibido**.
