---

feature_name: aut-74-workspace-server-target-path
process: bug-fix
created: "2026-11-03"
pbi_ref: AUT-74
tracker_ref: AUT-74
uuid: 3c9bff2a-fd75-44bb-be23-da7798eb5e5c
persist_ref: docs/fixes/aut-74-workspace-server-target-path
branch_name: fix/aut-74-workspace-server-target-path
execution_id: "dd4f1ae7-e72e-49fb-9943-f84cbee18120"
---
# Implementación — aut-74-workspace-server-target-path

## Touchpoints

| Área | Cambio |
|------|--------|
| `SddIA/tools/sddia-workspace-server/src/server.rs` | Esquemas MCP por tool; validación `TOOL_ARGS_MISSING` antes de `filesystem-manager`; tests de esquema y URI vacía. |
| `SddIA/tools/sddia-workspace-server/src/sandbox.rs` | Variante `InvalidArgument` / código `URI_PATH_MISSING` (reservado); rechazo `project://file/` vacío en `method_resources_read`. |
| `SddIA/skills/filesystem-manager/src/main.rs` | Path vacío → `TARGET_PATH_EMPTY`; tests vacío vs escape y `"."` en raíz. |
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | `resolve_doc_path("")` → `DOC_PATH_EMPTY`. |
| `SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs` | `resolve_tracker_ref_from_pbi_ref` para `pbi_ref` opaco (`AUT-74`). |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Emisión `Work_Initiated` / `Validation_Started` con cableado opaco. |
| `SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs` | `resolve_issue_ref` usa opaco; test lab `pbi_ref` solo. |

## Métrica DoR (AUT-74)

- `fs_list` `{}` → `TOOL_ARGS_MISSING`, sin `PROJECT_SCOPE_ESCAPE`.
- `tools/list` declara `target_path` requerido en `fs_read` / `fs_list`.
- `resources/read` `project://file/` → `URI_PATH_MISSING`.
- Cápsula: `target_path` `""` → `TARGET_PATH_EMPTY`.
- Ignición Linear: `Work_Initiated` con `pbi_ref: AUT-74` sella In Progress.
