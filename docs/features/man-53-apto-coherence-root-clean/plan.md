---

feature_name: man-53-apto-coherence-root-clean
created: "2026-11-22"
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
phases:
  - id: H1
    name: "Regla C5 y veto de checks troncales en apto_coherence.rs + tests"
  - id: H2
    name: "Aduana coherencia en pull-request-review / qa_attestation / residual_runner"
  - id: H3
    name: "Check físico de directorios efímeros en raíz en delivery-close-cycle y accept-pr"
  - id: H4
    name: "Verificación cargo test / clippy, rebuild binarios y documentación de cierre"
execution_id: "a3971880-ee8b-49c3-97d4-40f61477aa47"
---
# Plan de Implementación — man-53-apto-coherence-root-clean

## H1 — `apto_coherence.rs` (C5 + Trunk Gates)

1. Implementar `check_c5_ac_matrix` y `check_trunk_gates` en `SddIA/engine/execute-process/src/engine/apto_coherence.rs`.
2. Añadir tests unitarios `ka_ac_partial_1_*`, `ka_gate_trunk_2_*` y `ka_close_4_man50_counterfactual`.

## H2 — `pull-request-review` + `qa_attestation` + `residual_runner`

1. Integrar `validate_apto_coherence` en `pull_request_review.rs` (`handoff_accept_pr`), `qa_attestation.rs` (`try_write_from_ppr`) y `residual_runner.rs` (post-veredicto PPR).
2. Añadir tests unitarios en `pull_request_review.rs` y `qa_attestation.rs`.

## H3 — Check físico de raíz en `delivery-close-cycle` y `accept-pr`

1. Implementar `root_ephemeral_dir_violations` en `qa_attestation.rs`.
2. Integrar en `delivery_close.rs` (`capsule_gate_pre_pr_qa`) y `accept_pr.rs` (`Auditoría Genómica` / `Fusión Soberana`).
3. Añadir tests unitarios `ka_root_phys_3_*`.

## H4 — Verificación y Cierre

1. Ejecutar `cargo test -p execute-process` y `cargo clippy -p execute-process -- -D warnings`.
2. Recompilar `execute-process` (`cargo build --release -p execute-process`) si aplica para paridad runtime.
3. Completar `implementation.md`, `execution.md`, `validacion.md` y ejecutar `delivery-close-cycle`.
