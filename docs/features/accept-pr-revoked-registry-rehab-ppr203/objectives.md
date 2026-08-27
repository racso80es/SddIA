---
feature_name: accept-pr-revoked-registry-rehab-ppr203
created: "2026-08-27"
process: refactorization
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr203
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr203
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
document_id: PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY
uuid: b7e4a91c-2f5d-4c8b-9e1a-6d3f0a8b2c7e
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "6237015f-0f8d-42ea-97ea-a44afac5318d"
source_pr_url: https://github.com/racso80es/SddIA/pull/203
feature_ref: docs/features/emit-pr-audited-revoked-registry-rehab-ppr202
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — re-revoked post-#200 abrupt_success_rate_drop since 2026-08-27T12:31:30Z"
ola: A1
olas:
  - A1
runtime_execution_id: "2363d1e8-8fd0-4863-93b7-33eea61087a3"
---

# Objetivos — ola A1 accept-pr-revoked-registry-rehab-ppr203

## Objetivo

Rehabilitar el proceso `accept-pr` en instancia Cerbero/Radamanto tras re-revocación `abrupt_success_rate_drop` since `2026-08-27T12:31:30Z` (PPR #203). Solo **A1 Yunque Rúnico**. Motor anti-recurrencia = persist_ref hermano A2.

## Alcance

1. Eliminar `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Reset absoluto bucket raíz `accept-pr` (laudo #203, poda `samples`, `structure_valid: true`).
3. Laterales Cerbero intactos (`refactorization`).
4. Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats `healthy`; `recovery_attempts: 0`; laudo `PBI-PPR-203-ACCEPT-PR-REVOKED-REGISTRY`; samples podados; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia ausente del diff del PR. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Cascada A1 bajo este `persist_ref`. |

## Fuera de alcance

- Mutación motor `execute-process` (ola A2).
- Rehab `refactorization` (PPR #186).
- Residual Kalma2 Shell/`git-manager` (PPR #136).
- Reabrir A1 `emit-pr-audited-event` #202.
- Mutar `radamanto.thresholds.json` v1.1.0.
- Escribir semillas bajo `docs/todos/`.

## Restricciones

- Git solo `skill:git-manager`. Rama `refactor/accept-pr-revoked-registry-rehab-ppr203`.
- Orden host: A2 aplicado **antes** de A1.
- Cuerpo = `refined_requirements` Dedalo A1.

## Ley aplicada

- `features-documentation-pattern` v1.2.x.
- Vehículo `feature` + `process_label: refactorization` (CLI).
- Jurisprudencia `L-REHAB-INST` (#174+#177, #185, #187, #194, #200).
- `SddIA/norms/external-ai-constraints.md`.
