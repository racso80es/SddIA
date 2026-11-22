---



feature_name: process-stop-after-and-execution-registry
created: "2026-11-21"
updated: "2026-11-21"
process: feature
purpose: Registro de implementación — stop_after por phase_id + write-ahead + resume (MAN-34)
branch_name: feat/process-stop-after-and-execution-registry
persist_ref: docs/features/process-stop-after-and-execution-registry
pbi_ref: MAN-34
document_id: PBI-PROCESS-STOP-AFTER-AND-EXECUTION-REGISTRY
pbi_uuid: "7f4e9a12-3c8b-4d5e-9a21-6b8c4e109f32"
agent: tekton
phase: ejecucion
status: complete
norm_ref: features-documentation-pattern@1.3.2
execution_id: "1e7ca2f5-f505-46a6-a0be-c7a82d0b58a6"
---
# Implementación — process-stop-after-and-execution-registry

## Touchpoints de código y norma

| Archivo | Cambio | Propósito |
|---------|--------|-----------|
| `SddIA/engine/execute-process/src/engine/execution_registry.rs` | Nuevo módulo | `normalize_stop_after` (string legacy → lista, array, null → `[]`), `is_valid_phase_id`, `persist_phase_reports_atomic` (temp + rename), `upsert_active_execution` / `.SddIA/state/active-executions.json`, `resolve_execution_id_from_index`, `stop_after_from_inputs`. |
| `SddIA/engine/execute-process/src/engine/mod.rs` | `pub mod execution_registry;` | Registro del módulo en `engine`. |
| `SddIA/engine/execute-process/src/engine/executor.rs` | Parada por `phase_id`, write-ahead y `resume` | Elimina `SDDIA_TQM_STOP_AFTER`, `stop_after_design()` e `is_dedalo_design_phase()`. Evalúa `phase.id ∈ stop_after` tras `status: executed`, marca restantes `skipped` (`reason: stop_after`), persiste `phase_reports.json` en init/pre/post/barrera y actualiza `active-executions.json`. Implementa `resume` (skip `executed` + artifact check `spec.md`/`plan.md`, relanza `in_progress`/`failed`/`blocked`, barrera consumida no rebloquea) y emite `Work_Initiated` solo en la primera entrada efectiva a `agent:tekton`. |
| `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs` | Erradicación de env `SDDIA_TQM_STOP_AFTER` | Lee `stop_after` desde inputs vía `stop_after_from_inputs`; elimina inyección `SDDIA_TQM_STOP_AFTER` en `child_env_for_kalma2`. |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Ajuste `Work_Initiated` | Elimina emisión de `Work_Initiated` en Fase 1 (`workspace_init`); la emisión pasa al entry de `agent:tekton` en `executor.rs` (D-WI / KA-STOP-7). |
| `SddIA/engine/execute-process/src/engine/verify_process_integrity.rs` | Validación `phases[].id` | Exige `id` no vacío, `snake_case` y único por proceso en todo el catálogo multi-root. |
| `SddIA/process/process-contract.md` | Bump a `1.5.0` | Exige `phases[].id` (`snake_case`) y documenta inputs base universales `stop_after`, `resume` y `execution_id`. |
| `SddIA/process/*.md` y `SddIA/library/codexes/*/process/*.md` (65 procesos) | `phases[].id` + `hash_signature` | Añade `id` canónico `snake_case` a todas las fases del catálogo (incluidos IDs delivery de §2.1) y refresca `hash_signature`. |
| `SddIA/library/codexes/codex-software-engineering/process/feature.md` | Nota ignición `Work_Initiated` | Actualiza regla AUT-65/MAN-34: `Work_Initiated` se emite solo al entrar en `agent:tekton` (`execution`). |
