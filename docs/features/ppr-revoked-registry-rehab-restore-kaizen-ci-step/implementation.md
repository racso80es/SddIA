---
feature_name: ppr-revoked-registry-rehab-restore-kaizen-ci-step
created: "2026-09-05"
updated: "2026-09-05T11:47:42Z"
process: refactorization
phase: implementation
agents: tekton
items:
  - T1-instance-rehab
  - T2-evolution
  - T3-smoke-ppr
branch_name: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
pbi_ref: docs/todos/pending/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
olas:
  - A1
runtime_execution_id: "4fe5d41e-5ebb-430c-96c9-3f3a31b0103b"
---

# Implementation — ppr-revoked-registry-rehab-restore-kaizen-ci-step

## T1 — Instancia (fuera del PR)

| Touchpoint | Mutación |
|------------|----------|
| `.SddIA/cerbero/revoked_entities.json` | DELETE `revoked.pull-request-review` |
| `.SddIA/radamanto/stats.json` | Reset absoluto bucket `pull-request-review` (L-SAMPLES) |

Laterales no tocados. Umbrales no tocados. Motor no tocado.

## T2 — Diff Git

| Artefacto | Rol |
|-----------|-----|
| Cascada persist_ref | clarify → validacion |
| `SddIA/evolution/e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17.md` | Cicatriz A1 |
| `SddIA/evolution/Evolution_log.md` | Índice |

## T3 — Smoke

Inyección `pull-request-review` detached. Sin join. Assert post-acuse: entidad ∉ `revoked`.
