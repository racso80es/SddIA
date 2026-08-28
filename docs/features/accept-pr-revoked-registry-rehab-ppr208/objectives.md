---
feature_name: accept-pr-revoked-registry-rehab-ppr208
created: "2026-08-28"
process: refactorization
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr208
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr208
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY
uuid: d4f8e2a1-6c39-4b7e-9a05-1f3c8d7e6b20
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "4CMsk8z5Gx7mFQHc512m9FoJibvnr463cVyVcWz5imKm"
source_pr_url: https://github.com/racso80es/SddIA/pull/208
feature_ref: docs/fixes/kaizen-aduana-dlt-relay-supervisado
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #203).md
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — re-revoked post-#203 abrupt_success_rate_drop since 2026-08-28T18:21:13Z"
ola: A1
olas:
  - A1
runtime_execution_id: "e1de4691-5b6f-495b-85ff-b6a52dcd11c4"
---

# Objetivos — ola A1 accept-pr-revoked-registry-rehab-ppr208

## Objetivo

Rehabilitar el proceso `accept-pr` en instancia Cerbero/Radamanto tras re-revocación `abrupt_success_rate_drop` since `2026-08-28T18:21:13Z` (PPR #210), post-cierre #203. Solo **A1 Yunque Rúnico**.

## Alcance

1. Eliminar `revoked.accept-pr`. Assert `permanent.accept-pr` ausente.
2. Reset absoluto bucket raíz `accept-pr` (laudo #210, poda `samples`, `structure_valid: true`).
3. Laterales Cerbero intactos (`accept-pr`, `feature`, `refactorization`).
4. Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats `healthy`; `recovery_attempts: 0`; laudo `PBI-PPR-208-ACCEPT-PR-REVOKED-REGISTRY`; samples `[]`; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia ausente del diff del PR. |
| AC-ONTO | `entity_type: process`. |
| AC-DOC | Cascada A1 bajo este `persist_ref`. |

## Fuera de alcance

- Mutación motor `execute-process`.
- Rehab `feature` / `accept-pr` / `refactorization` (olas hermanas / done #186).
- Residual Kalma2 Shell/`git-manager` (PPR #136).
- Mutar `radamanto.thresholds.json` v1.1.0.
- Escribir semillas bajo `docs/todos/`.
- Ejecutar T1–T5 en la sesión de planning.

## Restricciones

- Git solo `skill:git-manager`. Rama `refactor/accept-pr-revoked-registry-rehab-ppr208`.
- Init lab: vehículo `feature` + `process_label: refactorization` (CLI).
- Cuerpo = `refined_requirements` Dedalo A1.

## Ley aplicada

- `features-documentation-pattern` v1.2.x.
- Vehículo `feature` + `process_label: refactorization` (CLI).
- Jurisprudencia `L-REHAB-INST` (#174+#177, #185, #187, #203, #200, #203).
- `SddIA/norms/external-ai-constraints.md`.
