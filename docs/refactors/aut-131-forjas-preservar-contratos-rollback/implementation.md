---
feature_name: aut-131-forjas-preservar-contratos-rollback
created: "2026-11-27"
process: refactorization
branch_name: refactor/aut-131-forjas-preservar-contratos-rollback
persist_ref: docs/refactors/aut-131-forjas-preservar-contratos-rollback
tracker_ref: AUT-131
---
# Implementación — AUT-131

## D1 — `patch_action_content_update`

- Campos de contrato solo se sustituyen si vienen en `inputs` (`action_inputs`, `action_outputs`, `action_context`, `actions_contract_version`).
- Sin payload: conserva valores del frontmatter leído del disco.

## D2 — `sync_action_index_row`

- Iteración línea a línea: si la fila coincide con `name`, reescribe solo esa fila de tabla.
- No usa `skip_garbage` ni truncado post-tabla.

## D3 — `run_action_forge` update

- `with_file_rollback` envuelve patch + validación anti-anemia + sync índice.
- Índice solo se muta si el closure termina en `Ok`.
- `UUID_NOT_V4` preexistente → warning; demás códigos → `Err` + rollback.

## D4 — `run_suite_forge` update

- Mismo closure aplica `artifact_yaml_replacements` y luego `markdown_body_replacements` si ambos presentes.
- Error en cuerpo → rollback incluye YAML no persistido.

## D5 — `run_agent_forge` update

- Tras YAML replacements sin body: `refresh_agent_hash_line_delimited`.
- Índice de agentes con snapshot restore si falla el closure.

## Tests

| KA | Test |
| --- | --- |
| KA-FORGE-1 | `action_update_preserves_frontmatter_not_in_payload`, `action_update_preserves_contract_and_index_tail` |
| KA-FORGE-2 | `sync_action_index_row_last_row_keeps_trailing_sections` |
| KA-FORGE-3 | `action_update_anemic_body_rolls_back_file_and_index` |
| KA-FORGE-4 | `suite_update_applies_yaml_and_body_together`, `suite_update_failing_body_rolls_back_yaml` |
| KA-FORGE-5 | `agent_update_yaml_only_refreshes_hash`, `agent_update_failure_restores_agent` |