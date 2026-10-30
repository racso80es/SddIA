---
feature_name: forge-pbi-lock-after-create
created: 2026-10-30
process: bug-fix
branch_name: fix/forge-pbi-scan-sin-ledger
persist_ref: docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
tracker_ref: AUT-29
execution_id: b7f670f6-35d3-411b-9e70-9fb68fb4fc61
global: APTO
pbi_archived: false
branch: fix/forge-pbi-scan-sin-ledger
checks:
  CA-LOCK: APTO
  CA-IDEMPOTENT: APTO
  CA-SCAN-REUSE: APTO
  CA-DOMAIN: APTO
  CA-TITLE: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/capsule_paths.rs
- SddIA/engine/execute-process/src/engine/capsules.rs
- SddIA/engine/execute-process/src/engine/handlers/forge_pbi.rs
- SddIA/engine/execute-process/src/engine/tracker_lifecycle_port.rs
- SddIA/tools/linear-tracker-adapter/src/lab_state.rs
- SddIA/tools/linear-tracker-adapter/src/main.rs
- docs/fixes/forge-pbi-lock-after-create/
---

# Validación — seguimiento AUT-29

| Check | Resultado |
|-------|-----------|
| `cargo test -p execute-process forge_pbi --lib -- --test-threads=1` | 14/14 OK |
| CA-LOCK | `lock_after_create_returns_ids_and_retry_same_issue`: éxito, `tracker_ref`, `document_id`, `pbi_forged_pending: true` |
| CA-IDEMPOTENT | Mismo test: segundo `forge-pbi` reutiliza el alta; un solo `create` |
| CA-SCAN-REUSE | `retry_without_ledger_reuses_child_by_forge_key`: `OSC-201`, `PBI-scan01`, `create_count` 0 |
| CA-DOMAIN | `core_project_slug_allowed_with_instance_software_forge_profile` 1/1 OK |
| CA-TITLE | Fuera de alcance (AUT-28); tests `distill_title_*` OK |

Seguimiento del merge `4ef0d08c`. `pbi_archived` sigue en false: el cierre Linear de AUT-29 ya ocurrió en ese merge (`done_gate: linear`).
