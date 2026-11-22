---
feature_name: man-54-entity-manager-update-guard
created: "2026-11-22"
process: feature
branch_name: feat/man-54-entity-manager-update-guard
persist_ref: docs/features/man-54-entity-manager-update-guard
execution_id: "c43ad921-2b83-4e95-990e-fba355f88415"
tracker_ref: MAN-54
---

# Objetivos — man-54-entity-manager-update-guard

## Misión

Blindar `entity-manager` y las forjas de `execute-process` frente a mutaciones silenciosas o pérdida de `context` (KZ-EST-05), extender `sddia-qa recalc-process-hash-signatures` a códices vía `process_search_roots(repo)`, y habilitar el `update` gobernado de cuerpo Markdown en meta-contratos `SddIA/*/*-contract.md`.

## Criterios de aceptación

- **AC-1 (`K-ALIAS` + `K-FAIL`):** Todas las forjas relevantes y `entity-manager` aceptan `"operation"` como alias de `"lifecycle_operation"` (prevalece `lifecycle_operation` si ambos vienen y discrepan). Si la operación efectiva es `"update"` y no hay ningún campo de mutación reconocido (`markdown_body_replacements`, `process_phases`, `process_outputs`, `hash_refresh_only: true`, `schema_json_patch`, `implementation_source_git_apply` o parche específico válido de la clase), la forja falla con error descriptivo (cero `idempotent` o éxito silencioso).
- **AC-2 (`K-CTX`):** `entity-manager` y `patch_process_phases_update` preservan el `context` existente en frontmatter cuando `process_context` no viene en el payload de actualización.
- **AC-3 (`K-RECALC`):** `sddia-qa recalc-process-hash-signatures` usa `process_search_roots(repo)` cubriendo `SddIA/process/` y todos los `SddIA/library/codexes/*/process/`.
- **AC-4 (`K-CONTRACT`):** `entity-manager` soporta `update` con `markdown_body_replacements` sobre meta-contratos `SddIA/*/*-contract.md`, devolviendo `handoff_hash_signature_old` y `handoff_hash_signature_new` distintos cuando el contenido cambia.
- **AC-5 (`K-REGRESS`):** Tests unitarios de `execute-process` y `sddia-qa` verdes; órganos vivos compilados/sincronizados y CI GitHub verde.
