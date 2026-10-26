---



feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
execution_id: "ff7464c1-fe32-4067-8fb5-4bf1ccd55cb2"
---
# Implementation

| Path | Cambio |
|------|--------|
| `SddIA/engine/execute-process/src/engine/tracker_item_document.rs` | Nuevo: parse, render, seal |
| `SddIA/engine/execute-process/src/engine/item_lifecycle_frontmatter.rs` | `normalize_item_markdown`; `seal_markdown` delega |
| `SddIA/engine/execute-process/src/engine/handlers/refine.rs` | Resolución remota/local vía normalize |
| `SddIA/engine/execute-process/src/engine/mod.rs` | Registro módulo |
