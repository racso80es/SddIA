---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
---
# Spec — AUT-131

## Oráculos (issue refinado)

| ID | Criterio | Oráculo |
| --- | --- | --- |
| AC-1 | Preservación contrato acción | `action_update_preserves_frontmatter_not_in_payload` + `action_update_preserves_contract_and_index_tail` |
| AC-2 | Cola índice intacta | `sync_action_index_row_last_row_keeps_trailing_sections` |
| AC-3 | Rollback update acción | `action_update_anemic_body_rolls_back_file_and_index` |
| AC-4 | Suite sin silencio | `suite_update_applies_yaml_and_body_together` + `suite_update_failing_body_rolls_back_yaml` |
| AC-5 | Agente hash + restore | `agent_update_yaml_only_refreshes_hash` + `agent_update_failure_restores_agent` |
| AC-6 | Suite verde | `cargo test -p execute-process forges` → 0 failed |
| AC-EM | entity-manager update sin defaults | `action_update_inputs_carry_only_seed_fields` |

## Superficies

- `SddIA/engine/execute-process/src/forges/common.rs` — `patch_action_content_update`, `sync_action_index_row`, `with_file_rollback`
- `SddIA/engine/execute-process/src/forges/factory.rs` — `run_action_forge`, `run_suite_forge`, `run_agent_forge`
- `SddIA/engine/execute-process/src/engine/entity_manager.rs` — seeds solo en create