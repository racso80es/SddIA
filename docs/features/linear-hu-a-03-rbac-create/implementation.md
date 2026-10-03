---
feature_name: linear-hu-a-03-rbac-create
created: "2026-10-03"
process: feature
---

# Implementación

`gate_linear_tracker_operation` valida norma + `context[]` del proceso antes de `create_issue`. Operaciones existentes sin cambio de política.

Verificación: `cargo test -p execute-process --lib ac_rbac`.
