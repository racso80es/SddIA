---

feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
process: feature
base: main
scope: execute-process/project_binding + handlers/forge_hu + .gitignore + fracture-corpus
execution_id: "17427fd2-d7dd-423f-8b97-33548496889c"
---
# Especificación técnica — aut-97-project-git-invalid-lab-prefix

## 1. `SddIA/engine/execute-process/src/engine/project_binding.rs`

En `core_nested_index_exempt(other: &Path, core_root: &Path) -> bool`:
- Extender la comprobación del primer componente normal `s` para aceptar también:
  - `s.starts_with("_forge_hu_")`
  - `s.starts_with("_linear_e2e_")`
  junto a `_refine_lab_` y `_forge_lab_`.
- Añadir test unitario `bind_core_tolerates_absolute_forge_hu_and_linear_e2e_labs_in_index` verificando que `bind(repo, {"project_slug": "core"})` tiene éxito cuando `.SddIA/projects/` contiene índices absolutos hacia `_forge_hu_forge-hu-ca5` y `_linear_e2e_lab`.

## 2. `SddIA/engine/execute-process/src/engine/handlers/forge_hu.rs`

En `mod tests`:
- Introducir un guard RAII `LabProjectGuard` (`impl Drop for LabProjectGuard`) que ejecute `cleanup(&self.core, &self.slug)` al salir del ámbito, garantizando la eliminación de `.SddIA/projects/{slug}.md` y `_forge_hu_{slug}` incluso ante un panic en el test.
- Invocar `cleanup` también al construir el guard antes de registrar para limpiar cualquier residuo previo.

## 3. `.gitignore`

Bajo `# Artefactos de laboratorio refine/forge (contaminación contextual Glob)`:
```gitignore
/_forge_hu_*/
/_linear_e2e_*/
```

## 4. Fixture `fracture-corpus`

Crear `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/e76035db-project-git-invalid-lab-prefix.json` documentando el incidente `AUT-97` (`trace_hash: e76035db4846`, `unclassified`).
