---
feature_name: bug-fix-fracture-1d4115c57471
created: "2026-08-29"
process: bug-fix
branch_name: fix/bug-fix-fracture-1d4115c57471
persist_ref: docs/fixes/bug-fix-fracture-1d4115c57471
items:
  - workspace_init/dirty_guard_no_domain_fracture
  - workspace_init/tests/run_dirty_outside_scope
---

# Implementation — fractura `1d4115c57471`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | Eliminada `emit_workspace_init_fracture` y su invocación en el ramo `F-DIRTY-WORKTREE`; se conserva `return Err(dirty-worktree:…)` |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` (tests) | `run_dirty_outside_scope_aborts_without_system_fracture` — abort sin `System_Fracture_Detected` en `.events/pending/` |

## Contrato (implementado)

- Dirty fuera de `persist_ref`/`pbi_ref` → `Err` con prefijo `dirty-worktree:` (sin cambio de mensaje).
- **No** se escribe `System_Fracture_Detected` en `eda_bus.pending`.
- `SDDIA_LAB_ALLOW_DIRTY`, `SDDIA_LAB_SKIP_GIT` y `git_required=false` sin cambios.
- Telemetría `Workspace_Init_Guard_Abort`: omitida (Fase 2 opcional del plan).

## Fuera de alcance (respetado)

- Sin mutación de genoma (`directories.events`).
- Sin emisión nueva de fractura en fallos fetch/checkout.
