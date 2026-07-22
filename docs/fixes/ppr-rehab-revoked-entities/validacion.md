---
feature_name: ppr-rehab-revoked-entities
created: "2026-07-22"
process: bug-fix
branch_name: fix/ppr-rehab-revoked-entities
persist_ref: docs/fixes/ppr-rehab-revoked-entities
pbi_ref: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
document_ids:
  - PBI-PPR-124-REVOKED-REGISTRY
  - PBI-PPR-125-REVOKED-REGISTRY
global: APTO
pbi_archived: true
branch: fix/ppr-rehab-revoked-entities
approval_status: approved
uuid: 23a81b0e-3930-4589-b5db-25ddd8eb5717
checks:
  LAUDO_REHABILITACION: APTO
  INSTANCE_REVOKED_ABSENT: APTO
  RADAMANTO_STATUS_HEALTHY: APTO
  LATENCY_EXEMPT_CODE: APTO
  UNIT_TEST: APTO
  PBI_124_ARCHIVED: APTO
  PBI_125_ARCHIVED: APTO
  RBAC_PROCESS_REGISTRY: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - docs/fixes/ppr-rehab-revoked-entities/
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
  - SddIA/evolution/23a81b0e-3930-4589-b5db-25ddd8eb5717.md
---

# Validación — Rehabilitación PPR revoked_entities

## Veredicto

**APTO** — laudo rehabilitación; instancia limpia; exención latency versionada; ambos PBIs en `done/`.

| Check | Estado |
|-------|--------|
| `RBAC_PROCESS_REGISTRY` | APTO (PPR ausente en revoked) |
| `LATENCY_EXEMPT_CODE` | APTO |
| PBI #124/#125 archivados | APTO |
