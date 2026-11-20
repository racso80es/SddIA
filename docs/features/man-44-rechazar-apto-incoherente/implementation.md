---


feature_name: man-44-rechazar-apto-incoherente
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Registro de implementación — aduana de coherencia documental APTO (MAN-44 / KZ-1)
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-8205539f
agent: tekton
phase: ejecucion
status: complete
norm_ref: features-documentation-pattern@1.3.1
execution_id: "a9098555-c861-40cc-8b13-c6871caf57e9"
---
# Implementación — man-44-rechazar-apto-incoherente

## Touchpoints de código

| Archivo | Cambio | Propósito |
|---------|--------|-----------|
| `SddIA/engine/execute-process/src/engine/apto_coherence.rs` | Nuevo módulo | `validate_apto_coherence` y `CoherenceFault`; reglas C2 (`execution.md` blocked/partial/ausente), C3 (fases de `plan.md` abiertas sin cierre en `execution.md`), C4 (denylist de `resolution` no canónica y elevadores diferidos); tests `ka_close_1`, `ka_close_2`, `ka_close_3`, `coherent_apto_passes`, `apto_coherence_noop_without_apto`, `apto_coherence_lab_skip`. |
| `SddIA/engine/execute-process/src/engine/mod.rs` | `pub mod apto_coherence;` | Exposición del módulo compartido en `engine`. |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | Puerta pre-sello en `capsule_diff_seal_validacion` | Invoca `apto_coherence::validate_apto_coherence` antes de `apply_diff_seal_to_validacion`; devuelve `status: blocked` con `friction_id` (`apto_coherence_*`) y `operator_hint` sin emitir `System_Fracture_Detected`. Test `dcc_fracture_suppressed_on_apto_coherence_block`. |
| `SddIA/engine/execute-process/src/engine/diff_seal.rs` | Puerta pre-fusión en `validate_for_accept_pr_merge` | Invoca `apto_coherence::validate_apto_coherence` tras `verify_diff_seal_at_head` cuando `validacion.md` declara `global: APTO`. Fixture de `aut53_scoped_persist_ref_ignores_sibling_fixture_apto_without_seal` actualizado con `execution.md` válido. |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | Consumo en `Fusión Soberana` | Aborta antes de `checkout`/`merge` si `validate_for_accept_pr_merge` retorna error de coherencia APTO. |

## Touchpoints normativos y documentales

| Archivo | Cambio |
|---------|--------|
| `SddIA/library/norms/features-documentation-pattern.md` | Regla explícita de coherencia APTO y prohibición de resoluciones no enumeradas para elevar `global: APTO`. |
| `SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md` | Documentación de puerta pre-sello de coherencia APTO y `hash_signature` actualizado. |
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | Documentación de puerta pre-fusión de coherencia APTO y `hash_signature` actualizado. |
| `docs/features/man-44-rechazar-apto-incoherente/_fixtures/ka-close-{1,2,3}/` | Fixtures reproducibles (`validacion.md`, `execution.md`, `plan.md`) para KA-CLOSE-1..3. |
