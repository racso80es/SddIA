---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
process: feature
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
items_applied:
  - domain_profile
  - workspace_init_gate
  - detector_i7
  - unit_tests
  - smoke_profile_git_skip
agents: tekton
---

# Execution — sddia-domain-abstraction

## Unit tests (`cargo test -p execute-process`)

| Suite | Resultado |
|-------|-----------|
| `engine::domain_profile::*` (4) | OK |
| `engine::workspace_init::*` (4) | OK |
| `cerbero_di_rbac::ac_r5_gate_pass_cerbero_deny` | OK (AC-DENY) |

## Smoke integración

```bash
CARGO_TARGET_DIR=SddIA/target cargo build -p execute-process   # desde SddIA/
SDDIA_AGENT_RUNTIME_COMMAND= SDDIA_LAB_SKIP_GIT= \
  SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  SddIA/target/debug/execute-process --process feature \
  --inputs "$(cat docs/features/sddia-domain-abstraction/_smoke-execution-profile.json)"
```

| Check | Evidencia |
|-------|-----------|
| `success: true` | envelope |
| `handler: workspace-init` | fase Inicialización **executed** |
| Git skip | `reason: profile_git_not_required`, `profile_source: input_execution_profile` |
| Sin `SDDIA_LAB_SKIP_GIT` | env vacío en smoke |
| `execution_profile.git_required: false` | data |
| `execution_profile.codex_slug` | `codex-software-engineering` (obligatorio post ABSTRACT-02) |
| Rama intacta | permaneció `feat/sddia-domain-abstraction` (no checkout) |

`execution_id` smoke histórico: `8606b3ba-dc3a-4b5c-8ca7-5971a012e7d4`. Re-audit 2026-08-10: smoke con slug software → `profile_git_not_required`; fixture sin slug → `DOMAIN_AUTHORITY_DENIED`.

## Nota operativa

`CARGO_TARGET_DIR` del sandbox puede apuntar a cache antigua: forzar `CARGO_TARGET_DIR=SddIA/target` al compilar el binario de smoke. Binario `SddIA/target/debug/execute-process` fechado 2026-07-24 = **stale** (sin gate de perfil).

Post ABSTRACT-02 (`domain_authority`): `feature` con `git_required: false` **exige** `codex_slug: codex-software-engineering`. `codex_slug: null` ya no es smoke válido.
