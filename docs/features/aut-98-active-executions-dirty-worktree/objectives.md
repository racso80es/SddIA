---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Objetivos estabilizados — eliminar auto-bloqueo dirty-worktree por .SddIA/state/active-executions.json
branch_name: feat/aut-98-active-executions-dirty-worktree
persist_ref: docs/features/aut-98-active-executions-dirty-worktree
pbi_ref: AUT-98
document_id: PBI-FIX-FRACTURE-5f46810af5ec
status: requisitos-estabilizados
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Objetivos — aut-98-active-executions-dirty-worktree

## Misión

Eliminar el auto-bloqueo (`dirty-worktree`) que provoca el propio motor `execute-process` al persistir `.SddIA/state/active-executions.json` antes de ejecutar la fase `workspace-init` en procesos de ciclo de vida (`feature`, `bug-fix`, `refactorization`).

Tracker **AUT-98** (documento `PBI-FIX-FRACTURE-5f46810af5ec`, `fracture_hash: 5f46810af5ec`).

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **CA-1** | `path_in_scope` en `workspace_init.rs` devuelve `true` para `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`. | Test unitario en `workspace_init.rs`. |
| **CA-2** | `.gitignore` ignora `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`. | Inspección de `.gitignore` y `git status`. |
| **CA-3** | Test de regresión en `workspace_init.rs` y fixture `5f46810af5ec` en `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/`. | `cargo test -p execute-process` en verde. |
| **CA-4** | Compilación limpia sin warnings (`rust-warning-gate`) y binario release actualizado. | `cargo build --release -p execute-process` y `cargo test -p execute-process`. |
