---
feature_name: aut-129-dcc-pr-opened-tracker-ref-persist
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-129-dcc-pr-opened-tracker-ref-persist
persist_ref: docs/fixes/aut-129-dcc-pr-opened-tracker-ref-persist
tracker_ref: AUT-129
execution_id: "f491f6fc-fa55-488a-8299-742b816a254c"
---

# Implementación

## Cambio motor

Archivo: `SddIA/engine/execute-process/src/engine/lifecycle_phase_action.rs`

En `build_cycle_payload`, bloque adicional:

- Si `tracker_ref` ausente y hay `persist_ref`, llamar `tracker_pbi_meta::resolve_tracker_ref_for_lifecycle(repo, &json!({ "persist_ref": persist }))` e inyectar en el payload.

## Regresión

Test unitario `build_cycle_payload_resolves_tracker_ref_from_persist_objectives_only` (objectives con `tracker_ref: AUT-129`, sin `pbi_ref` en inputs).

## AC2 — workflow Linear en PrOpened

- `apply_remote` (remoto): primero `tool_update_state`, luego `update_issue_description` (evita desfase `tracker_column: in_review` con workflow aún en *In Progress*).
- `tool_update_state`: tras adapter OK, `fetch_issue` y `verify_workflow_state_after_update` (falla explícita si `state` ≠ `state_map` esperado).