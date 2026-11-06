---
feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
phase: design
agents: dedalo
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
execution_id: "0cc0f2fe-6a6f-45cd-86db-4cf7837a846c"
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
---

# Spec — linear-update-state-graphql-types

## 1. Misión

`update_issue_state` debe pasar validación GraphQL de Linear y aplicar el `WorkflowState`. El sellado de origen de `refine-item` (`seal_refined_todo_at_origin`) podrá mover el issue a **Todo**.

## 2. Problema verificado

| Hecho | Evidencia |
| --- | --- |
| MAN-27 descripción sellada | `fetch_issue`: YAML `tracker_column: todo`, `refinement: refined`, `refinement_execution_id: 390b2ad2-…` |
| MAN-27 estado workflow | `state: Backlog` |
| Mutación física (pre-fix) | `UpdateState($id: ID!, $stateId: ID!)` en `linear-tracker-adapter` |
| Reproducción (pre-fix) | `LINEAR_TRANSPORT` HTTP 400 `GRAPHQL_VALIDATION_FAILED` — posiciones `issueUpdate(id:)` / `stateId` esperan `String` |
| Mutación física (post-fix) | `UpdateState($id: String!, $stateId: String)`; `UpdateProject($id: String!, $projectId: ID!)` |
| Contraste sano | `UpdateDesc($id: String!, …)` y `Archive($id: String!)` ya coinciden con `issueUpdate(id:)` / `issueArchive(id:)` |
| Congelación | Test `aut70_update_mutations_declare_id` exige `ID!` en `UpdateState` y `UpdateProject` |
| Origen AUT-70 | Spec AUT-70 AC2: mutaciones de proyecto y estado declaran `ID!`. Correcto para `$projectId`; incorrecto para `issueUpdate(id:)` |

`tracker-stamp` hace no-op en `PBI_Refined` (sellado en origen). El bus no reintenta la columna. `refine-item` traga `warn:` de `seal_refined_todo_at_origin` y acusa `success`.

## 3. Laudos

| Ref | Decisión |
| --- | --- |
| **L-ID** | `issueUpdate(id:)` y `issueArchive(id:)` usan `$id: String!`. `UpdateState` y `UpdateProject` adoptan el mismo `$id` que `UpdateDesc`. |
| **L-STATE** | `input.stateId` declara `$stateId: String` (esquema actual; no `ID!`). |
| **L-PROJECT** | `$projectId: ID!` se conserva en `list_issues` y en `UpdateProject` (AUT-70 AC1). |
| **L-TEST** | Sustituir `aut70_update_mutations_declare_id`: afirmar tipos reales, no `ID!` en el `id` del issue. |
| **L-KEEP-SOFT** | Fail-soft `warn:` en `refine-item` **fuera de alcance**. No cambia el contrato de acuse. |
| **L-BIN** | Compilar el adaptador nativo en `SddIA/target` (`unset CARGO_TARGET_DIR` / ignición). El CLI no usa el target sandbox. |
| **L-LINEAR** | Sin ficheros nuevos en `docs/todos/`. `pbi_ref` Linear opcional; tracker_ref ausente en init (`lifecycle_forge_started` fail-soft). |

## 4. Contrato GraphQL (post-fix)

| Mutación | Variables |
| --- | --- |
| `UpdateState` | `$id: String!`, `$stateId: String` |
| `UpdateProject` | `$id: String!`, `$projectId: ID!` |
| `UpdateDesc` | `$id: String!` (sin cambio) |
| `list_issues` + proyecto | `$projectId: ID!` (sin cambio) |

Payload de `tool_update_state` / `update_issue_state_request` no cambia: sigue enviando `state_name` + `team_key`; el adaptador resuelve `state_id` y llama `apply_state_id`.

## 5. Criterios de aceptación

1. El texto de `UpdateState` declara `$id: String!` y `$stateId: String`.
2. El texto de `UpdateProject` declara `$id: String!` y `$projectId: ID!`.
3. `list_issues` con `project_id` sigue usando `$projectId: ID!`.
4. `cargo test -p linear-tracker-adapter aut70` verde (test reescrito).
5. `./sddia-run.sh --tool linear-tracker-adapter` `update_issue_state` MAN-27 `state_name: Todo` `success: true` (no `GRAPHQL_VALIDATION_FAILED`).
6. `fetch_issue` MAN-27 → `state: Todo`.
7. Un PR. `validacion.md` APTO. Sin `docs/todos/pending` ni `done`.

## 6. Fuera de alcance

- Promover el `warn:` de `seal_refined_todo_at_origin` a error duro.
- Re-refinar MAN-27 / DoR / Mayeuta.
- Cambiar `update_issue_state_request` (ya envía `state_name`, no `state_key`).
- Alta de WorkflowState en Linear.
- Mutar `.SddIA/project.md`.
