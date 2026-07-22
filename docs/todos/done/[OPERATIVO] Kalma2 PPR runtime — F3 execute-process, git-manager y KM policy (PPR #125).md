---
document_id: PBI-PPR-125-KALMA2-RUNTIME-GAPS
title: "[OPERATIVO] Kalma2 PPR runtime — F3 execute-process, git-manager y KM policy (PPR #125)"
format: markdown
version: "1.1.0"
created: "2026-07-21"
updated: "2026-07-22"
status: done
priority: alta
process: bug-fix
uuid: 7d2b9e4f-1a8c-4e6b-9f3d-2c5a8b1e0d7f
source_feature: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
source_correlation_id: 8Bnq4p1hzQxat79duyKxq7iH6qkJDS8jr7myYYZ5Sebf
source_audit: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
fix_ref: docs/fixes/kalma2-ppr-runtime-gaps-ppr-125
validacion_ref: docs/fixes/kalma2-ppr-runtime-gaps-ppr-125/validacion.md
branch_name: fix/kalma2-ppr-runtime-gaps-ppr-125
pr_url: https://github.com/racso80es/SddIA/pull/125
related:
  - SddIA/skills/git-manager.md
  - SddIA/norms/external-ai-constraints.md
  - SddIA/process/pull-request-review.md
  - docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
  - docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
incident_ref: "PPR #125 — TECH_FORMAL_EXECUTE_PROCESS, GIT_EVIDENCE_VIA_GIT_MANAGER, RBAC_AUTHORING_KM_POLICY, BRANCH_RUNTIME_INJECT (parcial)"
---

# [OPERATIVO] Kalma2 PPR runtime — F3 execute-process, git-manager y KM policy (PPR #125)

## Mandato

Cerrar huecos de runtime Kalma2-agent detectados en la aduana PR #125 sin bloquear el peaje F2–F4 ya APTO.

| ID | Check origen | Deuda | Cierre |
|----|--------------|-------|--------|
| **G1** | `TECH_FORMAL_EXECUTE_PROCESS` | F3 no formal | handler `ppr-tech-triage` |
| **G2** | `GIT_EVIDENCE_VIA_GIT_MANAGER` | Prep sin cápsula | handler `ppr-prep-branch` |
| **G3** | `RBAC_AUTHORING_KM_POLICY` | Tekton×KM | regla runtime Cumulo-only |
| **G4** | `BRANCH_RUNTIME_INJECT` | `branch_name: None` | coalesce `pr_branch`→`branch_name` |

## Criterio de cierre

- [x] F3 formal: `ppr-tech-triage` + smoke `formal_execute_process: true`
- [x] `git-manager`: Prep nativo con `git_manager_invoked: true`
- [x] Política KM: prompt Kalma2 — seeds `docs/todos/` solo Cumulo / evento
- [x] `branch_name` desde ECST/`pr_branch` (route + agent_runtime + prompt)

## Fuera de alcance

- Contenido funcional F1 Fractura Core / GesFer.
- Merge histórico PR #125.
- Rehabilitación `revoked_entities` (seed ARQUITECTURA aparte).
- Fractura SSL Kalma2-bridge (soft-dep).
