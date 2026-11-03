---
feature_name: aut-70-fractura-project-id
created: 2026-11-03
process: bug-fix
branch: refactor/man-26-ciclo-vida-software
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/fixes/aut-70-fractura-project-id
pbi_ref: AUT-70
tracker_ref: AUT-70
uuid: a8d4e1f2-3c5b-4d9a-8e7f-1b2c3d4e5f6a
execution_id: impl-aut-70-20261103
source_process: bug-fix
global: APTO
pbi_archived: false
checks:
  AUT70-AC1: APTO
  AUT70-AC2: APTO
  AUT70-AC3: APTO
  AUT70-AC4: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  TECH_AUT70_TESTS: APTO
  TRACKER_LINEAR_NO_KM_FILE: APTO
pr_url: https://github.com/racso80es/SddIA/pull/433
validated_base_sha: 01bb719dabe321c6bea7c6b4ac6963cde6566ff0
validated_head_sha: b36ef3694c3f832e3c0be3748bbaead6d83fda40
validated_tree_sha: 11028b9e02c92352162f4020cf8016b370e80f8a
diff_digest: sha256:3e1286f5f4ffc83b3c0aa02ed6af7b6e86953274198ab96b9956d41de0ef1400
---

# Validación — AUT-70

## Veredicto

**APTO** — GraphQL `ID!`, acuse de error nativo en `route-domain-event` y barrido sin falso `kaizen-finalized` cumplen la spec. `done_gate: linear`: cierre del issue **AUT-70** con PR mergeado; sin PBI en `docs/todos/done/`.

## Evidencia

| ID | Medio | Resultado |
|----|--------|-----------|
| AC1 | `list_issues` + `project_id` con `$projectId: ID!` | Test `aut70` adapter |
| AC2 | `UpdateProject` / `UpdateState` con `ID!` | Revisión + tests |
| AC3 | `try_run_native` `Err` → acuse sin `invoke_execute_action` | Test `aut70` execute-process |
| AC4 | Sweep fractura sin kaizen cerrado | Test `aut70` eda_bus_topology |

Comandos: `cargo test -p linear-tracker-adapter aut70`, `cargo test -p execute-process aut70 --lib` — verde.