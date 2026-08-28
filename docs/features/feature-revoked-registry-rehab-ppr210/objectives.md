---
feature_name: feature-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
branch_name: refactor/feature-revoked-registry-rehab-ppr210
persist_ref: docs/features/feature-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-FEATURE-REVOKED-REGISTRY
uuid: f8b2c3d4-5e6f-7a89-0b1c-2d3e4f5a6b7c
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
incident_ref: "REVOKED_ENTITY_ALERT_FEATURE — re-revoked post-#185 abrupt_success_rate_drop since 2026-08-28T05:25:41Z"
ola: A1
olas:
  - A1
runtime_execution_id: "532a36c1-d46e-4c49-82ec-dbfc2ea50315"
---

# Objetivos — ola A1 feature-revoked-registry-rehab-ppr210

## Objetivo

Rehabilitar el proceso `feature` en instancia Cerbero/Radamanto tras re-revocación `abrupt_success_rate_drop` since `2026-08-28T05:25:41Z` (PPR #210), post-cierre #185 A1–A3. Solo **A1 Yunque Rúnico**. Motor anti-recurrencia = código #185 ya en `main` (assert T0, no reabrir).

## Alcance

1. Eliminar `revoked.feature`. Assert `permanent.feature` ausente.
2. Reset absoluto bucket raíz `feature` (laudo #210, poda `samples`).
3. Laterales Cerbero intactos.
4. Evidencia en `execution.md`. Prohibido versionar instancia en el diff.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `feature` ∉ `revoked` ni `permanent`; stats `healthy`; `recovery_attempts: 0`; laudo `PBI-PPR-210-FEATURE-REVOKED-REGISTRY`; samples `[]`. |
| AC-GIT-CLEAN | Instancia ausente del diff del PR. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Cascada A1 bajo este `persist_ref`. |

## Fuera de alcance

- Reabrir A2/A3 motor #185 sin T0 FAIL.
- Rehab `accept-pr` (PPR #208) / `bug-fix` (ola hermana) / `refactorization` (#186).
- Residual Kalma2 (PPR #136).
- Mutar umbrales v1.1.0.
- Ejecutar T0–T5 en la sesión de planning.

## Restricciones

- Git solo `skill:git-manager`. Rama `refactor/feature-revoked-registry-rehab-ppr210`.
- Vehículo CLI `feature` + `process_label: refactorization`.

## Ley aplicada

- `features-documentation-pattern` v1.2.x.
- Jurisprudencia `L-REHAB-INST` + A2/A3 #185.
- `SddIA/norms/external-ai-constraints.md`.
