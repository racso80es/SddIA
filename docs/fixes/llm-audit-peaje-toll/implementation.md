---
feature_name: llm-audit-peaje-toll
created: "2026-10-30"
process: bug-fix
tracker_ref: AUT-47
---

# Implementación

- `SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs` — `looks_like_issue_identifier`, `resolve_tracker_ref_for_lifecycle`.
- `SddIA/engine/execute-process/src/engine/llm_interaction_audit.rs` — `lifecycle_peaje_audit_blocked`, peaje con resultado JSON, resolución unificada de `tracker_ref`.
- `SddIA/engine/execute-process/src/engine/thermodynamic.rs` — `thermodynamic_toll.llm_process_peaje`.
