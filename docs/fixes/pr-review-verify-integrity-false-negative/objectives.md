---
feature_name: pr-review-verify-integrity-false-negative
created: "2026-05-22"
process: bug-fix
branch_name: fix/pr-review-verify-integrity-false-negative
persist_ref: docs/fixes/pr-review-verify-integrity-false-negative
related_incident: "PR #23 — aduana rechazó verify-process-integrity; verify directo OK"
---

# Objetivos — pr-review-verify-integrity-false-negative

## Misión

Corregir el **falso negativo** de `verify-process-integrity` en la Fase **Triaje técnico** de `pull-request-review`, cuando el worktree local no refleja `origin/<pr_branch>` tras `fetch`, y restaurar la trazabilidad EDA del evento `PullRequest_Presented` `c2573529-…` (PR #23).

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | Reproducir patrón | Smoke documentado: legacy checkout vs sync + verify |
| O2 | Corregir causa raíz | `capsule_pr_review_branch_prep` sincroniza con `origin/<branch>`; triaje técnico exit 0 si verify OK en rama remota |
| O3 | Retroactivo EDA PR #23 | Re-procesar dead-letter o documentar en `validacion.md` |
| O4 | Regresión | Smoke aduana / CI no depende de bypass `accept-pr` |

## No objetivos

- Cambiar algoritmo de `hash_signature` (recalc existente).
- Refactor global de `git-manager` (solo sync en aduana PR).
