---

feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
tracker_ref: AUT-126
execution_id: "2cddf289-b44b-4855-8339-fee4fb4397d6"
---
# Implementación — AUT-126

## Cambios

- `SddIA/engine/execute-process/src/engine/tracker_item_document.rs` — codec título/cuerpo y tests AUT-126.
- `SddIA/engine/execute-process/src/engine/project_binding.rs` — `tracker_provider_for_inputs`.
- `SddIA/engine/execute-process/src/engine/item_lifecycle_frontmatter.rs` — `seal_markdown_with_dialect`.
- `SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs` / `forge_hu.rs` — `compose_item_markdown`.
- `SddIA/engine/execute-process/src/engine/handlers/refine_item/` — `native_title`, sellado linear.
- `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs` — strip H1 linear.