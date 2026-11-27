---
feature_name: aut-137-entity-manager-update
created: 2026-11-27
updated: 2026-11-27T19:30:00Z
process: bug-fix
branch: fix/aut-137-entity-manager-update
branch_name: fix/aut-137-entity-manager-update
persist_ref: docs/fixes/aut-137-entity-manager-update
tracker_ref: AUT-137
tracker_provider: linear
source_process: bug-fix
global: APTO
pbi_archived: false
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  global: APTO
validated_base_sha: 82078bcc08c1e442658ae0eaf498fc2c574ba126
validated_head_sha: 20ee5621edab9eac0569c815194d4f27d4f253fd
validated_tree_sha: 8720594ef8adfa8e7b763c9d92d7f109dc6900e4
diff_digest: sha256:5f071b1268555494aeb6b62ccb7523b7a938857e09c5ca063ad1038830e32311
---

# Validación — AUT-137

## Veredicto

`global: APTO` — cuatro frentes del motor + tests unitarios `execute-process` en verde local.

## Evidencia técnica

- `split_md_frontmatter_strips_utf8_bom`
- `action_update_resolves_actions_contract_slug_without_semver`
- `migrate_entity_uuid_syncs_index_and_coverage`
- `suite_update_forwards_markdown_body_replacements`
- `cargo build -p execute-process --release`

## Tracker

Linear `AUT-137`; `done_gate: linear` — issue no pasa a Done en este paso (pre-merge).