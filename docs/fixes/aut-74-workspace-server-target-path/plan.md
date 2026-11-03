---

feature_name: aut-74-workspace-server-target-path
created: "2026-11-03"
process: bug-fix
branch_name: fix/aut-74-workspace-server-target-path
persist_ref: docs/fixes/aut-74-workspace-server-target-path
pbi_ref: AUT-74
execution_id: "dd4f1ae7-e72e-49fb-9943-f84cbee18120"
---
# Plan — aut-74-workspace-server-target-path

- [x] Esquemas MCP y validación previa en `server.rs`
- [x] `URI_PATH_MISSING` para `project://file/` en `server.rs` / `sandbox.rs`
- [x] `TARGET_PATH_EMPTY` en `filesystem-manager`
- [x] `DOC_PATH_EMPTY` en `resolve_doc_path`
- [x] Tests unitarios servidor + cápsula
- [x] Cableado `pbi_ref` opaco (`AUT-74`) en `Work_Initiated` / `Validation_Started` y `tracker-stamp`
