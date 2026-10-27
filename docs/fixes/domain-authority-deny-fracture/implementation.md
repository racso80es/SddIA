---
feature_name: domain-authority-deny-fracture
created: "2026-10-27"
process: bug-fix
branch_name: fix/domain-authority-deny-fracture
persist_ref: docs/fixes/domain-authority-deny-fracture
pbi_ref: AUT-8
execution_id: "383795cc-6dee-478f-bcb1-6cbbc670f10c"
---

# Implementación

- `SddIA/engine/execute-process/src/engine/domain_authority.rs`: `friction_id_for_denial_message`, `emit_fracture_for_denial` vía `materialize_pending_domain_event`.
- `SddIA/engine/execute-process/src/engine/mod.rs`: emisión en rama `domain-authority` deny.
- Tests en `domain_authority::tests`.
