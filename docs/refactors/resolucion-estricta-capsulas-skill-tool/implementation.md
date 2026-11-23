---
feature_name: resolucion-estricta-capsulas-skill-tool
created: "2026-11-23"
process: refactorization
branch_name: refactor/resolucion-estricta-capsulas-skill-tool
persist_ref: docs/refactors/resolucion-estricta-capsulas-skill-tool
tracker_ref: AUT-118
---
# Implementación — resolucion-estricta-capsulas-skill-tool (`AUT-118`)

- `SddIA/engine/execute-process/src/engine/notify_humanized_pr_merged.rs`, `route_domain_core.rs`, `handlers/email_noise_digest.rs`: migradas las 3 invocaciones de `send-telegram-notification` a `invoke_tool_capsule_json`.
- `SddIA/engine/execute-process/src/engine/capsules.rs`: añadida `validate_capsule_kind_against_ssot` en `resolve_capsule` abortando con `CapsuleKindMismatch`.
- `SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs`: añadido `CerberoDiCode::CapsuleKindMismatch` y `record_capsule_kind_mismatch` (traza + `publish_dead_letter_via_port`).
- `SddIA/tools/sddia-qa/src/gate_capsule_kind.rs`, `main.rs`, `verify_tools_index.rs`: nuevo gate `gate-capsule-kind` integrado también en `verify-tools-index` con tests unitarios.
