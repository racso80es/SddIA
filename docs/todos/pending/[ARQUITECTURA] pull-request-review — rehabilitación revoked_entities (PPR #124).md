---
document_id: PBI-PPR-124-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124)"
format: markdown
version: "1.0.0"
created: "2026-07-21"
status: abierto
priority: media
process: bug-fix
uuid: a3f7c2e8-4d1b-4a9f-b6e3-8c2d5f1a0e9b
source_feature: docs/features/kaizen-kalma2-feature-cycle-observability
source_correlation_id: G79QSzhWBfGLLEQ1HhJiyAjcCfdCt1SCFY2RHTRjG66F
source_audit: docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
pr_url: https://github.com/racso80es/SddIA/pull/124
related:
  - .SddIA/cerbero/revoked_entities.json
  - SddIA/process/pull-request-review.md
  - docs/features/kaizen-kalma2-feature-cycle-observability/validacion.md
  - docs/todos/pending/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
incident_ref: "RBAC_PROCESS_REGISTRY:NO_APTO — pull-request-review en revoked_entities desde 2026-06-10"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124)

## Mandato

Retirar o justificar formalmente la entrada `pull-request-review` en `.SddIA/cerbero/revoked_entities.json` tras evidencia de aduana APTO en PR #124 (`correlation_id: G79QSzhWBfGLLEQ1HhJiyAjcCfdCt1SCFY2RHTRjG66F`).

| Campo | Valor |
|-------|--------|
| Check origen | `RBAC_PROCESS_REGISTRY: NO_APTO` |
| Entidad | `pull-request-review` (`entity_type: tool`) |
| Motivo registro | `abrupt_success_rate_drop` (2026-06-10) |
| Evidencia contraria | F2/F3/F4 APTO; `authorization_status.exitCode: 0`; witness bus Kaizen PR #124 |

## Deduplicación

- Seed hermana PR #125 (`PBI-PPR-125-REVOKED-REGISTRY`) cubre el mismo check en aduana posterior; cerrar en un único fix o laudo conjunto.

## Criterio de cierre

- [ ] Laudo Cerbero: rehabilitación, sustitución por política de observabilidad, o documentación de revocación permanente con alternativa operativa.
- [ ] `revoked_entities.json` coherente con el laudo (sin drift respecto a procesos activos en `main`).
- [ ] `validacion.md` del fix con `global: APTO` y PBI en `docs/todos/done/` en el mismo PR.

## Fuera de alcance

- Reescritura del proceso `pull-request-review` (v2.2.0 vigente).
- Merge de PR #124 (handoff `accept-pr`).
