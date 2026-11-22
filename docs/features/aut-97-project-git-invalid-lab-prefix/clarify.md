---
feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
process: feature
pbi_ref: AUT-97
execution_id: "cb6b60ee-cc70-49ea-a4e2-55d7d0ca15c7"
---
# Clarificación — aut-97-project-git-invalid-lab-prefix

Refinamiento previo sellado en `refine-item` (`execution_id: 095d7930-53d4-4c85-b0ef-7748fd49f85b`, `verdict: refined`, `open_questions: []`).

## Diagnóstico empírico de la fractura `e76035db4846`

1. `SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs` (`register_lab_project`) crea proyectos efímeros en `core.join(format!("_forge_hu_{slug}"))` y registra `.SddIA/projects/{slug}.md` con `project_root` absoluto.
2. `SddIA/engine/execute-process/src/engine/linear_direct_cycle_e2e.rs` (`register_project`) crea proyectos efímeros con prefijo `_linear_e2e_{slug}`.
3. `core_nested_index_exempt` en `SddIA/engine/execute-process/src/engine/project_binding.rs` solo eximía `_refine_lab_` y `_forge_lab_` (y `docs/_fixtures`), provocando `PROJECT_GIT_INVALID` en `assert_git_isolation_for_slug` cuando `bind` se ejecuta sobre `core` mientras existe o quedó un residuo `_forge_hu_*` o `_linear_e2e_*`.
4. `.gitignore` tampoco listaba `/_forge_hu_*/` ni `/_linear_e2e_*/`.
