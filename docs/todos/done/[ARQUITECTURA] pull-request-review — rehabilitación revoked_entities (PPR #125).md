---
document_id: PBI-PPR-125-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125)"
format: markdown
version: "1.1.0"
created: "2026-07-21"
updated: "2026-07-22"
status: done
priority: media
process: bug-fix
uuid: c4e8f2a1-9b3d-4f6e-a7c2-1d5e8f9a0b3c
source_feature: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
source_correlation_id: 8Bnq4p1hzQxat79duyKxq7iH6qkJDS8jr7myYYZ5Sebf
fix_ref: docs/fixes/ppr-rehab-revoked-entities
validacion_ref: docs/fixes/ppr-rehab-revoked-entities/validacion.md
branch_name: fix/ppr-rehab-revoked-entities
closed_with: PBI-PPR-124-REVOKED-REGISTRY
pr_url: https://github.com/racso80es/SddIA/pull/125
related:
  - .SddIA/cerbero/revoked_entities.json
  - SddIA/process/pull-request-review.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
incident_ref: "RBAC_PROCESS_REGISTRY:NO_APTO — pull-request-review en revoked_entities"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125)

## Laudo Cerbero (2026-07-22)

**Rehabilitación** conjunta con `PBI-PPR-124-REVOKED-REGISTRY` (mismo check; un solo fix).

Causa raíz observada: `latency_threshold` por wall-clock de fases `agent:` (~703s outlier), no fallo de aduana (F2–F4 APTO).

## Criterio de cierre

- [x] Laudo rehabilitación
- [x] Instancia + política Radamanto
- [x] Cascada `docs/fixes/ppr-rehab-revoked-entities/` APTO
