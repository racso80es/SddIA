---

feature_name: aut-98-active-executions-dirty-worktree
created: "2026-11-22"
process: feature
phases:
  - id: T1
    name: Eximir active-executions.json en workspace_init.rs y .gitignore
  - id: T2
    name: Añadir test de regresión y fixture fracture-corpus 5f46810af5ec
  - id: T3
    name: Compilar execute-process, ejecutar tests y cerrar validación
execution_id: "bf9e18c4-3ef6-490b-852b-fdfb688d9e04"
---
# Plan de implementación — aut-98-active-executions-dirty-worktree

1. **T1:** Editar `SddIA/engine/execute-process/src/engine/workspace_init.rs` (`path_in_scope`) y `.gitignore` para eximir `.SddIA/state/active-executions.json` y `.SddIA/state/active-executions.json.tmp`.
2. **T2:** Añadir test unitario en `workspace_init.rs` y crear `SddIA/engine/execute-process/tests/fixtures/fracture-corpus/5f46810a-workspace-init-active-executions.json`.
3. **T3:** Ejecutar `cargo test -p execute-process`, compilar `cargo build --release -p execute-process` y registrar `implementation.md`, `execution.md` y `validacion.md`.
