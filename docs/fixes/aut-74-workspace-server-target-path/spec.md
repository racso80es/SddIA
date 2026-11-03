---
feature_name: aut-74-workspace-server-target-path
created: "2026-11-03"
process: bug-fix
branch_name: fix/aut-74-workspace-server-target-path
persist_ref: docs/fixes/aut-74-workspace-server-target-path
pbi_ref: AUT-74
execution_id: "6c811ed9-8c0d-45d5-8801-4386b54821df"
---

# Spec — aut-74-workspace-server-target-path

## Servidor MCP (`sddia-workspace-server`)

- `tools/list`: esquema por tool con `target_path` / `content` / `destination_path` según AUT-74; `git_status` vacío; `run_check` exige `executable`.
- `tools/call` fs_*: validar argumentos antes de `filesystem-manager`; respuesta `TOOL_ARGS_MISSING` sin cápsula.
- `resources/read` `project://file/`: `URI_PATH_MISSING`, no `PROJECT_SCOPE_ESCAPE`.

## Cápsula (`filesystem-manager`)

- `target_path` vacío → `TARGET_PATH_EMPTY`; escape real sigue en `PROJECT_SCOPE_ESCAPE`.

## Motor (opcional)

- `resolve_doc_path("")` → `DOC_PATH_EMPTY`, no prefijo escape.

## Tests

- Servidor: esquema, `fs_list` `{}`, URI vacía.
- Cápsula: vacío vs `..`; `"."` resuelve raíz.
