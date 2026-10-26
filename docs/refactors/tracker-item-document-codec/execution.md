---
feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
items_applied:
  - "Codec tracker_item_document + normalize Linear fence"
  - "refine-pbi AUT-15 sellado en laboratorio"
---

# Execution

- Rama `refactor/tracker-item-document-codec`.
- `refine-pbi` sobre `AUT-15` acusó `refinado` tras normalización de fence.
- Tests: `seal_markdown_accepts_linear_yaml_fence`, `tracker_item_document::parse_linear_fence_and_seal_refined`.
