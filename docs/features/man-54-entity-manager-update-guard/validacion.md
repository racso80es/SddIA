---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
tracker_ref: MAN-54
execution_id: c43ad921-2b83-4e95-990e-fba355f88415
global: APTO
checks:
  execute_process_forges_tests: APTO
  execute_process_entity_manager_tests: APTO
  sddia_qa_tests: APTO
  recalc_process_hash_signatures_live: APTO
  sync_live_organs_release: APTO
  verify_compiled_capsules_live: APTO
  qa_gates_suite: APTO
git_changes:
- SddIA/engine/execute-process/src/forges/common.rs
- SddIA/engine/execute-process/src/forges/factory.rs
- SddIA/engine/execute-process/src/forges/capability_contract.rs
- SddIA/engine/execute-process/src/engine/entity_manager.rs
- SddIA/tools/sddia-qa/src/main.rs
- SddIA/evolution/Evolution_log.md
- SddIA/evolution/d1685963-5daf-4c3a-aab4-f766c5e226b8.md
---

# Validación — man-54-entity-manager-update-guard

Veredicto global: **APTO**

## Cobertura de criterios (MAN-54)

| Criterio | Evidencia | Estado |
|----------|-----------|--------|
| **AC-1 (`K-ALIAS` + `K-FAIL`)** | `resolve_lifecycle_operation` + `normalize_lifecycle_inputs` en `common.rs`/`entity_manager.rs` y fail-loud en `update` vacío en `factory.rs`; tests `operation_alias_and_precedence_with_lifecycle_operation`, `normalize_lifecycle_inputs_honors_operation_alias_and_canonical_precedence` y `update_without_recognized_mutation_fails_loud_and_hash_refresh_succeeds` verdes | APTO |
| **AC-2 (`K-CTX`)** | Eliminado default `process_context` en `entity_manager.rs::creator_inputs_from_entity`; test `process_update_preserves_existing_context_when_process_context_omitted` verde | APTO |
| **AC-3 (`K-RECALC`)** | `recalc_process_hash_signatures` usa `process_search_roots(repo)` en `sddia-qa/src/main.rs`; test `recalc_covers_core_and_codex_process_roots` verde y ejecución real `recalc-process-hash-signatures` (`no changes`) | APTO |
| **AC-4 (`K-CONTRACT`)** | `resolve_meta_contract_path` + `update_meta_contract_artifact` para `SddIA/*/*-contract.md`; test `meta_contract_update_via_markdown_body_replacements` verde | APTO |
| **AC-5 (`K-REGRESS`)** | `cargo test -p execute-process --lib forges::` (51 passed), `entity_manager::` (7 passed), `cargo test -p sddia-qa` (26 passed), `./SddIA/scripts/sync-live-organs.sh` (9/9 `--release`, `verify-compiled-capsules` `success: true`) y batería completa `sddia-qa` gates verde | APTO |
