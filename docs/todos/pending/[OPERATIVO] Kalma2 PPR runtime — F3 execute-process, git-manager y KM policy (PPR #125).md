---
document_id: PBI-PPR-125-KALMA2-RUNTIME-GAPS
title: "[OPERATIVO] Kalma2 PPR runtime — F3 execute-process, git-manager y KM policy (PPR #125)"
format: markdown
version: "1.0.0"
created: "2026-07-21"
status: abierto
priority: alta
process: bug-fix
uuid: 7d2b9e4f-1a8c-4e6b-9f3d-2c5a8b1e0d7f
source_feature: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
source_correlation_id: 8Bnq4p1hzQxat79duyKxq7iH6qkJDS8jr7myYYZ5Sebf
source_audit: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
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

| ID | Check origen | Deuda |
|----|--------------|-------|
| **G1** | `TECH_FORMAL_EXECUTE_PROCESS` | Fase F3 no invocada vía `execute-process` en runtime Kalma2; evidencia proxy en auditoría |
| **G2** | `GIT_EVIDENCE_VIA_GIT_MANAGER` | Cápsula `git-manager` no materializa stdout en sesión agente (Shell/Auto-review rejected) |
| **G3** | `RBAC_AUTHORING_KM_POLICY` | Agente obrero (Tekton) sin `knowledge-management` en paths KM al materializar TODOs |
| **G4** | `BRANCH_RUNTIME_INJECT` | Input runtime `branch_name: None` (ECST/PR #125 sí alinean rama) |

## Deduplicación

- **G4** parcialmente cubierto por feature Kaizen `kaizen-kalma2-feature-cycle-observability` (PR #124, `AC_O4`). Este PBI exige verificación en el camino feliz PR #125, no reimplementar O4.

## Criterio de cierre

- [ ] F3 formal: invocación `execute-process` documentada en `execution.md` de un smoke PPR Kalma2 (o laudo de excepción proxy permanente).
- [ ] `git-manager`: invocación JSON stdin/stdout reproducible desde runtime agente (`sddia-run.sh --tool git-manager`) sin bypass raw.
- [ ] Política KM: agentes externos con mandato de escribir bajo `docs/todos/` cumplen `external-ai-constraints.md` o delegación vía `entity-manager`.
- [ ] `branch_name` propagado desde ECST/Kalma2 al runtime PPR (test o witness).

## Fuera de alcance

- Contenido funcional F1 Fractura Core / GesFer.
- Merge PR #125.
