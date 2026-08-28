---
feature_name: bug-fix-revoked-registry-rehab-ppr210
created: "2026-08-28"
process: refactorization
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr210
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr210
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #210).md
document_id: PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY
uuid: e7a1b2c3-4d5e-6f78-9a0b-1c2d3e4f5a6b
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "4c2dfd1d-393d-4411-8956-d596ff0eef9c"
source_pr_url: https://github.com/racso80es/SddIA/pull/210
feature_ref: docs/fixes/route-domain-event-fracture-6a49e0ad
parent_pbi: docs/todos/done/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — re-revoked post-#194 abrupt_success_rate_drop since 2026-08-28T05:32:55Z"
ola: A1
olas:
  - A1
runtime_execution_id: "243b6790-ee2a-42f8-8869-4fbf17a3c16b"
---

# Objetivos — ola A1 bug-fix-revoked-registry-rehab-ppr210

## Objetivo

Rehabilitar el proceso `bug-fix` en instancia Cerbero/Radamanto tras re-revocación `abrupt_success_rate_drop` since `2026-08-28T05:32:55Z` (PPR #210), post-cierre #194. Solo **A1 Yunque Rúnico**.

## Alcance

1. Eliminar `revoked.bug-fix`. Assert `permanent.bug-fix` ausente.
2. Reset absoluto bucket raíz `bug-fix` (laudo #210, poda `samples`, `structure_valid: true`).
3. Laterales Cerbero intactos (`accept-pr`, `feature`, `refactorization`).
4. Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `bug-fix` ∉ `revoked` ni `permanent`; stats `healthy`; `recovery_attempts: 0`; laudo `PBI-PPR-210-BUG-FIX-REVOKED-REGISTRY`; samples `[]`; evidencia A1 en `execution.md`. |
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

- Git solo `skill:git-manager`. Rama `refactor/bug-fix-revoked-registry-rehab-ppr210`.
- Init lab: vehículo `feature` + `process_label: refactorization` (CLI).
- Cuerpo = `refined_requirements` Dedalo A1.

## Ley aplicada

- `features-documentation-pattern` v1.2.x.
- Vehículo `feature` + `process_label: refactorization` (CLI).
- Jurisprudencia `L-REHAB-INST` (#174+#177, #185, #187, #194, #200, #203).
- `SddIA/norms/external-ai-constraints.md`.
