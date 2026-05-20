---
feature_name: pbi-005-hito3-ola-b
created: "2026-05-20"
process: feature
phase: planning-ola-b
---

# Plan — Ola B (hooks ciclo PR / cierre CA-3)

## Fase documental (cerrada)

- [x] `objectives.md` + `workspace-init`
- [x] `clarify.md` + Resoluciones de Acero O1–O5
- [x] `spec.md` v1.0.0
- [x] `plan.md` (este archivo)

## Fase 1 — Contrato normativo (H3.1)

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 1.1 | Evolution H3.1 | `SddIA/evolution/git-hooks-ca3-ola-b-contract.md` | [x] |

## Fase 2 — Módulos compartidos y `pre-push` (H3.2, O1–O3)

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 2.1 | Utilidades repo, bus, heurística | `hook_common.py` | [x] |
| 2.2 | Gate pre-push | `pre_push_gate.py` | [x] |
| 2.3 | Wrapper shell | `git-hooks/pre-push` | [x] |

## Fase 3 — `post-merge` y resiliencia `accept-pr` (H3.3, O4)

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 3.1 | Gate post-merge | `post_merge_gate.py` | [x] |
| 3.2 | Wrapper shell | `git-hooks/post-merge` | [x] |
| 3.3 | Cápsula `accept-pr` + merge huérfano | `execute_process_capsules.py` | [x] |
| 3.4 | Payload anomalía en sello | `execute-action.py` | [x] |

## Fase 4 — Instalador dinámico (O5, H3.4)

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 4.1 | Refactor instalador Windows | `install-hooks.ps1` | [x] |
| 4.2 | Instalador Unix | `install-hooks.sh` | [x] |
| 4.3 | Revisión estática sin `gh pr merge` | grep en `git-hooks/` | [x] |

## Fase 5 — Documentación de ejecución y validación

| # | Tarea | Artefacto | Estado |
|---|-------|-----------|--------|
| 5.1 | Touchpoints | `implementation.md` | [x] |
| 5.2 | Registro smoke | `execution.md` | [x] |
| 5.3 | Smoke H3.5 + checks V-B* | `validacion.md` | [x] |
| 5.4 | PBI operativo v1.5.0 | manifiesto Ola A | [x] pre-merge |

## Commits sugeridos

1. `docs: plan pbi-005-hito3-ola-b`
2. `feat(qa): git-hooks Ola B pre-push post-merge + instalador dinámico`
3. `feat(qa): cápsula accept-pr resiliente merge huérfano (O4)`
4. `docs: implementation execution validacion Ola B`

Merge vía **`accept-pr`** cuando Argos emita **APTO**.
