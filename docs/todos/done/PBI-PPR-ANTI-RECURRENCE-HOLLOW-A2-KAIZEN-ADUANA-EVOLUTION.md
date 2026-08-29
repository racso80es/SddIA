---
document_id: PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION
title: "[ARQUITECTURA] pull-request-review — poda anti-recurrencia Radamanto (ola A2 kaizen-aduana-evolution)"
format: markdown
version: "1.0.0"
created: "2026-08-29"
updated: "2026-08-29T04:54:57Z"
status: done
refinement_status: implemented
pbi_archived: true
priority: alta
process: refactorization
executor_vehicle: feature
type: refactorization
dispatch: false
uuid: 18bacf31-9223-4b07-853e-a66c0d6c3ebd
entity: pull-request-review
entity_type: process
ola: A2
olas:
  - A2
suggested_branch: refactor/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
persist_ref: docs/features/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
branch_name: refactor/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution
parent_pbi: docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
parent_persist_ref: docs/features/ppr-revoked-registry-rehab-kaizen-aduana-evolution
source_correlation_id: "8ZjTzcBwfFAVFQujfjGCJwJeJcj5pbB4SMHAD5bn5ybE"
feature_ref: docs/fixes/kaizen-aduana-evolution-local
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — bucle autoconfirmante hipotético (KO tempranos 636–1301 ms); A1 cerrado PR #220; A2 contract-first L-A2-T0 inconcluso"
blocked_by:
  - "refactorization ∈ revoked since 2026-08-20T05:48:56Z → vehículo DCC = feature"
related:
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - docs/features/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution/spec.md
  - docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
---

# [ARQUITECTURA] pull-request-review — poda anti-recurrencia Radamanto (ola A2)

## Mandato

Cerrar la ola **A2** diferida del ciclo `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY`: extender `is_survival_hollow()` para que un aborto **`CERBERO_ENTITY_REVOKED` auto-referencial** no degrade `success_rate`, sin silenciar violaciones legítimas (`CERBERO_RBAC_DENIED`, `CERBERO_CONFIG_ERROR`).

A1 (rehab instancia) **done** @ PR #220 merge `c1007a51`. Este PBI solo toca genoma motor + documentación; **sin** mutación `.SddIA/cerbero/` ni `.SddIA/radamanto/`.

## Genealogía

| Episodio | Estado |
|----------|--------|
| Parent A1 `c4e8f1a2-…` | **done** — rehab + samples podados |
| **Este PBI** `18bacf31-…` | **done** — motor A2 |

## Alcance A2

1. **T0:** documentar confirmación empírica de `failed_phase_code` en muestras KO históricas (eventos purgados post-A1). Si inconcluso, implementación contract-first con laudo explícito.
2. `radamanto_batch_core.rs`: poda solo cuando `failed_phase_code == CERBERO_ENTITY_REVOKED` **y** `revoked_provider(failed_phase_error) == target_entity_from_payload(payload)`.
3. Tests: `t_a2_hollow_entity_revoked_self`, `t_a2_hollow_rbac_denied_not_podado`, `t_a2_hollow_revoked_other_provider_not_podado`; podas preexistentes intactas.
4. **Prohibido:** `phase_terminal.rs`, `radamanto.thresholds.json`, YAML `pull-request-review.md`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A2-DISCRIM | Solo `CERBERO_ENTITY_REVOKED` auto-referencial; RBAC/CONFIG nunca podados |
| AC-A2-TESTS | `cargo test -p execute-process --lib` verde; `t_a2_hollow_*` |
| AC-GIT-CLEAN | Diff sin instancia Cerbero/Radamanto |
| AC-NO-THRESH | Sin umbrales |
| AC-DOC | persist_ref + evolution UUID `18bacf31-9223-4b07-853e-a66c0d6c3ebd` |

## Laudos heredados

- **L-A2-HOLLOW** / **L-A2-NO-BLIND** / **L-A2-T0** — ver `docs/features/ppr-anti-recurrence-hollow-a2-kaizen-aduana-evolution/spec.md`
- **L-VEHICLE:** DCC `source_process: feature` + `process_label: refactorization`
