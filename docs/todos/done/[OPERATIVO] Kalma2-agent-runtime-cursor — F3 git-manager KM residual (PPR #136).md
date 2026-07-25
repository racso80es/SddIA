---
document_id: PBI-PPR-136-KALMA2-AGENT-RUNTIME-RESIDUAL
title: "[OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136)"
format: markdown
version: "1.0.0"
created: "2026-07-22"
updated: "2026-07-25"
status: done
priority: alta
process: feature
uuid: 3d9bb1de-e45d-49fe-99f7-9b0b31d79c1d
source_feature: docs/features/inyeccion-dependencias-envelope-homologacion
source_correlation_id: e3079c94-2a40-4f60-b9c4-b4ade1ca031b
source_audit: docs/features/inyeccion-dependencias-envelope-homologacion/validacion.md
feature_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual
validacion_ref: docs/features/kalma2-agent-runtime-cursor-f3-km-residual/validacion.md
branch_name: feat/kalma2-agent-runtime-cursor-f3-km-residual
source_pr_url: https://github.com/racso80es/SddIA/pull/136
pr_url: https://github.com/racso80es/SddIA/pull/159
merge_commit: c987dcbd4d4248861a06ae3b0cca9793a56d5134
merged_at: "2026-07-25T08:17:40Z"
related:
  - docs/todos/done/[OPERATIVO] Kalma2 PPR runtime — F3 execute-process, git-manager y KM policy (PPR #125).md
  - SddIA/skills/git-manager.md
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - SddIA/process/pull-request-review.md
incident_ref: "PPR #136 — TECH_FORMAL_EXECUTE_PROCESS, GIT_EVIDENCE_VIA_GIT_MANAGER, RBAC_AUTHORING_KM_POLICY (residual path kalma2-agent-runtime-cursor)"
---

# [OPERATIVO] Kalma2-agent-runtime-cursor — F3 git-manager KM residual (PPR #136)

## Mandato

Cerrar el **residual** de aduana PPR cuando el runtime es `kalma2-agent-runtime-cursor` (CLI/agente), no el handler nativo ya cubierto por PPR #125.

| ID | Check origen | Deuda observada (PPR #136) | Nota vs #125 |
|----|--------------|----------------------------|--------------|
| **R1** | `TECH_FORMAL_EXECUTE_PROCESS` | F3 formal no invocado en sesión agent-runtime | #125 cerró `ppr-tech-triage` nativo |
| **R2** | `GIT_EVIDENCE_VIA_GIT_MANAGER` | `./sddia-run.sh --tool git-manager` rechazado (Shell/Auto-review); sin stdout físico | #125 cerró `ppr-prep-branch` nativo |
| **R3** | `RBAC_AUTHORING_KM_POLICY` | Política KM sigue NO_APTO en dictamen Argos agent path | #125 marcó G3 done; residual runtime |

## Criterio de cierre

- [x] F3 formal ejecutable desde `kalma2-agent-runtime-cursor` (o evidencia nativa inyectada al agent handoff) → check APTO.
- [x] Evidencia `git-manager` materializada en sesiones agent-runtime (o bypass soberano documentado) → `GIT_EVIDENCE_VIA_GIT_MANAGER: APTO`.
- [x] Autoría KM Cumulo-only enforceable en agent-runtime sin falso NO_APTO.

## Fuera de alcance

- Reapertura del PBI-042 (Hito 4 envelope; multi-hito).
- Rehabilitación `revoked_entities` de `delivery-close-cycle` (seed ARQUITECTURA PPR #136 aparte).
- Merge histórico PR #136.
