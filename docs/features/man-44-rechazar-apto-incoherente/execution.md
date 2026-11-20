---
feature_name: man-44-rechazar-apto-incoherente
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Evidencia de ejecución — aduana de coherencia documental APTO (MAN-44 / KZ-1)
branch_name: feat/man-44-rechazar-apto-incoherente
persist_ref: docs/features/man-44-rechazar-apto-incoherente
pbi_ref: MAN-44
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-8205539f
agent: tekton
phase: ejecucion
status: complete
verdict: ok
norm_ref: features-documentation-pattern@1.3.1
execution_id: "e049f4db-4e0e-41d0-b651-e9b9de645f71"
phases_closed:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
---
# Ejecución — man-44-rechazar-apto-incoherente

## Estado de fases (`plan.md`)

| Fase | Estado | Evidencia |
|------|--------|-----------|
| **H0** | `closed` | Rama `feat/man-44-rechazar-apto-incoherente` aislada; huecos pre-sello (`delivery_close.rs`) y pre-fusión (`diff_seal.rs` / `accept_pr.rs`) inventariados en `implementation.md`. |
| **H1** | `closed` | `SddIA/engine/execute-process/src/engine/apto_coherence.rs` creado y registrado en `engine/mod.rs`; C2, C3, C4 y `SDDIA_LAB_SKIP_APTO_COHERENCE` implementados. |
| **H2** | `closed` | Puerta pre-sello integrada en `delivery_close::capsule_diff_seal_validacion` con supresión de `System_Fracture_Detected` y `friction_id` accionable; `delivery-close-cycle.md` actualizado y con `hash_signature` sincronizado. |
| **H3** | `closed` | Puerta pre-fusión integrada en `diff_seal::validate_for_accept_pr_merge` (invocada por `accept_pr` en `Fusión Soberana`); `accept-pr.md` actualizado y con `hash_signature` sincronizado. |
| **H4** | `closed` | `features-documentation-pattern.md` actualizado; fixtures `_fixtures/ka-close-{1,2,3}/` materializados; contrafactual KA-CLOSE-4 documentado abajo. |
| **H5** | `closed` | Tests unitarios, `clippy` y `verify-process-integrity` en verde tras la última edición. |

## Verificación ejecutada

- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib apto_coherence`: **7 passed; 0 failed** (`apto_coherence_lab_skip`, `apto_coherence_noop_without_apto`, `coherent_apto_passes`, `ka_close_1_execution_blocked_rejects_apto`, `ka_close_2_open_phases_rejects_apto`, `ka_close_3_resolution_denylist_rejects_apto`, `dcc_fracture_suppressed_on_apto_coherence_block`).
- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib diff_seal`: **14 passed; 0 failed**.
- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib delivery_close::tests`: **38 passed; 0 failed**.
- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib accept_pr`: **21 passed; 0 failed**.
- `./sddia-run.sh --verify-process-integrity`: **`verify-process-integrity: OK`**.
- `cargo clippy --manifest-path SddIA/Cargo.toml -p execute-process -- -D warnings`: **0 errors, 0 warnings**.

## Contrafactual documentado (KA-CLOSE-4)

Origen: auditoría `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-1). En el cierre del epílogo de **MAN-43** (PR #453, sin revertir histórico):
1. `execution.md` presentaba `verdict: blocked` / `status: partial` (C2) y `plan.md` declaraba fases H2–H6 sin cierre en `execution.md` (C3).
2. `validacion.md` elevaba `global: APTO` apoyándose en `resolution: PASS_H1_MILESTONE` (C4, fuera del patrón documental).

Con la aduana `apto_coherence::validate_apto_coherence` activa tanto en `delivery-close-cycle` (`capsule_diff_seal_validacion`) como en `accept-pr` (`validate_for_accept_pr_merge`), **cualquiera** de las tres condiciones (`apto_coherence_execution_blocked`, `apto_coherence_open_phases`, `apto_coherence_resolution`) habría bloqueado el sello de diff y abortado la fusión del PR #453 con fricción accionable y sin emitir `System_Fracture_Detected`.
