---
feature_name: ppr-revoked-registry-rehab-restore-kaizen-ci-step
created: "2026-09-05"
process: refactorization
phase: design
agents: dedalo
base: main
scope: rehab-ppr-cerbero-a1-instance-only
branch_name: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
pbi_ref: docs/todos/pending/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
version_spec: "1.0.0"
status: dedalo_locked
ola: A1
olas:
  - A1
source_correlation_id: "AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc"
parent_pbi: docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — abrupt_success_rate_drop since 2026-08-29T05:01:52Z"
runtime_execution_id: "4fe5d41e-5ebb-430c-96c9-3f3a31b0103b"
---

# Spec — ppr-revoked-registry-rehab-restore-kaizen-ci-step

## 1. Misión técnica

Saneamiento de instancia `pull-request-review` (Yunque A1). Sin motor. Sin umbrales.

## 2. Diagnóstico (corte 2026-09-05)

| Vector | Hecho |
|--------|--------|
| Cerbero | `revoked.pull-request-review` since `2026-08-29T05:01:52Z` · `abrupt_success_rate_drop`. `permanent` vacío. |
| Stats | `degraded` · `structure_valid: false` · `recovery_attempts: 1` · laudo fósil kaizen-aduana · `rehabilitated_at: 2026-08-29T04:47:57Z` · 20 samples (10/10) · rate **0.50**. |
| Disparo | n=3 (801847/0, 743/1, 851/1) · rate 0.33 < 0.70 · `abrupt_drop_min_samples = 3`. |
| A2 | Done PR #221 `5024a022` @ `10:48:06Z`; instancia no tocada. |
| H10 | FIFO post-revoke mixto (8× exit 0 largos). Sightings 2026-09-01..04: F4→F5 handoff blocked. |
| H6 | `feature` y `delivery-close-cycle` ∈ revoked. `validate_di_rbac` = proveedores, no `process_name`. |

## 3. Laudos Dedalo

| Ref | Decisión |
|-----|----------|
| **L-REHAB-INST** | Locus `radamanto.revoked_entities` / `radamanto.stats`. Fuera del diff git. |
| **L-CERBERO** | DELETE `revoked.pull-request-review`. |
| **L-RESET-ABS** | Bucket raíz según contrato A1 + laudo este `document_id`. |
| **L-SAMPLES** | `samples: []`. |
| **L-ONTOLOGY** | `entity_type: process`. |
| **L-LATERAL** | Cinco laterales intactos. |
| **L-NO-THRESH** | `radamanto.thresholds.json` prohibido. |
| **L-NO-A2** | Sin `radamanto_batch_core.rs` / YAML PPR / `phase_terminal.rs`. |
| **L-VEHICLE-DUAL** | DCC `source_process: feature` / `process_label: refactorization`. |
| **L-TWO-REGIMES** | No afirmar «F4 aborta el 100 % de runs». |
| **L-CI** | CA6 sin run verde ⇒ `PENDIENTE-CI`. |
| **L-DOC** | Cascada este persist_ref. Evolution UUID ciclo. |

## 4. Contrato A1

```text
DELETE revoked["pull-request-review"]
ASSERT permanent["pull-request-review"] absent
ASSERT revoked[{bug-fix, delivery-close-cycle, entity-manager, feature, refactorization}] unchanged
stats["pull-request-review"] := {
  status: healthy,
  recovery_attempts: 0,
  consecutive_success_count: 0,
  degraded_at: null,
  entity_type: process,
  structure_valid: true,
  rehab_laudo: "PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY",
  rehabilitated_at: <ISO A1>,
  samples: []
}
```

## 5. Smoke

`./sddia-run.sh --process pull-request-review` + `SDDIA_AGENT_RELAY_IDE=1` + `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=1`. Éxito = acuse JSON `success` + `data.detached: true`. Lectura Cerbero inmediata. Prohibido join (DA-5).

## 6. Documental Git

`clarify.md` `objectives.md` `spec.md` `plan.md` `implementation.md` `execution.md` `validacion.md` + `SddIA/evolution/e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17.md`. PBI en `done/` en la rama del PR tras CA6 verde.
