---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
process: feature
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
items:
  - domain_profile
  - workspace_init_gate
  - detector_i7
  - unit_tests
agents: tekton
---

# Implementation — sddia-domain-abstraction

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/engine/execute-process/src/engine/domain_profile.rs` | **Nuevo** — `ExecutionProfile`, `resolve_execution_profile`, precedencia input>instancia>default |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Detector I7 (`requires_capability` ∨ delegates ∨ `resolved_provider`); gate Git por perfil; `execution_profile` en salida |
| `SddIA/engine/execute-process/src/engine/mod.rs` | `pub mod domain_profile` |

## No tocados (Filtro C)

- `SddIA/process/feature.md` / bug-fix / refactorization
- `SddIA/library/codexes/*`
- ABSTRACT-02

## Fixture ejemplo

Ver `_smoke-execution-profile.json` en este `persist_ref`.
