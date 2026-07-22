---
document_id: PBI-PPR-124-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124)"
format: markdown
version: "1.1.0"
created: "2026-07-21"
updated: "2026-07-22"
status: done
priority: media
process: bug-fix
uuid: a3f7c2e8-4d1b-4a9f-b6e3-8c2d5f1a0e9b
source_feature: docs/features/kaizen-kalma2-feature-cycle-observability
source_correlation_id: G79QSzhWBfGLLEQ1HhJiyAjcCfdCt1SCFY2RHTRjG66F
fix_ref: docs/fixes/ppr-rehab-revoked-entities
validacion_ref: docs/fixes/ppr-rehab-revoked-entities/validacion.md
branch_name: fix/ppr-rehab-revoked-entities
closed_with: PBI-PPR-125-REVOKED-REGISTRY
pr_url: https://github.com/racso80es/SddIA/pull/124
related:
  - .SddIA/cerbero/revoked_entities.json
  - SddIA/process/pull-request-review.md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
incident_ref: "RBAC_PROCESS_REGISTRY:NO_APTO — pull-request-review en revoked_entities"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124)

## Laudo Cerbero (2026-07-22)

**Rehabilitación** de `pull-request-review` (no revocación permanente).

| Acción | Evidencia |
|--------|-----------|
| Retirar de `revoked` (instancia) | `.SddIA/cerbero/revoked_entities.json` sin clave PPR |
| Reset stats Radamanto → `healthy` | outlier 703s podado; `entities.pull-request-review.status=healthy` |
| Política anti-recurrencia | `LATENCY_THRESHOLD_EXEMPT` incluye `pull-request-review` |

Cierre conjunto con `PBI-PPR-125-REVOKED-REGISTRY` en `docs/fixes/ppr-rehab-revoked-entities/`.

## Criterio de cierre

- [x] Laudo Cerbero: rehabilitación
- [x] Instancia coherente + exención latency versionada
- [x] `validacion.md` APTO + PBI en `done/`
