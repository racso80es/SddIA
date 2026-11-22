---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
tracker_ref: MAN-54
items_applied:
  - alias-operation-and-fail-loud-update
  - preserve-process-context-on-phases-update
  - sddia-qa-recalc-multi-root
  - meta-contract-body-update
  - release-sync-evolution-and-dcc
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
---

# Ejecución — man-54-entity-manager-update-guard

## Tareas aplicadas

- **T1 (`common.rs` / `entity_manager.rs` / `factory.rs` / `capability_contract.rs`):** `resolve_lifecycle_operation` + `normalize_lifecycle_inputs` para alias `operation` ↔ `lifecycle_operation` (`D-E2`), y fail-loud en `update` sin campo de mutación reconocido (`AC-1`).
- **T2 (`entity_manager.rs`):** Eliminado default de `process_context` cuando no viene en `semantic_seed`, preservando `context` existente en `patch_process_phases_update` (`AC-2`).
- **T3 (`sddia-qa/src/main.rs`):** `recalc_process_hash_signatures` ampliado vía `process_search_roots(repo)` a `SddIA/process/` y `SddIA/library/codexes/*/process/` (`AC-3`).
- **T4 (`factory.rs`):** `resolve_meta_contract_path` + `update_meta_contract_artifact` para `update` gobernado con `markdown_body_replacements` en `SddIA/*/*-contract.md` (`AC-4`).
- **T5 (Verificación y evolución):**
  - `cargo test -p execute-process --lib forges::`: 51 passed, 0 failed, 1 ignored.
  - `cargo test -p execute-process --lib entity_manager::`: 7 passed, 0 failed.
  - `cargo test -p sddia-qa`: 26 passed, 0 failed.
