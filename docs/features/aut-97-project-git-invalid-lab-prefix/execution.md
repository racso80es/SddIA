---
feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
process: feature
items_applied:
  - T1
  - T2
  - T3
execution_id: "cb6b60ee-cc70-49ea-a4e2-55d7d0ca15c7"
---
# Registro de ejecución — aut-97-project-git-invalid-lab-prefix

- **T1:** Exención de los prefijos `_forge_hu_` y `_linear_e2e_` en `SddIA/engine/execute-process/src/engine/project_binding.rs` (`core_nested_index_exempt`) e incorporación de `/_forge_hu_*/` y `/_linear_e2e_*/` en `.gitignore`.
- **T2:** Añadido guard RAII `LabProjectGuard` (`impl Drop`) en `SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs`, test unitario `bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index` en `project_binding.rs`, y fixture `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json`.
- **T3:** Ejecutados `cargo test --manifest-path SddIA/Cargo.toml -p execute-process --lib project_binding::tests` (`19 passed; 0 failed`), `handlers::forge_hu::tests` (`1 passed; 0 failed`), `RUSTFLAGS="-D warnings" cargo check --manifest-path SddIA/Cargo.toml -p execute-process --all-targets` (`0` warnings) y `cargo build --release --manifest-path SddIA/Cargo.toml -p execute-process`.
