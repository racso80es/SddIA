---

feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
tracker_ref: AUT-15
execution_id: "5fdff8ff-4df3-43d7-a1a9-aed1f9d7a85c"
---
# Spec — tracker item document codec

## Componentes

| Pieza | Rol |
|-------|-----|
| `item_lifecycle_frontmatter::normalize_item_markdown` | Lectura: `---` o ` ```yaml ` → markdown canónico `---` |
| `tracker_item_document` | `TrackerItemDocument { properties, body, provider_extras }`; parse / render / seal |
| `item_lifecycle_frontmatter::seal_markdown` | Delega en `seal_item_description` (dialecto fichero) |
| `refine::run_pbi` / `run_hu` | Resuelve raw vía `normalize_item_markdown` antes del sello |
| `tracker_stamp::sync_item_tracker_column_remote` | `seal_markdown` sobre descripción remota (hereda codec) |

## Iteración futura

Operaciones `parse_item_document` / `render_item_document` en `linear-tracker-adapter` y `file-tracker-adapter` bajo contrato `tracker.lifecycle` v1.1.

## Fuera

`tracker-markdown-apply` con cuerpo editable pegado en crudo (warn documentado en auditoría).
