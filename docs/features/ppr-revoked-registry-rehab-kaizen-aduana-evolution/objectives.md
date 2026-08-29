---
feature_name: ppr-revoked-registry-rehab-kaizen-aduana-evolution
created: "2026-08-29"
process: refactorization
branch_name: refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution
persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
pbi_ref: docs/todos/pending/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
document_id: PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY
uuid: c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
feature_ref: docs/fixes/kaizen-aduana-evolution-local
parent_pbi: docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #190).md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — re-revoked post-rehab #190 since 2026-08-28T10:10:42Z"
ola: A1
olas:
  - A1
  - A2
runtime_execution_id: "aa0d1244-043a-421f-9b60-efb76c4985ca"
---

# Objetivos — ppr-revoked-registry-rehab-kaizen-aduana-evolution

## Objetivo

Rehabilitar el proceso `pull-request-review` en instancia Cerbero/Radamanto tras re-revocación `success_rate_below_threshold` since `2026-08-28T10:10:42Z` (Cosecha `kaizen-aduana-evolution-local`), post-cierre #190. Causa raíz: samples no podadas (rate 0.25). A1 innegociable. A2 (poda gobernanza en Radamanto) condicionada a laudo.

## Alcance

1. Eliminar `revoked.pull-request-review`. Assert `permanent.pull-request-review` ausente.
2. Reset absoluto bucket raíz `pull-request-review` (laudo este `document_id`, poda `samples`, `structure_valid: true`, `recovery_attempts: 0`).
3. Laterales Cerbero intactos (`bug-fix`, `refactorization`).
4. Evidencia A1 en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff.
5. A2 (si laudo): `is_survival_hollow` poda denegación de gobernanza (`FAIL_F4_RBAC` / entidad revocada). Tests `t_a2_hollow_*`. Sin umbrales ni YAML PPR.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1-CERBERO | `pull-request-review` ∉ `revoked` ni `permanent`; laterales intactos. |
| AC-A1-SAMPLES | `samples: []` (**L-SAMPLES**). |
| AC-A1-LAUDO | `rehab_laudo` = este `document_id`; residuales #190 eliminados. |
| AC-A1-REDEEM | `healthy` · `structure_valid: true` · `recovery_attempts: 0` · `degraded_at: null`. |
| AC-A1-SMOKE | PPR post-rehab sin re-revocación inmediata; `execution_id` en `execution.md`. |
| AC-A2-HOLLOW | Denegación de gobernanza no degrada `success_rate` **o** PBI hijo abierto. |
| AC-A2-TESTS | `cargo test -p execute-process --lib` verde; podas preexistentes intactas. |
| AC-GIT-CLEAN | Diff PR sin `.SddIA/cerbero/**` ni `.SddIA/radamanto/**`. |
| AC-NO-THRESH | `radamanto.thresholds.json` sin modificar. |
| AC-DOC | Cascada + evolution UUID `c4e8f1a2-9b3d-4f7e-a6c1-2d8e5f0b3a71`. |

## Fuera de alcance

- Rehab `refactorization` (done #186) y `bug-fix` (episodio nuevo @ `16:18:17Z`, seed propia).
- Residual Kalma2 Shell/`git-manager` (PPR #136).
- Mutar umbrales v1.1.0 o YAML `pull-request-review.md`.
- Merge/Handoff del ciclo `kaizen-aduana-evolution-local`.
- Ejecutar T1–T5 en la sesión de planning.

## Restricciones

- Git solo `skill:git-manager`. Rama `refactor/ppr-revoked-registry-rehab-kaizen-aduana-evolution`.
- Init lab: vehículo `feature` + `process_label: refactorization`. `execution_id` `aa0d1244-043a-421f-9b60-efb76c4985ca`.
- Cuerpo = `refined_requirements` Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.1.
- Vehículo `feature` + `process_label: refactorization` (CLI).
- Jurisprudencia `L-REHAB-INST` / `L-SAMPLES` (#190 fallo; #208/#210 éxito).
- `SddIA/norms/external-ai-constraints.md` (DA-5; DA-2 no aplica a instancia ni a crate engine).
