---
feature_name: man-45-accept-pr-ppr-tree-sha
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Evidencia de ejecución — aduana PPR tree_sha en accept-pr (MAN-45 / KZ-2)
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-75d12fd0
agent: tekton
phase: ejecucion
status: complete
verdict: ok
norm_ref: features-documentation-pattern@1.3.2
execution_id: "73910788-b7e7-4a61-9ac2-72fedd024b2a"
phases_closed:
  - H0
  - H1
  - H2
  - H3
  - H4
  - H5
---
# Ejecución — man-45-accept-pr-ppr-tree-sha

## Estado de fases (`plan.md`)

| Fase | Estado | Evidencia |
|------|--------|-----------|
| **H0** | `closed` | Rama `feat/man-45-accept-pr-ppr-tree-sha` aislada; huecos inventariados en `implementation.md`. |
| **H1** | `closed` | `qa_attestation.rs` actualizado con `issuer_process == "pull-request-review"`, `delta_class` (`active` vs `passive`), rechazo de `docs-only` en delta activo, rechazo de `skipped-ppr-dedupe`, y `validate_for_accept_pr_with_state`. |
| **H2** | `closed` | `accept_pr.rs` (`Auditoría Genómica`) invoca siempre la validación de atestación PPR (no solo bajo `merge_already_done`), abortando si falta o falla; `accept-pr.md` actualizado y con `hash_signature` sincronizado. |
| **H3** | `closed` | Fixtures `_fixtures/ka-ppr-{1,2,3,4}/` materializados y cubiertos por tests `ka_ppr_1..4` y `ka_ppr_1_genomic_audit_aborts_without_attestation`. |
| **H4** | `closed` | Contrafactual KA-PPR-5 documentado en `ka-ppr-5-counterfactual.md`. |
| **H5** | `closed` | `LAB_ENV_LOCK` añadido en `qa_attestation::tests` para ejecución paralela segura; tests unitarios, `clippy` y `verify-process-integrity` en verde. |

## Verificación ejecutada

- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib qa_attestation`: **15 passed; 0 failed** (en paralelo).
- `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib accept_pr`: **22 passed; 0 failed**.
- `cargo clippy --manifest-path SddIA/Cargo.toml -p execute-process -- -D warnings`: **0 errors, 0 warnings**.
- `./sddia-run.sh --verify-process-integrity`: **`verify-process-integrity: OK`**.
