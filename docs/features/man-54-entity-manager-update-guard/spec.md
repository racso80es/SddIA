---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
tracker_ref: MAN-54
---

# Especificación Técnica — man-54-entity-manager-update-guard

## 1. Resolución de alias `operation` ↔ `lifecycle_operation` (`AC-1` / `K-ALIAS`)

- Añadir helper `resolve_lifecycle_operation(inputs: &Value, default: &str) -> String` en `forges/common.rs`:
  - Lee `lifecycle_operation` (string no vacío tras `trim`); si falta o es vacío, lee `operation` (string no vacío tras `trim`); si falta, devuelve `default`.
  - Si ambos están presentes y discrepan, prevalece `lifecycle_operation` (`D-E2`).
- En `engine/entity_manager.rs::run`:
  - Antes de comprobar `seal-anchor` y antes de `validate_process_inputs`, normalizar `process_inputs` clonado: si `lifecycle_operation` falta/es vacío y `operation` es string no vacío, inyectar `lifecycle_operation = operation`.
- En `forges/factory.rs` (las 10 forjas de clase) y `forges/capability_contract.rs`:
  - Reemplazar `str_field(inputs, "lifecycle_operation", "create")` por `resolve_lifecycle_operation(inputs, "create")`.

## 2. Fail-loud en `update` sin campo de mutación (`AC-1` / `K-FAIL`)

- En `run_process_forge`:
  - Si `lifecycle == "update"`:
    - Si `locate_existing_process_path(repo, &name)` es `None`, retornar `Err(format!("process update: no existe {name}"))`.
    - Comprobar `hash_refresh_only == true` → refrescar hash y retornar `Ok`.
    - Comprobar `markdown_body_replacements`, `process_phases` (array no vacío), `process_outputs`/`outputs` (array no vacío), `implementation_source_git_apply`.
    - Si ninguno está presente, retornar `Err(...)` descriptivo en lugar de refrescar silenciosamente el hash.
- En `run_event_forge`:
  - Si `lifecycle == "update"` y `name != "events-contract"`: si `!event_path.is_file()` retornar `Err`; si no hay `hash_refresh_only: true`, `markdown_body_replacements` ni `implementation_source_git_apply`, retornar `Err` descriptivo en lugar de caer a creación.
- En `run_suite_forge`:
  - Si `lifecycle == "update"`: si `!suite_path.is_file()` retornar `Err`; soportar `hash_refresh_only: true` y `markdown_body_replacements`; si no hay ninguno, retornar `Err` descriptivo en lugar de recrear.
- En `run_tool_forge`, `run_action_forge`, `run_agent_forge`, `run_norm_forge`, `run_codex_forge`, `run_skill_forge`, `run_daemon_forge`:
  - Si `lifecycle == "update"` y el artefacto no existe (y no es meta-contrato), retornar `Err` en lugar de caer a `create`.
  - Soportar `hash_refresh_only: true` donde faltaba (`agent`, `codex`).

## 3. Preservación de `context` en `process` (`AC-2` / `K-CTX`)

- En `engine/entity_manager.rs::creator_inputs_from_entity` (`"process"`):
  - No insertar `"process_context"` con default `"ecosystem-evolution"` cuando `seed.get("process_context")` está ausente. Solo propagar `"process_context"` si viene en `seed`.
  - En `run_process_forge` (rama `create`), mantener el default `"ecosystem-evolution"` cuando `process_context` está ausente.
  - En `patch_process_phases_update`, si `process_context` es `None`, el `context` existente en frontmatter permanece intacto (`D-E4`).

## 4. Recalc multi-root en `sddia-qa` (`AC-3` / `K-RECALC`)

- En `SddIA/tools/sddia-qa/src/main.rs::run_recalc`:
  - Sustituir `repo.join("SddIA/process")` por `execute_process::core::resolver::process_search_roots(repo)`.
  - Extraer función pura `recalc_process_hash_signatures(repo, write, stems)` testeable con `TempDir` cubriendo `SddIA/process/` y `SddIA/library/codexes/*/process/`.

## 5. Update gobernado de meta-contratos `SddIA/*/*-contract.md` (`AC-4` / `K-CONTRACT`)

- Añadir helper `resolve_meta_contract_path(repo, entity_class, name) -> Option<PathBuf>` y `run_meta_contract_update(repo, path, name, inputs) -> Result<Value, String>` en `forges/factory.rs`:
  - Resuelve `SddIA/{class_dir}/{name}.md` o busca en `SddIA/*/{name}.md` (un nivel bajo `SddIA/`).
  - Requiere `markdown_body_replacements` (o versión explícita) cuando `lifecycle == "update"` y `name.ends_with("-contract")` (fail-loud si vacío).
  - Calcula `old_hash = sha256:{hex(before)}` y `new_hash = sha256:{hex(after)}`, aplica `apply_markdown_body_replacements` al cuerpo preservando el frontmatter (actualizando `contract_version` si se pasa en `inputs`), escribe el archivo y retorna `handoff_entity_uuid`, `handoff_version`, `handoff_hash_signature_old`, `handoff_hash_signature_new`, `skip_domain_seal: true` y `artifact_contract_md`.
  - Invocado antes de `resolve_effective_event_family` en `run_event_forge` (de modo que `events-contract` no exija `event_family`) y en el resto de forjas / `materialize_by_inputs` cuando `lifecycle == "update"` y `name.ends_with("-contract")`.
