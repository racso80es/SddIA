---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
branch_name: fix/pr-review-verify-integrity-false-negative
---

# Implementación — Sync worktree aduana PR

## Touchpoints

| # | Artefacto | Cambio |
|---|-----------|--------|
| H2.1 | `execute_process_capsules.py` | `_sync_pr_review_worktree`; `capsule_pr_review_branch_prep` delega sync |
| H2.2 | `execute_process_capsules.py` | `capsule_pr_review_technical` pasa `SDDIA_REPO_ROOT` |
| H2.3 | `verify-process-integrity.py` | `_repo_root()` + `SDDIA_REPO_ROOT` |
| H1 | `smoke-pr-review-verify-integrity.py` | Reproducción legacy vs sync |

## Notas

- `git checkout -B <branch> origin/<branch>` vía `shell-executor` (sin ampliar contrato congelado `git-manager`).
- Fallback `local-checkout` si la rama aún no existe en `origin`.
