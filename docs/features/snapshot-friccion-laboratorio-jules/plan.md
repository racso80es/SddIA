---
feature_name: snapshot-friccion-laboratorio-jules
process: feature
created: "2026-06-11"
phases:
  - id: T1
    name: Git failsoft offline
    tracks: [O1, CA-1]
  - id: T2
    name: Norma DA-4 Raw Kernel
    tracks: [O2, CA-2]
  - id: T3
    name: Skill intent-transpiler
    tracks: [O3, CA-3]
  - id: T4
    name: Cierre documental
    tracks: [O4, CA-5]
---

# Plan — snapshot-friccion-laboratorio-jules

## Fase T1 — Git failsoft (O1)

- [x] Añadir detección offline en `scripts/skills/git-manager.py` (`fetch`, `pull`, `push`)
- [x] Propagar envelope `offline: true` con `exitCode: 0` en cápsula
- [x] Actualizar `invoke_git_manager` / `_invoke_git_manager_native` para no elevar excepción
- [x] Tolerar `fetch`/`pull` offline en `run_workspace_init`
- [x] Smoke: detección marker Jules

## Fase T2 — DA-4 Raw Kernel (O2)

- [x] Ampliar `SddIA/norms/external-ai-constraints.md` v1.1.0 — DA-4 + prefijo creator
- [x] Referencia en `.cursorrules` § blindaje IA

## Fase T3 — intent-transpiler (O3)

- [x] Forja vía `entity-manager` → `skill-creator`
- [x] Completar contrato I/O en `SddIA/skills/intent-transpiler.md`
- [x] Indexar en `SddIA/skills/index.md`

## Fase T4 — Cierre (O4–O5)

- [x] `implementation.md` + `execution.md`
- [x] `validacion.md` APTO, `pbi_archived: true`
- [x] Mover PBI a `docs/todos/done/`
- [x] Entrada `SddIA/evolution/c9e2a1f0-8b4d-4e6f-9a0c-1d2e3f4a5b6c.md`
- [x] `verify-process-integrity.py` exit 0
