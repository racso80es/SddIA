---





feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T6
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
execution_id: "65f1bd41-ca2d-4139-83b3-edc774251e56"
---
# Plan — linear-update-state-graphql-types

Parada actual (**L0**): `objectives.md`, `spec.md`, `plan.md`. Rama: `fix/linear-update-state-graphql-types`. Base: `main`.

Código en estímulo de Ejecución. Genoma `SddIA/tools/linear-tracker-adapter/` solo entonces.

## T0 — Sellar planificación

| Paso | Acción |
| --- | --- |
| T0.1 | Frontmatter coherente (`document_id`, `uuid`, `execution_id`). |
| T0.2 | `spec.md` + `plan.md` bajo `persist_ref`. Prohibido `implementation.md` en este sello. |
| T0.3 | Commit documental: `docs: plan GraphQL String en UpdateState` |

## T1 — Mutaciones

1. `apply_state_id` / `UpdateState`: `$id: String!`, `$stateId: String`.
2. `UpdateProject`: `$id: String!`; conservar `$projectId: ID!`.
3. No tocar `UpdateDesc`, `Archive`, `commentCreate`, ni `list_issues`.

## T2 — Test AUT-70

1. Reescribir `aut70_update_mutations_declare_id`:
   - `UpdateState($id: String!, $stateId: String)`
   - `UpdateProject($id: String!, $projectId: ID!)`
2. Conservar aserción existente `$projectId: ID!` en `list_issues_query_includes_label_filter`.
3. `cd SddIA && env -u CARGO_TARGET_DIR cargo test -p linear-tracker-adapter aut70`

## T3 — Binario nativo

1. `env -u CARGO_TARGET_DIR cargo build -p linear-tracker-adapter` (target del repo, no sandbox).
2. Si ignición sella cápsulas, repetir vía `start-sddia.sh` / `--seal-capsules` según contrato de distribución. No sustituir el ELF que usa `./sddia-run.sh --tool` por un target ajeno.

## T4 — Verificación física

1. `update_issue_state` MAN-27 → `Todo` (mismos campos que el sellado: `team_key` del issue, `project_id` del manifiesto core).
2. CA: acuse `success: true`; ausencia de `GRAPHQL_VALIDATION_FAILED` / `ID!` vs `String`.
3. `fetch_issue` MAN-27: `state: Todo`.
4. No re-ejecutar `refine-item` (ya refined; el defecto era la mutación, no Mayeuta).

## T5 — Evolution

1. Alta canónica en `SddIA/evolution/` + fila en `Evolution_log.md`. Ligar `document_id` PBI-cd8405c2 y sighting MAN-27.

## T6 — Cierre documental

1. `implementation.md` + `execution.md` (un commit por tarea T1–T5).
2. `validacion.md` APTO. Un PR vía `delivery-close-cycle`. Sin ficheros en `docs/todos/`.

## Orden y riesgos

| Riesgo | Mitigación |
| --- | --- |
| `CARGO_TARGET_DIR` sandbox deja ELF viejo | T3: `env -u CARGO_TARGET_DIR`; verificar mtime bajo `SddIA/target/` |
| AUT-70 AC2 histórico (`ID!` en estado) | T2 reescribe el test; evolution T5 documenta el recorte: `ID!` solo `projectId` |
| `UpdateProject` no reproducido en MAN-27 | Mismo `issueUpdate(id:)` que UpdateState; T1 lo alinea preventivo |
| Fail-soft oculta el 400 | Fuera de alcance (L-KEEP-SOFT); T4 verifica la mutación directa |
