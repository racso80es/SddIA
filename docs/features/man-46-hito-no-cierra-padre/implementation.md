---
feature_name: man-46-hito-no-cierra-padre
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Registro de implementación — aduana parental_close_gate (MAN-46 / KZ-3)
branch_name: feat/man-46-hito-no-cierra-padre
persist_ref: docs/features/man-46-hito-no-cierra-padre
pbi_ref: MAN-46
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-74cd5db0
agent: tekton
phase: ejecucion
status: complete
norm_ref: features-documentation-pattern@1.3.2
execution_id: "cbf032fc-8bbf-452e-b124-a5266b7d3fc4"
---
# Implementación — man-46-hito-no-cierra-padre

## Touchpoints de código y norma

| Archivo | Cambio | Propósito |
|---------|--------|-----------|
| `SddIA/engine/execute-process/src/engine/parental_close_gate.rs` | Nuevo módulo | `assert_parental_close_allowed`, `ParentalCloseFault`, `ParentalCloseEvidence`; valida AC abiertos en `objectives.md`, `delivery_state: success`, narración «ciclo cerrado», tokens de hito (`H*-only`, `*_deferred`, `PASS_*_MILESTONE`) y detector de `refine` sellado previo (`scope_refine_proof.md`); tests `ka_mile_1`, `ka_mile_2`, `ka_mile_3`, `resolve_persist_by_pbi_ref`, `parental_close_lab_skip`. |
| `SddIA/engine/execute-process/src/engine/mod.rs` | `pub mod parental_close_gate;` | Registro del módulo en `engine`. |
| `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs` | Guarda en `maybe_complete_hu_when_children_done` | Invoca `assert_parental_close_allowed(..., rollup)` antes de mover `hu_ref` a `done`. |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | Guarda en `capsule_diff_seal_validacion` | Invoca `assert_parental_close_allowed(..., success_seal)` tras `apto_coherence`; devuelve `status: blocked` con `friction_id` y suprime `System_Fracture_Detected`. |
| `SddIA/engine/execute-process/src/engine/diff_seal.rs` | Guarda en `validate_for_accept_pr_merge` | Invoca `assert_parental_close_allowed(..., success_seal)` antes de habilitar fusión en `accept-pr`. |
| `SddIA/library/codexes/codex-software-engineering/process/feature.md` | Nota normativa hito ≠ Done | Documenta explícitamente que un hito de plan se entrega como ítem hijo y no cierra el padre. |
