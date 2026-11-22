---

feature_name: aut-97-project-git-invalid-lab-prefix
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Objetivos estabilizados — eliminar rechazo PROJECT_GIT_INVALID por prefijos de laboratorio (_forge_hu_*, _linear_e2e_*)
branch_name: feat/aut-97-project-git-invalid-lab-prefix
persist_ref: docs/features/aut-97-project-git-invalid-lab-prefix
pbi_ref: AUT-97
document_id: PBI-FIX-FRACTURE-e76035db4846
status: requisitos-estabilizados
execution_id: "17427fd2-d7dd-423f-8b97-33548496889c"
---
# Objetivos — aut-97-project-git-invalid-lab-prefix

## Misión

Eliminar el rechazo espurio `DOMAIN_AUTHORITY_DENIED: PROJECT_GIT_INVALID` en `delivery-close-cycle` (y cualquier proceso sobre `project_slug: "core"`) causado por directorios e índices efímeros de laboratorio (`_forge_hu_*` y `_linear_e2e_*`) no incluidos en `core_nested_index_exempt` (`project_binding.rs`), garantizando además su limpieza determinista e higiene en `.gitignore`.

Tracker **AUT-97** (documento `PBI-FIX-FRACTURE-e76035db4846`, `fracture_hash: e76035db4846`).

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **CA-1** | `core_nested_index_exempt` en `project_binding.rs` devuelve `true` para rutas bajo `core_root` cuyo primer componente normal comience por `_refine_lab_`, `_forge_lab_`, `_forge_hu_` o `_linear_e2e_`. | Test unitario en `project_binding.rs`. |
| **CA-2** | `forge_hu::tests` limpia `.SddIA/projects/{slug}.md` y `_forge_hu_{slug}` mediante guard RAII (`Drop`) al finalizar el test (incluso ante panic), y `.gitignore` incluye `/_forge_hu_*/` y `/_linear_e2e_*/`. | Inspección de `forge_hu.rs` y `.gitignore`. |
| **CA-3** | Test de regresión en `project_binding.rs` (`bind` con índice `_forge_hu_forge-hu-ca5` y `_linear_e2e_*`) y fixture `e76035db4846` en `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/`. | `cargo test -p execute-process` en verde. |
| **CA-4** | Compilación limpia sin warnings (`rust-warning-gate`) y suite de tests en verde. | `cargo test --manifest-path SddIA/Cargo.toml -p execute-process` y `cargo clippy` / `cargo build --release -p execute-process`. |
