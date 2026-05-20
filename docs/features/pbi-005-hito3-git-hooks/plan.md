---
feature_name: pbi-005-hito3-git-hooks
created: "2026-05-20"
process: feature
phase: planning-ola-a
---

# Plan — Ola A (pre-commit) y roadmap Ola B

## Fase documental (cerrada)

- [x] `objectives.md` + inicio `feature` (`workspace-init`)
- [x] `clarify.md` (D1–D12)
- [x] `spec.md` v1.0.0 + ADN decisiones + nota Fase 1 / 1b
- [x] `plan.md` (este archivo)

## Fase 1 — Ola A: materialización `pre-commit` (cerrada en lab)

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 1.1 | Estructura SSOT `SddIA/scripts/qa/git-hooks/` | directorio | [x] |
| 1.2 | Wrapper `pre-commit` (sh) + `pre_commit_gate.py` | fail-fast VPI → audit | [x] |
| 1.3 | Criterio EDA: **Existencia en Bus** (`--scan`, bus completo) | `pre_commit_gate.py` | [x] |
| 1.4 | Bypass humano `SDDIA_SKIP_HOOKS=1` | shell + Python | [x] |
| 1.5 | Gate de activación: `verify-process-integrity` OK en rama | smoke local | [x] |
| 1.6 | Gate de activación: `audit --scan` → `orphan_count: 0` | smoke local | [x] |
| 1.7 | Instalación local documentada | `implementation.md`, `install-hooks.ps1` | [x] |
| 1.8 | Smoke gate (`validacion.md`) | `validacion.md` APTO Ola A | [x] |
| 1.9 | Enlace backlog (objetivo C) | manifiesto post-PR11 | [x] |

### Instalación local (operador)

```powershell
# Desde raíz del repo (Git Bash o sh de Git for Windows)
cp SddIA/scripts/qa/git-hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit .git/hooks/../  # Git Bash
# Alternativa: core.hooksPath apuntando a git-hooks/ (equipo)
```

```powershell
# Prueba sin instalar en .git/hooks
python SddIA/scripts/qa/git-hooks/pre_commit_gate.py
echo $LASTEXITCODE
```

## Fase 1b — Diagnóstico (opcional, no gatekeeper)

| # | Tarea | Notas | Estado |
|---|-------|-------|--------|
| 1b.1 | Flag `--require-pending-for-staged` en `audit-entity-eda-coverage.py` | Solo mantenimiento Argos; **no** en cadena `pre-commit` | [ ] |
| 1b.2 | Documentar uso en runbook de auditoría masiva | `spec.md` nota al pie | [x] |

## Fase 2 — Ola B (CA-3 hooks PR)

| # | Tarea | Depende de |
|---|-------|------------|
| 2.1 | H3.1 — contrato hooks en `SddIA/evolution/` o norma | laudo Ola A APTO |
| 2.2 | H3.2 — `pre-push` → `delivery-close-cycle` | 2.1 |
| 2.3 | H3.3 — `post-merge` → `accept-pr` | 2.1 |
| 2.4 | H3.5 — smoke + `validacion.md` con `event_ids` | 2.2–2.3 |
| 2.5 | CA-3 y DoD PBI-005 | validación Argos |

## Commits sugeridos

1. `docs: clarify/spec/plan hito3 git-hooks (Ola A ADN)`
2. `feat(qa): git-hooks pre-commit Argos gate`
3. `docs: implementation + validacion smoke Ola A`

Merge hacia `main` vía **`accept-pr`** cuando Argos emita **APTO**.
