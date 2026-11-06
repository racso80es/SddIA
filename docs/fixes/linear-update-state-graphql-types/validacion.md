---

feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
branch: fix/linear-update-state-graphql-types
global: APTO
pbi_archived: false
checks:
  CA_graphql: APTO
  CA_test_aut70: APTO
  CA_tool_update_state: APTO
git_changes:
  - SddIA/tools/linear-tracker-adapter/src/main.rs
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/tools/index.md
  - SddIA/evolution/f8c1a2e3-4d5b-4c6d-a7e8-f9a0b1c2d3e4.md
  - SddIA/evolution/Evolution_log.md
  - docs/fixes/linear-update-state-graphql-types/
execution_id: "65f1bd41-ca2d-4139-83b3-edc774251e56"
---
# Validación — linear-update-state-graphql-types

## Criterios

| ID | Criterio | Estado |
| --- | --- | --- |
| CA_graphql | Mutaciones `UpdateState` / `UpdateProject` con `$id: String!`; `UpdateState.$stateId: String`; `UpdateProject.$projectId: ID!` | APTO |
| CA_test_aut70 | `cargo test -p linear-tracker-adapter aut70` | APTO |
| CA_tool_update_state | `./sddia-run.sh --tool linear-tracker-adapter` `update_issue_state` MAN-27 → Todo `success: true` | APTO |

## Notas

- La resolución de cápsula prueba perfiles `release` antes que `debug` para WASM y nativo; obligatorio reconstruir **release** de `linear-tracker-adapter` (nativo + wasm32-wasip1) tras el cambio de fuente.

## Cierre Linear

`tracker.provider: linear` — cierre vía issue en **Done** tras merge del PR; sin archivos en `docs/todos/done/`.
