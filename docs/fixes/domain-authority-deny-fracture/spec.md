---
feature_name: domain-authority-deny-fracture
created: "2026-10-27"
process: bug-fix
branch_name: fix/domain-authority-deny-fracture
persist_ref: docs/fixes/domain-authority-deny-fracture
pbi_ref: AUT-8
document_id: PBI-50be8903
execution_id: "383795cc-6dee-478f-bcb1-6cbbc670f10c"
tracker_ref: AUT-8
---

# Especificación — fractura en `DOMAIN_AUTHORITY_DENIED` (AUT-8)

## Problema

`run_process` devolvía `success: false` con `error_code: DOMAIN_AUTHORITY_DENIED` sin depositar `System_Fracture_Detected` (colapso mudo).

## Cambio

En `engine/mod.rs`, antes del envelope de deny, invocar `domain_authority::emit_fracture_for_denial`:

| Condición del mensaje | `friction_id` |
|------------------------|---------------|
| Prefijo `PROJECT_GIT_INVALID` | `F-DOMAIN-PROJECT-GIT-INVALID` |
| Resto de denies de autoridad | `F-DOMAIN-AUTHORITY-DENIED` |

Payload ECST: `process_name`, `error_trace`, `agent_emitter`, `attempted_action`, `friction_id`; opcional `project_slug`.

## Criterios de aceptación

| ID | Verificación |
|----|----------------|
| AC-1 | Deny de membresía → evento `System_Fracture_Detected` en store |
| AC-2 | Mensaje con prefijo `PROJECT_GIT_INVALID` → `F-DOMAIN-PROJECT-GIT-INVALID` |
| AC-3 | Otro deny → `F-DOMAIN-AUTHORITY-DENIED` |
| AC-4 | Test `emit_fracture_on_deny_materializes_system_fracture` |

## Fuera de alcance

Relajar `assert_git_isolation`; bypass git/gh; AUT-6 (labs anidados).
