---
feature_name: ppr-revoked-registry-rehab-restore-kaizen-ci-step
created: "2026-09-05"
purpose: Estabilización Mayeuta — PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY v1.2.0
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
version_clarify: "1.0.0"
---

# Clarificación — ppr-revoked-registry-rehab-restore-kaizen-ci-step

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/DCC + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `4fe5d41e-5ebb-430c-96c9-3f3a31b0103b`. Rama `refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## D0 — Semilla v1.2.0

| Vector | Hecho |
|--------|--------|
| PBI | `PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY` · uuid `e2f8a1c4-…` · `refinement_status: refinado` · v1.2.0 |
| Ola | **A1** instancia. A2 motor **done** PR #221 — fuera de alcance (L-NO-A2). |
| Vehículo | `--process feature` + `process_label: refactorization` (**L-VEHICLE-DUAL**: ambos ∈ revoked; no gate F4 de este PBI). |
| Padre | `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY` done @ PR #220. |
| Sighting origen | Cosecha PR #247 · CID `AU1Azkr…` · `FAIL_F4_RBAC`. |

### Anti-alucinación (PBI §0)

H1–H5 (v1.1) y H6–H10 (v1.2) son laudos de Filtro A. No reabrir: rate 0.50 no 0.0; disparo n=3 no n=20; A2 no es causa; L-VEHICLE-DUAL; `rehabilitated_at` ≠ `mergedAt` #220; #190 merge 2026-08-25; ms enteros; FIFO mixto post-revoke (H10).

## D1 — Misión

Sanar `pull-request-review` en Cerbero/Radamanto para que `RBAC_PROCESS_REGISTRY` deje de nacer `NO_APTO` y el handoff a `accept-pr` deje de nacer `blocked` por peaje de proceso.

## D2 — Decisiones

| ID | Laudo |
|----|-------|
| L-REHAB-INST | Solo `.SddIA/cerbero/` y `.SddIA/radamanto/`. Fuera del diff. |
| L-SAMPLES | `samples: []` obligatorio. |
| L-LATERAL | Laterales Cerbero intactos. |
| L-VEHICLE-DUAL | Vehículo `feature`; no rehab `feature`/`DCC`/`refactorization`. |
| L-NO-THRESH | Umbrales prohibidos. |
| L-NO-A2 | Sin mutación motor. |
| L-TWO-REGIMES | FIFO mixto no niega el gate de handoff 2026-09-01..04. |
| L-CI | CA6: `validacion.md` `global: APTO` exige run CI verde. |

## Fuera

Rehab laterales; umbrales; YAML `pull-request-review.md`; `radamanto_batch_core.rs`; merge/`accept-pr` de este PR (handoff aparte).
