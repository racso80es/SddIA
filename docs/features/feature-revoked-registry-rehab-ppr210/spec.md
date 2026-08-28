---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-feature-cerbero-a1
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — abrupt_success_rate_drop since 2026-08-28T05:25:41Z"
---

# Spec — ola A1 feature-revoked-registry-rehab-ppr210

## Misión

Saneamiento instancia `feature` (Yunque). **Cero** mutación motor.

## Contrato A1

```text
DELETE revoked["feature"]
stats["feature"] := healthy + laudo PBI-PPR-210-FEATURE-REVOKED-REGISTRY + samples []
```

## Laudos

**L-NO-ENGINE** · **L-REUSE-185** · **L-STOP** planning cumplido.
