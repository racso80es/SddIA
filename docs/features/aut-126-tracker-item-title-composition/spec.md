---






feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
tracker_ref: AUT-126
execution_id: "8b9cb460-0ea6-4434-bef9-2b193625d13c"
---
# Spec — codec título/cuerpo tracker

## Componentes

| Pieza | Rol |
|-------|-----|
| `tracker_item_document` | `title` en struct; `compose_item_markdown`, `canonicalize_local_markdown`, `seal_with_refined_body`, dialectos |
| `project_binding::tracker_provider_for_inputs` | Resuelve `file` \| `linear` |
| `forge_pbi` / `forge_hu` | Proyección dialecto en forja |
| `refine_item` | `native_title` en snapshot; sellado linear |
| `materialize_fracture_pbi` | Strip H1 en linear al crear fracture PBI |

## Tests

`cargo test -p execute-process --lib tracker_item`