---













feature_name: linear-hu-b-01-outbound
document_id: PBI-LINEAR-B-01-OUTBOUND
branch: feat/linear-hu-b-01-outbound
execution_id: "4ab5e4fc-57c8-4d28-b9eb-f403fd99d74a"
---
# Implementación — HU-B 01

- `SddIA/engine/execute-process/src/engine/tracker_outbound.rs` — append JSONL fail-soft.
- Hook en `capsules::invoke_tool_for_process` tras éxito de `linear-tracker-adapter` (`create_issue`, `update_issue_state`, `create_comment`).
- Emisores cubiertos: `tracker-stamp`, `tracker-sync-replay`, `forge-pbi`, `refine-hu` (vía cápsula común).
