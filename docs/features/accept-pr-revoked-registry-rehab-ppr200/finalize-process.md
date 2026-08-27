---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
process: refactorization
pr_url: https://github.com/racso80es/SddIA/pull/202
merged_pr: https://github.com/racso80es/SddIA/pull/202
merge_commit: 42fff0765f1b0986f1807b89586bbef3f53c0011
timestamp: "2026-08-27T12:15:00Z"
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
---

# Cierre — accept-pr-revoked-registry-rehab-ppr200

## Resumen

Rehabilitación `accept-pr` post re-revocación #194 + anti-recurrencia A2 (`fail_soft` sello `PullRequest_Merged` post-`merge_commit_hash`).

| Hito | Evidencia |
|------|-----------|
| PR mergeado | #202 · `42fff0765f1b0986f1807b89586bbef3f53c0011` |
| A1 instancia | `accept-pr` ∉ revoked · stats `healthy` · laudo `PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY` |
| A2 motor | `accept_pr.rs` + `residual_runner.rs` · tests `t_a2_seal_*` |
| Smoke handoff | `accept-pr` exit 0 · Merged `c3a80d66…` · sin re-revocación Cerbero |
| Documental | `validacion.md` APTO · PBI en `done/` · evolution `a8f3c1e2-…` |

## Done

```text
Done = PR #202 mergeado en main
     + validacion.md APTO (pbi_archived: true)
     + PBI en docs/todos/done/
     + smoke accept-pr sin re-revocación inmediata
```
