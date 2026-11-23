---
feature_name: resolucion-estricta-capsulas-skill-tool
created: "2026-11-23"
process: refactorization
branch_name: refactor/resolucion-estricta-capsulas-skill-tool
persist_ref: docs/refactors/resolucion-estricta-capsulas-skill-tool
tracker_ref: AUT-118
---
# Plan — resolucion-estricta-capsulas-skill-tool (`AUT-118`)

1. **Ola 0:** Añadir `aut118_resolve_capsule_rejects_kind_mismatch` en `capsules.rs` y confirmar rojo.
2. **Call sites:** Sustituir `invoke_capsule_json` por `invoke_tool_capsule_json` para `send-telegram-notification` en `notify_humanized_pr_merged.rs`, `route_domain_core.rs` y `handlers/email_noise_digest.rs`.
3. **Runtime + Cerbero RBAC:** Añadir `validate_capsule_kind_against_ssot` en `capsules.rs` y `CerberoDiCode::CapsuleKindMismatch` + `record_capsule_kind_mismatch` en `cerbero_di_rbac.rs`.
4. **Gate `sddia-qa`:** Crear `gate_capsule_kind.rs`, registrarlo en `main.rs` e integrarlo en `verify_tools_index.rs`.
5. **Evolution + Cierre:** Registrar en `SddIA/evolution/` y cerrar ciclo `delivery-close-cycle` → `accept-pr`.
