---

feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
process: feature
items:
  - path: SddIA/engine/execute-process/src/engine/project_binding.rs
    action: update
    summary: Eximir _forge_hu_ y _linear_e2e_ en core_nested_index_exempt + test de regresión bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index
  - path: SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs
    action: update
    summary: Guard RAII LabProjectGuard (Drop) en mod tests para limpieza determinista de _forge_hu_{slug} y .SddIA/projects/{slug}.md
  - path: .gitignore
    action: update
    summary: Ignorar /_forge_hu_*/ y /_linear_e2e_*/
  - path: SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json
    action: create
    summary: Fixture fracture-corpus para traza e76035db4846 (AUT-97)
execution_id: "17427fd2-d7dd-423f-8b97-33548496889c"
---
# Implementación — aut-97-project-git-invalid-lab-prefix

## Touchpoints

- `SddIA/engine/execute-process/src/engine/project_binding.rs`:
  - `core_nested_index_exempt`: añadido soporte para los prefijos `_forge_hu_` y `_linear_e2e_` junto a `_refine_lab_` y `_forge_lab_`.
  - `tests::bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index`: verifica que `bind(repo, {"project_slug": "core"})` tolera índices lab absolutos anidados bajo `_forge_hu_*` y `_linear_e2e_*`.
- `SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs`:
  - `tests::LabProjectGuard`: guard RAII (`Drop`) que limpia `.SddIA/projects/{slug}.md` y `_forge_hu_{slug}` al entrar y salir del test.
- `.gitignore`:
  - Añadidas `/_forge_hu_*/` y `/_linear_e2e_*/`.
- `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json`:
  - Registro del incidente `e76035db4846` en el corpus de fracturas.
