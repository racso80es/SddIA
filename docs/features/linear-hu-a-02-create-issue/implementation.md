---
feature_name: linear-hu-a-02-create-issue
created: "2026-10-03"
process: feature
---

# Implementación

- Mutación `issueCreate` tras lookups: `teamId`, `labelIds`, `parentId`, `stateId`, `projectId`, `priority`.
- Lab: fixture `OSC-42` / errores sintéticos en labels y parent.
- Verificación: `cargo test -p linear-tracker-adapter`.
