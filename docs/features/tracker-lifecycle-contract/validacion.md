---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
branch: feat/tracker-lifecycle-contract
branch_name: feat/tracker-lifecycle-contract
persist_ref: docs/features/tracker-lifecycle-contract
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-01 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
uuid: "1b17e6fa-a8f3-4ce3-91a4-7aff1954a910"
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
global: APTO
pbi_archived: true
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/library/norms/capability-taxonomy.md
  - SddIA/library/norms/index.md
  - SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json
  - SddIA/core/capability-bindings.md
  - SddIA/core/eda-coverage.json
  - SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs
  - SddIA/evolution/2c882ecf-0c03-48a9-9259-8de9a15a2a74.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/tracker-lifecycle-contract/objectives.md
  - docs/features/tracker-lifecycle-contract/clarify.md
  - docs/features/tracker-lifecycle-contract/spec.md
  - docs/features/tracker-lifecycle-contract/plan.md
  - docs/features/tracker-lifecycle-contract/implementation.md
  - docs/features/tracker-lifecycle-contract/execution.md
  - docs/features/tracker-lifecycle-contract/validacion.md
  - docs/todos/done/[ARQUITECTURA] HU-003 PBI-01 — tracker lifecycle.md
---

# Validación — tracker-lifecycle-contract

AC-1: el catálogo contiene `tracker:lifecycle`. `validate_phase_capability_di` sobre una fase que lo requiere, sin delegate, responde `CapabilityProviderMismatch`. No responde `CapabilityNotIndexed`.

AC-2: `ac_tracker_lifecycle_emit_v2_body` pasó. El cuerpo de éxito de `emit_v2` valida. Un objeto sin `success` y `exitCode` es `SchemaMismatch`. El fallo real de `emit_v2` (con ambos campos) también valida.

AC-3: una fila `tracker:lifecycle` en `capability-bindings.md` v1.7.0. Cero `capability_id` repetidos.

El sello `Domain_Entity_Updated` de esta norma ya existía (`5505e7ef-a933-4ad4-ade1-4e75607abf3e`). El motor lo trató como idempotente. La cobertura quedó en el hash `sha256:4cf09806f229ac1d4ca0c3131bb054a2db06c715bce28d549310a1d111e1ceab`.
