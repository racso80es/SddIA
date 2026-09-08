---
schema: kalma2-agent-runtime-evidence/v1
phase: Triaje documental
agent: argos
process: pull-request-review
materialized_at: "2026-09-08T12:35:00Z"
execution_id: 158fac27-fb74-4e52-815e-3e5ac3435926
correlation_id: CH1qptjxLgm5YhkSRLwhbJM9mCnKJuVUgGBZbiWaN8KB
sibling_veredicto_exec: bce7c6f9-32a1-4e8c-9f08-d994110147d9
sibling_veredicto_cid: a7641b9f-c6f7-409d-80fc-c2ff9981e4ae
persist_ref: docs/ingest-kintsugi-pbis-20260908
---

# Sidecar Argos F2 — CID CH1qptjx… · exec 158fac27…

Ancla anti-carrera frente a sibling Veredicto `bce7c6f9…` / CID `a7641b9f…` (mismo PR #271 / branch `docs/ingest-kintsugi-pbis-20260908`).

| Campo | Valor |
|-------|-------|
| `resolution` | `FAIL_F2_DOC` |
| `global` | `NO_APTO` |
| `F2_DOC_GATE` | `NO_APTO` |
| `F3`/`F4`/`F5` | `pending` |
| `TECH_FORMAL_EXECUTE_PROCESS` | `APTO` (session `prosthesis_subprocess`; notes none) |
| `GIT_EVIDENCE_VIA_GIT_MANAGER` | `APTO` (Evidence Bridge copia; sin stdout inventado) |
| `GIT_EVIDENCE_SESSION_SHELL` | `NO_APTO` — Shell Rejected |
| `BRANCH_WORKTREE_SYNC` | `APTO` — inject = FS `.git/HEAD` → `docs/ingest-kintsugi-pbis-20260908` |
| `PERSIST_REF_RESOLVED` | `APTO` → `docs/ingest-kintsugi-pbis-20260908` (isomorfo; cascada ausente) |
| `RBAC_AUTHORING_KM_POLICY` | `APTO` — 0 writes `docs/todos/**` |
| `event_type` | `PullRequest_Presented` · emisor `github-bridge-watcher` |
| `pr_url` | `https://github.com/racso80es/SddIA/pull/271` |
| `delivery_state` | `failed` |

**Dictamen:** `blocked` · `FAIL_F2_DOC` · cascada ausente; ledger KM no sustituye F2.
