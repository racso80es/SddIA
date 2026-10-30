---
feature_name: bridge-presented-tracker-ref
created: "2026-10-30"
process: bug-fix
branch_name: fix/bridge-presented-tracker-ref
persist_ref: docs/fixes/bridge-presented-tracker-ref
pbi_ref: AUT-25
tracker_ref: AUT-25
execution_id: "ceb7ecb1-5e05-4aef-b6f9-c58ed2b6540e"
---

# Implementación

- `SddIA/sddia-daemon-runtime/src/github_bridge.rs`
  - `attach_tracker_from_head` / `objectives_refs_at_head` leen el blob del head.
  - `compose_pre_anchor_event` copia `tracker_ref` y `pbi_ref` si vienen en el PR.
  - `process_pr` enriquece antes de anclar.
  - `build_bus_event` publica `delivery_state: {}`.
