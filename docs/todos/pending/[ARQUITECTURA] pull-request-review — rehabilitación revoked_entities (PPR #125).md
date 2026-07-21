---
document_id: PBI-PPR-125-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125)"
format: markdown
version: "1.0.0"
created: "2026-07-21"
status: abierto
priority: media
process: bug-fix
uuid: c4e8f2a1-9b3d-4f6e-a7c2-1d5e8f9a0b3c
source_feature: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
source_correlation_id: 8Bnq4p1hzQxat79duyKxq7iH6qkJDS8jr7myYYZ5Sebf
source_audit: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
pr_url: https://github.com/racso80es/SddIA/pull/125
related:
  - .SddIA/cerbero/revoked_entities.json
  - SddIA/process/pull-request-review.md
  - docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin/validacion.md
  - docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
incident_ref: "RBAC_PROCESS_REGISTRY:NO_APTO — pull-request-review en revoked_entities desde 2026-06-10"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125)

## Mandato

Retirar o justificar formalmente la entrada `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` tras evidencia de aduana APTO en PR #125 (`correlation_id: 8Bnq4p1hzQxat79duyKxq7iH6qkJDS8jr7myYYZ5Sebf`).

| Campo | Valor |
|-------|--------|
| Check origen | `RBAC_PROCESS_REGISTRY: NO_APTO` |
| Entidad | `pull-request-review` (`entity_type: tool`) |
| Motivo registro | `abrupt_success_rate_drop` (2026-06-10) |
| Evidencia contraria | F2/F3/F4 APTO; `authorization_status.exitCode: 0`; witness bus Kaizen PR #124 |

## Criterio de cierre

- [ ] Laudo Cerbero: rehabilitación, sustitución por política de observabilidad, o documentación de revocación permanente con alternativa operativa.
- [ ] `revoked_entities.json` coherente con el laudo (sin drift respecto a procesos activos en `main`).
- [ ] `validacion.md` del fix con `global: APTO` y PBI en `docs/todos/done/` en el mismo PR.

## Fuera de alcance

- Reescritura del proceso `pull-request-review` (v2.2.0 vigente).
- Merge de PR #125 (handoff `accept-pr`).
