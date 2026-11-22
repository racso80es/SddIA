---
feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
process: feature
phases:
  - id: T1
    name: Eximir _forge_hu_ y _linear_e2e_ en project_binding.rs y .gitignore
  - id: T2
    name: Blindar limpieza RAII en forge_hu.rs, añadir test de regresión y fixture fracture-corpus e76035db4846
  - id: T3
    name: Compilar execute-process, ejecutar tests y cerrar validación
execution_id: "cb6b60ee-cc70-49ea-a4e2-55d7d0ca15c7"
---
# Plan de implementación — aut-97-project-git-invalid-lab-prefix

1. **T1:** Editar `SddIA/engine/execute-process/src/engine/project_binding.rs` (`core_nested_index_exempt`) y `.gitignore` para eximir e ignorar `_forge_hu_*` y `_linear_e2e_*`.
2. **T2:** Añadir `LabProjectGuard` (`Drop`) en `SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs`, añadir test unitario `bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index` en `project_binding.rs` y crear `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json`.
3. **T3:** Ejecutar `cargo test --manifest-path SddIA/Cargo.toml -p execute-process`, compilar `cargo build --release --manifest-path SddIA/Cargo.toml -p execute-process` y registrar `implementation.md`, `execution.md` y `validacion.md`.
