---
feature_name: argos-validation-on-main
created: "2026-10-23"
updated: "2026-10-23T08:49:00Z"
process: bug-fix
phase: Verificación
agent: argos
branch: fix/argos-validation-on-main
branch_name: fix/argos-validation-on-main
persist_ref: docs/fixes/argos-validation-on-main
pbi_ref: docs/todos/done/[FIX] Argos sella validacion en main y bloquea el init.md
document_id: PBI-FIX-ARGOS-VALIDATION-ON-MAIN
uuid: "f126631d-3d98-43c4-b251-911410e06001"
execution_id: "a0509a75-3e2f-4a4d-ae64-18379411a5cb"
global: APTO
pbi_archived: true
checks:
  FX_CA1: APTO
  FX_CA2: APTO
  FX_CA3: APTO
  FX_CA4: APTO
  FX_CA5: APTO
  CARGO_TEST_VERIFICATION_HANDOFF: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - docs/fixes/argos-validation-on-main/
  - docs/todos/done/[FIX] Argos sella validacion en main y bloquea el init.md
---

# Validación — argos-validation-on-main

**global: APTO**. PBI archivado en `docs/todos/done/`.

`cargo test -p execute-process --lib` filtrado a `verification_handoff`, `verification_missing`, `runtime_evidence_forwards` y `branch_name_coalesces`: 5 passed, 0 failed (2026-10-23, exit 0).

| CA | Evidencia |
|----|-----------|
| FX-CA1 | Sin `branch` o sin `execution_id` la fase falla con `verification-handoff-incomplete` y no hace spawn. |
| FX-CA2 | Ref local ausente: `git-manager` `checkout` antes del spawn; fallo de checkout no arranca Argos. |
| FX-CA3 | El sello sucio de `docs/features/tracker-lifecycle-sink/validacion.md` se restauró a HEAD antes del checkout de esta rama. |
| FX-CA4 | Mensaje `dirty-worktree` de `workspace_init.rs` sin cambio de contrato. |
| FX-CA5 | T20 y `SddIA_AP` no se tocan. |
