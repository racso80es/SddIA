---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
version_clarify: "1.0.0"
---

# Clarificación — Causa raíz confirmada

## 1. Incidente (PR #23)

| Campo | Evidencia |
|-------|-----------|
| Rama | `fix/delivery-close-hook-eda-governance` |
| Evento | `PullRequest_Presented` `c2573529-ca49-4716-bbf9-ae77135be8fe` → `dead-letter/` |
| Aduana | Triaje técnico: 5× `hash_signature mismatch` (file=post-recalc, computed=pre-recalc) |
| Directo | `python SddIA/scripts/qa/verify-process-integrity.py` → **OK** (misma sesión tras `fetch` + `checkout` manual) |

## 2. Laudo técnico

**Causa raíz (H1 + H3):** `capsule_pr_review_branch_prep` hacía `git checkout <rama>` sobre la **rama local** sin alinear el worktree con `origin/<rama>` tras `fetch --prune`. Si la rama local estaba detrás del remoto (o el operador tenía cambios mezclados), el subproceso `verify-process-integrity` leía `SddIA/process/*.md` con **frontmatter** ya actualizado en remoto pero **contenido de `phases`** coherente con un snapshot anterior → mismatch sistemático.

`verify-process-integrity.py` resuelve `REPO` por ruta del script (no por `cwd`); el fallo no era un segundo repositorio, sino **contenido desincronizado en el mismo worktree**.

## 3. Corrección adoptada

| Cambio | Detalle |
|--------|---------|
| `_sync_pr_review_worktree` | Tras `fetch`+`prune`, si existe `origin/<branch>` → `git checkout -B <branch> origin/<branch>`; si no, checkout local |
| Triaje técnico | `SDDIA_REPO_ROOT=<repo>` en subproceso verify (defensa en profundidad) |
| Smoke | `smoke-pr-review-verify-integrity.py` con modo `--legacy-checkout` vs sync |

## 4. Fuera de alcance inmediato

- Extender contrato congelado `git-manager` con operación `reset` (sustituido por `checkout -B` vía `shell-executor`).
- Re-merge PR #23 (ya materializado vía `accept-pr`); retroactivo EDA en Hito 3.
