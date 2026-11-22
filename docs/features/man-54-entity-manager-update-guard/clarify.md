---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
tracker_ref: MAN-54
---

# Clarificación — man-54-entity-manager-update-guard

## Origen y fricción (`KZ-EST-05`)

- Referencias: `MAN-43` (O10, O11, O14b), `MAN-50` (`docs/features/man-50-cierre-man-43/execution.md:58-59`), `docs/audits/informe_de_diagnostico_desacople_eventos_sddia.md` (§3.3 y §5.1).
- Causas raíz identificadas tras inspección de código:
  1. `forges/factory.rs` y `forges/capability_contract.rs` leen `lifecycle_operation` (default `"create"`), ignorando `"operation"`. En `entity_manager.rs`, `validate_process_inputs` también exige `lifecycle_operation` antes de normalizar `"operation"`.
  2. `run_process_forge`, `run_event_forge` y `run_suite_forge` pueden ejecutar refresh silencioso o caer a flujo de creación en `update` sin campo de mutación reconocido.
  3. `entity_manager.rs::creator_inputs_from_entity` (`"process"`) inyecta `"process_context": seed_field(seed, "process_context", json!("ecosystem-evolution"))`, sobrescribiendo el `context` existente en frontmatter durante `patch_process_phases_update` cuando el payload omite `process_context`.
  4. `sddia-qa` (`run_recalc` en `SddIA/tools/sddia-qa/src/main.rs`) escanea solo `SddIA/process/`, omitiendo los procesos en `SddIA/library/codexes/*/process/`.
  5. Solo `agents-contract` y `events-contract` (este último además exigiendo `event_family`) tenían ruta `update` en `factory.rs`, sin cubrir el resto de meta-contratos `SddIA/*/*-contract.md` ni devolver `handoff_hash_signature_old`/`new` verificables.

## Decisiones estabilizadas (D-E1 … D-E6)

- **D-E1:** Lista cerrada de campos de mutación reconocidos para `AC-1`: `markdown_body_replacements`, `process_phases`, `process_outputs` (incl. alias `outputs` ya soportado en proceso), `hash_refresh_only` (`true`), `schema_json_patch`, `implementation_source_git_apply` (manteniendo los campos específicos ya existentes como `cognitive_role_descriptor`, `artifact_*_replacements`, `daemon_*`, `adopt`, `action_body`/`orchestration_logic`).
- **D-E2:** `"operation"` es alias de `"lifecycle_operation"`; si ambos están presentes y discrepan, prevalece `"lifecycle_operation"`.
- **D-E3:** `hash_refresh_only: true` cuenta como mutación reconocida (no dispara fail-loud de update vacío).
- **D-E4:** Preservar `context` en `patch_process_phases_update` aplica cuando `process_context` está ausente en `semantic_seed` / `inputs`; si viene explícito, se aplica.
- **D-E5:** Meta-contratos en alcance = `SddIA/*/*-contract.md` (un nivel bajo `SddIA/`).
- **D-E6:** El canon de `hash_signature` de procesos no cambia; `recalc-process-hash-signatures` solo amplía su alcance a `process_search_roots(repo)`.
