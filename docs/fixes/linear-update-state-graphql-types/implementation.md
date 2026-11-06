---

feature_name: linear-update-state-graphql-types
created: "2026-11-06"
process: bug-fix
branch_name: fix/linear-update-state-graphql-types
persist_ref: docs/fixes/linear-update-state-graphql-types
document_id: PBI-cd8405c2
uuid: cd8405c2-2394-4d3e-b846-ab2ffc5f38cc
execution_id: "65f1bd41-ca2d-4139-83b3-edc774251e56"
---
# Implementación — linear-update-state-graphql-types

## Touchpoints

| Ubicación | Cambio |
| --- | --- |
| `SddIA/tools/linear-tracker-adapter/src/main.rs` | `UpdateProject`: `$id: String!`, `$projectId: ID!`. `UpdateState`: `$id: String!`, `$stateId: String`. Test `aut70_update_mutations_declare_id` alineado. |
| `SddIA/tools/linear-tracker-adapter.md` | `version` 1.1.1, `hash_signature` bundle, nota en cuerpo sobre `issueUpdate`. |
| `SddIA/tools/index.md` | Fila `linear-tracker-adapter` → versión 1.1.1. |
| `SddIA/evolution/f8c1a2e3-4d5b-4c6d-a7e8-f9a0b1c2d3e4.md` | Hito. |
| `SddIA/evolution/Evolution_log.md` | Fila + `universe_total` 150. |

## Artefactos compilados (instancia)

Tras `env -u CARGO_TARGET_DIR cargo build -p linear-tracker-adapter --release` y `--target wasm32-wasip1 --release`, el runtime resuelve `wasm` **release** antes que `debug`; sin `release` actualizado el fallo `ID!` persistía aunque el fuente estuviera corregido.

## Fuera de este diff

Cambios locales no commitados bajo `docs/fixes/lifecycle-phase-state-name/` (rama previa).
