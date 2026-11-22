---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
tracker_ref: MAN-54
---

# Implementación — man-54-entity-manager-update-guard

## Cambios realizados

1. **`SddIA/engine/execute-process/src/forges/common.rs`:**
   - Añadido `resolve_lifecycle_operation(inputs, default)` aceptando `"operation"` como alias de `"lifecycle_operation"` con precedencia del canónico (`D-E2`).
   - Añadido `sha256_text(text)` para sellado de contenido de meta-contratos.
2. **`SddIA/engine/execute-process/src/engine/entity_manager.rs`:**
   - Añadido `normalize_lifecycle_inputs` al inicio de `run` para propagar `"operation"` → `"lifecycle_operation"` antes de `validate_process_inputs`.
   - Eliminada la inyección por defecto `"process_context": "ecosystem-evolution"` en `creator_inputs_from_entity("process", ...)` para preservar el `context` existente en frontmatter durante `patch_process_phases_update` (`AC-2` / `K-CTX`).
3. **`SddIA/engine/execute-process/src/forges/factory.rs` y `capability_contract.rs`:**
   - Todas las forjas (`tool`, `action`, `process`, `agent`, `norm`, `codex`, `suite`, `skill`, `event`, `daemon`, `capability-contract`) usan `resolve_lifecycle_operation(inputs, "create")`.
   - `update` sin campo de mutación reconocido falla explícitamente (`fail-loud`, cero `idempotent` ni refresh silencioso) en `process`, `event`, `suite` y el resto de forjas (`AC-1` / `K-FAIL`), manteniendo `hash_refresh_only: true` como mutación explícita válida (`D-E3`).
   - Añadidos `resolve_meta_contract_path` y `update_meta_contract_artifact` para soportar `update` con `markdown_body_replacements` sobre cualquier meta-contrato `SddIA/*/*-contract.md` (incl. `events-contract` sin `event_family`), devolviendo `handoff_hash_signature_old`, `handoff_hash_signature_new` y `skip_domain_seal: true` (`AC-4` / `K-CONTRACT`).
4. **`SddIA/tools/sddia-qa/src/main.rs`:**
   - `recalc_process_hash_signatures` usa `execute_process::core::resolver::process_search_roots(repo)` cubriendo `SddIA/process/` y `SddIA/library/codexes/*/process/` (`AC-3` / `K-RECALC`).
