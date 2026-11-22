---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
phases:
  - alias-operation-and-fail-loud-update
  - preserve-process-context-on-phases-update
  - sddia-qa-recalc-multi-root
  - meta-contract-body-update
  - release-sync-evolution-and-dcc
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
tracker_ref: MAN-54
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
---

# Plan — man-54-entity-manager-update-guard

| Tarea | Alcance |
|-------|---------|
| **T1** | `resolve_lifecycle_operation` en `forges/common.rs`, normalización de `operation` en `entity_manager.rs`, uso en todas las forjas (`factory.rs` + `capability_contract.rs`) y fail-loud en `update` sin campo de mutación reconocido (`AC-1`). |
| **T2** | Preservación de `context` en `entity_manager.rs::creator_inputs_from_entity` + verificación con `patch_process_phases_update` (`AC-2`). |
| **T3** | `sddia-qa` `run_recalc` multi-root vía `process_search_roots(repo)` + tests unitarios en `sddia-qa` (`AC-3`). |
| **T4** | Update gobernado de meta-contratos `SddIA/*/*-contract.md` con `markdown_body_replacements` y hashes `old`/`new` en `forges/factory.rs` + tests unitarios (`AC-4`). |
| **T5** | Tests de `execute-process` y `sddia-qa`, sincronización de órganos vivos (`./SddIA/scripts/sync-live-organs.sh`), registro de evolución (`sddia-evolution-register`), `validacion.md` APTO, `delivery-close-cycle`, iterar hasta CI verde y fin de ciclo (`AC-5`). |
