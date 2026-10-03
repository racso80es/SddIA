---
feature_name: linear-tracker-adapter
created: "2026-10-02"
process: feature
---

# Especificación — linear-tracker-adapter

## Cápsula Rust `linear-tracker-adapter`

E/S JSON stdin/stdout (`capsule-json-io`). Contexto tool: `tracker-operations`.

Operaciones: `fetch_issue`, `transition_issue`, `add_comment`, `update_issue_description` (GraphQL ciego, errores tipados, mock lab `SDDIA_LAB_MOCK_LINEAR_URL`).

## Artefactos

- `SddIA/tools/linear-tracker-adapter.md`
- `SddIA/tools/linear-tracker-adapter/src/main.rs`
