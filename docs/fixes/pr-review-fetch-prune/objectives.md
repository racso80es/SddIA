---
feature_name: pr-review-fetch-prune
created: "2026-05-22"
process: bug-fix
branch_name: fix/pr-review-fetch-prune
persist_ref: docs/fixes/pr-review-fetch-prune
related_incident: ia-obrera-blindaje PR #16 — aduana requirió SDDIA_LAB_SKIP_GIT_CHECKOUT
---

# Objetivos — pr-review-fetch-prune

## Misión

Corregir la invocación `git-manager fetch` en `pull-request-review` (Fase Preparación de rama) para cumplir el contrato congelado §3.7 (`remote` + `prune` obligatorios) y alinear el handler `workspace-init` del laboratorio con el proceso `bug-fix`, hoy acoplado solo a `feature_name`.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | **Fix fetch** | `capsule_pr_review_branch_prep` invoca `fetch` con `prune: true` |
| O2 | **Smoke aduana** | `pull-request-review` Fase 1 sin `SDDIA_LAB_SKIP_GIT_CHECKOUT` → exit 0 |
| O3 | **Handler bug-fix** | `workspace-init` activo con `branch_name` + `persist_ref` y rama `fix/` |
| O4 | **Documentación proceso** | `bug-fix.md` § Perfil laboratorio alineado a `feature.md` |

## No objetivos

- Cambiar contrato congelado `skill-io-git-manager-frozen.md`.
- Refactor global de todas las invocaciones git en el repo (solo el hallazgo confirmado).

## Ley aplicada

- Proceso `bug-fix` v1.2.0
- `SddIA/norms/skill-io-git-manager-frozen.md` §3.7
- Patrón documental `features-documentation-pattern` (subconjunto fix)
