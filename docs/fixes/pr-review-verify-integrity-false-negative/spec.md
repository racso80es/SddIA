---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
base: main
scope: pull-request-review-verify-integrity
version_spec: "1.0.0"
---

# Especificación — Sync worktree aduana PR

## Hito 1 — Reproducción

- Script `SddIA/scripts/qa/smoke-pr-review-verify-integrity.py`.
- Modo `--legacy-checkout`: `fetch` + `checkout` local (patrón pre-fix).
- Modo default: `_sync_pr_review_worktree` + verify con `SDDIA_REPO_ROOT`.

## Hito 2 — Corrección

### 2.1 `execute_process_capsules.py`

```python
def _sync_pr_review_worktree(repo, branch):
    fetch(prune=True)
    if origin/{branch} exists:
        git checkout -B {branch} origin/{branch}
    else:
        git checkout {branch}
```

`capsule_pr_review_technical`: exportar `SDDIA_REPO_ROOT` al subproceso verify.

### 2.2 `verify-process-integrity.py`

- `_repo_root()` honra `SDDIA_REPO_ROOT` cuando está definido.

## Hito 3 — Retroactivo EDA

- Re-emitir o re-enrutar `c2573529-…` desde `dead-letter/` tras fix en lab (documentar en `validacion.md`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Rama con recalc en `origin` → `pull-request-review` triaje técnico passed |
| CA-2 | `verify-process-integrity` con `SDDIA_REPO_ROOT` equivalente a invocación directa |
| CA-3 | Smoke sync vs legacy documentado en `execution.md` |
