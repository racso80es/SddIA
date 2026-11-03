---
feature_name: aut-74-workspace-server-target-path
created: 2026-11-03
process: bug-fix
branch_name: fix/aut-74-workspace-server-target-path
persist_ref: docs/fixes/aut-74-workspace-server-target-path
pbi_ref: AUT-74
tracker_ref: AUT-74
uuid: 3c9bff2a-fd75-44bb-be23-da7798eb5e5c
execution_id: 6c811ed9-8c0d-45d5-8801-4386b54821df
source_process: bug-fix
global: APTO
pbi_archived: false
checks:
  AUT74-MCP-SCHEMA: APTO
  AUT74-TOOL-ARGS: APTO
  AUT74-TARGET-PATH-EMPTY: APTO
  AUT74-URI-PATH: APTO
  AUT74-WORK-INITIATED: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  TECH_TESTS: APTO
  TRACKER_LINEAR_NO_KM_FILE: APTO
validated_base_sha: 55d3561df2db08b88de92e783d9bef33a29358ec
validated_head_sha: e13bd67c493c1bf596a1c2bfa59818f45e542ad2
validated_tree_sha: ff792af72e8b9ba5eb5d12855a875ec8728bf394
diff_digest: sha256:bb5c17282bf9fd64dca19a6134e9fb8586e54f96569148e84a61a94454b3f5f5
---

# Validación — AUT-74

## Veredicto

**APTO** — Esquema MCP, corte `TOOL_ARGS_MISSING` / `URI_PATH_MISSING` / `TARGET_PATH_EMPTY`, cableado `pbi_ref` opaco en `Work_Initiated` y métricas DoR cubiertos por tests. `done_gate: linear`: cierre de **AUT-74** con PR mergeado; sin fichero en `docs/todos/done/`.

## Evidencia

| ID | Medio | Resultado |
|----|--------|-----------|
| MCP | `cargo test -p sddia-workspace-server` | 9 tests OK |
| Cápsula | `cargo test -p filesystem-manager` | 4 tests OK |
| Ignición | `resolve_tracker_ref_from_opaque`, `work_initiated_stamps_with_opaque` | OK |
| Linear | `fetch_issue` AUT-74 | In Progress tras sello |