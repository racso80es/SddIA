---
feature_name: aut-70-fractura-project-id
process: bug-fix
created: "2026-11-03"
pbi_ref: AUT-70
---

# Implementación — AUT-70

| Tarea | Cambio |
| --- | --- |
| T1 | `UpdateProject` / `UpdateState` con `$id`, `$projectId`, `$stateId` como `ID!` (`list_issues` ya tenía `$projectId: ID!`) |
| T2 | `dispatch_subscriber`: `Err` de `try_run_native` → acuse `failed` sin `invoke_execute_action` |
| T3 | `try_sweep_event`: `System_Fracture_Detected` sin éxito de `cumulo.materialize-fracture-pbi` → `fracture-unmaterialized`, no purga |

Binario: recompilar `linear-tracker-adapter` y `execute-process` en `SddIA/target/debug/`.
