---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — re-revoked post-#185 since 2026-08-28T05:25:41Z"
ola: A1
olas:
  - A1
runtime_execution_id: "532a36c1-d46e-4c49-82ec-dbfc2ea50315"
---

# Objetivos — ola A1 feature-revoked-registry-rehab-ppr210

## Objetivo

Rehabilitar `feature` en instancia Cerbero/Radamanto (PPR #210). Solo **A1 Yunque**. Motor #185 reutilizado (T0 PASS).

## Alcance

1. DELETE `revoked.feature`. Reset stats raíz + laudo #210.
2. Evidencia `execution.md`. Instancia fuera del diff.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `feature` ∉ `revoked`; stats `healthy`; laudo #210. |
| AC-GIT-CLEAN | Sin `.SddIA/cerbero|radamanto` en diff. |
| AC-DOC | Cascada APTO + PBI `done/`. |
