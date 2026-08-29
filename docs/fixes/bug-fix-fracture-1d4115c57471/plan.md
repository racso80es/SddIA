---
feature_name: bug-fix-fracture-1d4115c57471
created: "2026-08-29"
process: bug-fix
phases:
  - remove-dirty-fracture-emit
  - optional-telemetry
  - verify-unit
  - document-and-stop-for-laudo
branch_name: fix/bug-fix-fracture-1d4115c57471
persist_ref: docs/fixes/bug-fix-fracture-1d4115c57471
---

# Plan — fractura `1d4115c57471`

Corte de esta entrega: **Diseño (spec + plan) + commit**. Sin implementación de código, sin re-lanzar `delivery-close-cycle` (PR #230 ya abierto por DCC sobre fases `simulated`).

## Fase 1 — Quitar emisión de dominio en el guard

Motor `SddIA/engine/execute-process/src/engine/workspace_init.rs` (no genoma DA-2):

1. En el ramo `!dirty.is_empty()`: conservar `return Err(msg)` con el mismo texto `dirty-worktree: …`.
2. **Eliminar** `emit_workspace_init_fracture(...)` de ese ramo.
3. Si `emit_workspace_init_fracture` queda sin callers: borrar la función.

No tocar `dirty_paths_outside_scope`, `path_in_scope`, ni escapes `SDDIA_LAB_ALLOW_DIRTY` / `SDDIA_LAB_SKIP_GIT`.

## Fase 2 — Telemetría (opcional, mismo PR si es trivial)

Si existe helper fractal de telemetría ya usado por el motor (`write_fractal_event` / `eda_fractal.telemetry`):

- Emitir `Workspace_Init_Guard_Abort` con `friction_id: F-DIRTY-WORKTREE` y lista de paths.
- Si no hay helper limpio: **omitir**; el `Err` + log de fase basta. No inventar clase ECST de dominio.

## Fase 3 — Verificación

```text
cd SddIA && cargo test -p execute-process workspace_init
```

Añadir test: repo con fichero dirty fuera de `persist_ref` + `git_required=true` (o mock de porcelain) → `Err` contiene `dirty-worktree` y `.events/pending/` no contiene `event_type: System_Fracture_Detected`.

Argos posterior: `validacion.md` CA-1…CA-6. Archivo PBI a `docs/todos/done/` en la **misma** rama.

## Fase 4 — Cierre de entrega

Solo tras código + `validacion.md` APTO. No tratar el snapshot `a183a8a` / PR #230 como Done. El cierre real reutiliza la rama `fix/bug-fix-fracture-1d4115c57471` (push incremental; no segundo PR documental).

## Orden

```text
spec/plan (este commit)
  → laudo Vértice Biológico
    → Fase 1 (motor)
      → Fase 2 (telemetría si trivial)
        → Fase 3 tests + validacion + PBI a done/
          → Fase 4 delivery-close con agentes ejecutados
```
