---
feature_name: l1-o5-runbooks-paridad
created: "2026-05-25"
process: feature
items:
  - docs/features/l1-o5-runbooks-paridad/runbook-accept-pr.md
  - SddIA/scripts/qa/verify-runbook-paridad.py
  - SddIA/norms/pull-request-orchestration.md
  - SddIA/norms/git-operations.md
  - docs/features/pbi-005-hito2-action-engine/execution.md
  - docs/features/pbi-005-debt-liquidation/execution.md
  - docs/features/pbi-005-hito3-git-hooks/execution.md
---

# Implementación — L1-O5 Runbooks paridad

## Touchpoints

| Ámbito | Cambio |
|--------|--------|
| **Runbook SSOT** | `runbook-accept-pr.md` — 10 secciones operativas |
| **Gate QA** | `verify-runbook-paridad.py` — escaneo `execution.md` + `pending/` |
| **Normas** | Enlace runbook en `pull-request-orchestration.md` §6 y `git-operations.md` §3 |
| **Legacy** | Banners + bloques `runbook-historical` en 3 `execution.md` históricos |
| **Cierre** | FIX + PBI post-PR11 → `docs/todos/done/` |

## Decisiones de implementación

1. **Gate acotado** — solo `execution.md` bajo `docs/features/` (no planning docs ni anti-patrones del runbook).
2. **Inmutabilidad histórica** — comandos legacy preservados dentro de `<!-- runbook-historical -->`.
3. **Sin cambios runtime** — cápsulas y hooks intactos; feature 100 % documental + gate.

## Propuestas no aplicadas (Kaizen)

- Integrar `verify-runbook-paridad` en `verify-process-integrity.py` — documentado en runbook §10; integración CI opcional posterior.
