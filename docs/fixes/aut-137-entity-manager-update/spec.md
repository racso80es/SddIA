---
feature_name: aut-137-entity-manager-update
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-137-entity-manager-update
persist_ref: docs/fixes/aut-137-entity-manager-update
tracker_ref: AUT-137
execution_id: "727f8de1-4875-4cb3-a7e7-f11cf64cc9a0"
---

# Especificación — AUT-137

## Objetivo

Restaurar update de `entity-manager` para los cuatro bloqueantes empíricos (BOM, contract sin SemVer, UUID legacy, suite sin `markdown_body_replacements`).

## Cambios de motor

1. `common::split_md_frontmatter` elimina U+FEFF antes del delimitador `---`.
2. `patch_action_content_update` resuelve versión contractual cuando `contract: actions-contract` carece de sufijo SemVer (lee `actions-contract.md`).
3. Input opcional `entity_uuid_migrate` (UUID v4): migra frontmatter, fila de `actions/index.md` y entrada en `eda-coverage.json` (`transfer_entity_coverage`).
4. Mapper `suite` en `entity_manager.rs` reenvía `markdown_body_replacements` como norm/codex.

## Criterios

AC-1..AC-5 del ítem Linear AUT-137 cubiertos por tests unitarios en `execute-process`.