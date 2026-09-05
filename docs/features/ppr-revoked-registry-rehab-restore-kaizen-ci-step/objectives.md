---
feature_name: ppr-revoked-registry-rehab-restore-kaizen-ci-step
created: "2026-09-05"
process: refactorization
phase: mayeuta-stabilization
agents: mayeuta
branch_name: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
pbi_ref: docs/todos/pending/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
source_correlation_id: "AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc"
feature_ref: docs/fixes/restore-pbi-kaizen-ci-step-archive
parent_pbi: docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — abrupt_success_rate_drop since 2026-08-29T05:01:52Z"
ola: A1
olas:
  - A1
runtime_execution_id: "4fe5d41e-5ebb-430c-96c9-3f3a31b0103b"
---

# Objetivos — ppr-revoked-registry-rehab-restore-kaizen-ci-step

## Objetivo

Rehabilitar `pull-request-review` en instancia Cerbero/Radamanto tras `abrupt_success_rate_drop` since `2026-08-29T05:01:52Z` (Cosecha `restore-pbi-kaizen-ci-step-archive` PR #247). Causa raíz operativa actual: entidad ∈ `revoked` → F4 `RBAC_PROCESS_REGISTRY: NO_APTO` → handoff `accept-pr` bloqueado. Receta A1: DELETE clave Cerbero + reset absoluto Radamanto con **L-SAMPLES**.

## Alcance

1. Eliminar `revoked.pull-request-review`. Assert `permanent.pull-request-review` ausente.
2. Reset absoluto bucket raíz `pull-request-review` (laudo este `document_id`, `samples: []`, `structure_valid: true`, `recovery_attempts: 0`).
3. Laterales Cerbero intactos (`bug-fix`, `delivery-close-cycle`, `entity-manager`, `feature`, `refactorization`).
4. Evidencia A1 en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff.
5. Smoke PPR: inyección detached, sin join (DA-5); post-acuse entidad ∉ `revoked`.
6. Cascada documental + evolution UUID `e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17`.
7. PR; CA6 CI verde antes de `global: APTO` sobre ese check.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `pull-request-review` ∉ `revoked` ni `permanent`; laterales intactos. |
| CA2 | `samples: []`; `healthy`; `structure_valid: true`; `recovery_attempts: 0`; `degraded_at: null`; `rehab_laudo` = este `document_id`. |
| CA3 | Diff PR sin `.SddIA/**` ni `radamanto.thresholds.json`. |
| CA4 | Acuse PPR `success` + `detached: true`; entidad sigue fuera de `revoked`. |
| CA5 | DCC vehículo `feature` / `process_label: refactorization`; sin rehab laterales. |
| CA6 | Checks GitHub del PR en verde (`run_id`/URL) antes de marcar este CA APTO. |

## Fuera de alcance

- Rehab `feature`, `refactorization`, `delivery-close-cycle`, `bug-fix`, `entity-manager`.
- Motor A2 (`radamanto_batch_core.rs`) — done PR #221.
- Umbrales y YAML `pull-request-review.md`.
- Merge/`accept-pr` de este PR.

## Restricciones

- Git solo `skill:git-manager`. Rama `refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step`.
- Init lab: vehículo `feature` + `process_label: refactorization`. `execution_id` `4fe5d41e-5ebb-430c-96c9-3f3a31b0103b`.
- PBI v1.2.0 H6–H10 vinculantes (Filtro A).
- `features-documentation-pattern` v1.2.1: CA de CI sin run verde ⇒ `PENDIENTE-CI`.

## Ley aplicada

- `features-documentation-pattern` v1.2.1.
- `SddIA/norms/external-ai-constraints.md` (DA-5; DA-2 no aplica a instancia).
- Jurisprudencia L-REHAB-INST / L-SAMPLES (fallo #190; éxito #208/#210/#220).
