---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/cognitive-tier-native-dispatch
tracker_ref: AUT-26
execution_id: "73d65b19-25cb-489c-94e5-abffe60b1e46"
items_applied:
  - T1
  - T2
---

# Ejecución — AUT-26

1. **T1** — `subscriber_declares_native`: claves `process`, `action`, `tool` en `route_domain_core.rs`.
2. **T2** — Test `store_backed_native_capsule_does_not_use_cognitive_runtime` en módulo `tier_dispatch`.
3. `cargo test -p execute-process tier_dispatch --lib`.
